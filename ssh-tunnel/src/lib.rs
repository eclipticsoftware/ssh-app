use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

pub mod config;
pub mod logger;
pub mod status;
pub mod tunnel;

use crate::{
    config::SshConfig,
    status::{ExitCondition, Result, SshStatus},
    tunnel::{ChildProc, SshTunnel},
};

/// A handle for a watcher thread
///
/// This thread will run until the child process ends. When that happens, it will capture the SshStatus from the child
/// process's stderr, as well as the process's exit code, and will return a tuple containing the [SshStatus] and the
/// [ExitCondition] parsed from the exit code.
pub type SshHandle = thread::JoinHandle<(SshStatus, ExitCondition)>;

/// Starts a tunnel process and a watcher thread
///
/// This is the only public entry function to the library. It will spawn a new ssh child process, as well as a thread to
/// watch that process. It will always return a handle to the [tunnel](SshTunnel) object, as well as a handle to the watcher
/// thread.
///
/// The given status_callback provides a custom interface for updating the system SshStatus when it changes. This status will
/// only change when the tunnel process first starts (at which point it will either go into the) [SshStatus::Connected], or
/// one of the **Error** states (see [SshStatus] documentation).
///
/// The function can either wait for the ssh tunnel to complete (or fail), by setting the wait parameter to true, or it can
/// spawn the process asynchronously and allow the system status to be updated through the status_callback.
///
/// # Errors
///
/// If the tunnel process fails to spawn or if it fails to acquire a lock on the tunnel's mutex, it will return an
/// [SshStatus::AppError].
///
/// # Examples
///
/// Start tunnel and wait for it to connect:
/// ```no_run
/// # use std::sync::{Arc, Mutex};
/// # use ssh_tunnel::{
/// #    config::SshConfig,
/// #    status::{Result, SshStatus},
/// #    tunnel::{ChildProc, SshHandle, SshTunnel, TunnelChild},
/// # };
/// # fn spawn_proc() -> Result<()> {
/// # let config = SshConfig::new(
/// #     "endhost",
/// #     "username",
/// #     "keypath",
/// #     "tohost",
/// #     1,
/// #     2,
/// #     10,
/// #     &["-T"],
/// # );
/// let exit_callback = Arc::new(Mutex::new(|status| {
///     println!("Status: {:?}", status);
///     match status {
///         SshStatus::Dropped => println!("Dropped connection"),
///         SshStatus::Unreachable => println!("Unreachable"),
///         SshStatus::Ready => println!("Disconnected cleanly"),
///         _ => println!("Unsupported status: {status}"),
///     }
/// }));
///
/// let tunnel: SshTunnel<TunnelChild>;
/// let handle: SshHandle;
///
/// (tunnel, handle) = ssh_tunnel::start_and_watch_ssh_tunnel(config, exit_callback, true)?;
/// # Ok(())
/// # }
/// ```
///
/// Start tunnel and update status asynchronously:
/// ```no_run
/// # use std::sync::{Arc, Mutex};
/// # use ssh_tunnel::{
/// #    config::SshConfig,
/// #    status::{Result, SshStatus},
/// #    tunnel::{ChildProc, SshHandle, SshTunnel, TunnelChild},
/// # };
/// # fn spawn_reconnect(config: Arc<Mutex<SshConfig>>) { }
/// # fn emit_status(status: SshStatus) {}
/// # fn spawn_proc() -> Result<()> {
/// # let config = SshConfig::new(
/// #     "endhost",
/// #     "username",
/// #     "keypath",
/// #     "tohost",
/// #     1,
/// #     2,
/// #     10,
/// #     &["-T"],
/// # );
/// let config_thread = Arc::new(Mutex::new(config.clone()));
/// let callback = Arc::new(Mutex::new(move |status| {
///     let status = match status {
///         SshStatus::Dropped => {
///             // Attempt to reconnect in separate thread
///             println!("Dropped tunnel connection");
///             spawn_reconnect(config_thread.clone());
///             SshStatus::Reconnecting
///         }
///         _ => status,
///     };
///     emit_status(status)
/// }));
///
/// let tunnel: SshTunnel<TunnelChild>;
/// let handle: SshHandle;
///
/// (tunnel, handle) = ssh_tunnel::start_and_watch_ssh_tunnel(config, callback, false)?;
/// # Ok(())
/// # }
/// ```
pub fn start_and_watch_ssh_tunnel<T, F>(
    config: SshConfig,
    status_callback: Arc<Mutex<F>>,
    wait: bool,
) -> Result<(SshTunnel<T>, SshHandle)>
where
    T: ChildProc + Send + 'static,
    F: FnMut(SshStatus) + Send + 'static,
{
    let tunnel = if wait {
        start_wait_ssh_tunnel(config)
    } else {
        start_ssh_tunnel(config, status_callback.clone())
    }?;
    let watched_tunnel = tunnel.clone();
    log::debug!("Spawning watcher thread");
    let handle = std::thread::spawn(move || ssh_watch_loop(watched_tunnel, status_callback));

    Ok((tunnel, handle))
}

/// Starts a tunnel process and waits for the tunnel to connect (or fail), and returns a handle to the process
///
/// # Errors
///
/// If the tunnel process fails to spawn or if it fails to acquire a lock on the tunnel's mutex, it will return an
/// [SshStatus::AppError].
fn start_wait_ssh_tunnel<T>(config: SshConfig) -> Result<SshTunnel<T>>
where
    T: ChildProc + Send + 'static,
{
    let tunnel = T::new(config)?;
    match wait_for_start(tunnel.clone()) {
        Ok(_) => Ok(tunnel),
        Err(_) => Err(tunnel
            .lock()
            .map_err(|err| SshStatus::AppError(format!("Failed to lock tunnel: {err}")))?
            .exit_status()),
    }
}

/// Starts a tunnel process and returns a handle to the process immediately.
///
/// The status of the ssh connection will be returned through the status_callback once the tunnel has connected (or failed to
/// connect).
///
/// # Errors
///
/// If the tunnel process fails to spawn or if it fails to acquire a lock on the tunnel's mutex, it will return an
/// [SshStatus::AppError].
fn start_ssh_tunnel<T, F>(config: SshConfig, status_callback: Arc<Mutex<F>>) -> Result<SshTunnel<T>>
where
    T: ChildProc + Send + 'static,
    F: FnMut(SshStatus) + Send + 'static,
{
    let tunnel = T::new(config)?;
    let tunnel_sts = tunnel.clone();

    log::debug!("Spawning start watcher");
    thread::spawn(move || {
        if let Err(status) = wait_for_start(tunnel_sts) {
            log::debug!("Start status: {status}");
            match status {
                SshStatus::Ready => {}
                _ => call_status_callback(status_callback, status),
            }
        } else {
            call_status_callback(status_callback, SshStatus::Connected)
        }
    });

    Ok(tunnel)
}

/// Waits for the ssh process to start
///
/// This is a utility function to allow the waiting to happen without holding a lock on the tunnel.
///
/// # Errors
///
/// * Returns [SshStatus::Ready] if the ssh process spawned, but there was an error. The details of this error
///   can be read with the [ChildProc::exit_status()] method.
/// * Returns [SshStatus::AppError] if there was an error in the process itself.
fn wait_for_start<T>(tunnel: SshTunnel<T>) -> Result<()>
where
    T: ChildProc + Send + 'static,
{
    let mut stdout = {
        tunnel
            .lock()
            .map_err(|err| SshStatus::AppError(format!("Failed to lock tunnel: {err}")))?
            .stdout()?
    };

    let mut buffer = [0; 15];
    let len = stdout
        .read(&mut buffer)
        .map_err(|err| SshStatus::AppError(format!("Failed to read from stdout: {err}")))?;

    log::debug!("stdout: {}", String::from_utf8_lossy(&buffer));

    if len >= 15 {
        Ok(())
    } else {
        Err(SshStatus::Ready)
    }
}

/// Watches a tunnel process and calls the given callback when it exits.
///
/// This function is meant to run in a thread and will not return until the tunnel process ends. When that happens, it will
/// capture the [exit status](SshStatus) from the child process's stderr and call the exit_callback with that status.
///
/// # Returns
///
/// Returns a tuple containing the SshStatus and the ExitCondition
fn ssh_watch_loop<T, F>(
    tunnel: SshTunnel<T>,
    exit_callback: Arc<Mutex<F>>,
) -> (SshStatus, ExitCondition)
where
    T: ChildProc,
    F: FnMut(SshStatus) + Send,
{
    loop {
        match tunnel.lock() {
            Ok(mut tunnel) => {
                if let Some(exit_cond) = tunnel.exited() {
                    let ssh_status = tunnel.exit_status();
                    call_status_callback(exit_callback, ssh_status.clone());
                    return (ssh_status, exit_cond);
                }
            }
            Err(err) => {
                let ssh_status = SshStatus::AppError(format!("Failed to lock tunnel: {err}"));
                call_status_callback(exit_callback, ssh_status.clone());
                return (ssh_status, ExitCondition::ProcError);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/// Helper function to call the status callback
fn call_status_callback<F>(status_callback: Arc<Mutex<F>>, status: SshStatus)
where
    F: FnMut(SshStatus) + Send,
{
    match status_callback.lock() {
        Ok(mut cb) => cb(status),
        Err(err) => log::error!("Failed to get exit callback handle: {err}"),
    }
}
