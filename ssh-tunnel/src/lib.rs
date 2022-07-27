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
    tunnel::{ChildProc, SshHandle, SshTunnel},
};

/// Starts a tunnel process and waits for the tunnel to connect (or fail)
///
/// # Returns
///
/// Returns a result with the tunnel process if it was successfully spawned, and with a status
/// giving the reason for the failure if not.
pub fn start_wait_ssh_tunnel<T>(config: SshConfig) -> Result<SshTunnel<T>>
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

/// Starts a tunnel process and captures the start status in a callback
///
/// # Returns
///
/// Returns a result with the tunnel process if it was successfully spawned, and with a status
/// giving the reason for the failure if not.
pub fn start_ssh_tunnel<T, F>(
    config: SshConfig,
    status_callback: Arc<Mutex<F>>,
) -> Result<SshTunnel<T>>
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

/// Waits for the ssh process to start without holding a lock on the tunnel
///
/// # Returns
///
/// * Returns `Ok` if the ssh process started without error
/// * Returns `Err(SshStatus::Ready)` if the ssh process spawned, but there was an error. The details of this error
///   can be read with the `ChildProc::exit_status()` method.
/// * Returns `Err(AppError)` if there was an error in the process itself
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

/// Watches a tunnel process and calls the given callback when it exits
///
/// # Returns
///
/// Returns a tuple containing the SshStatus and the ExitCondition
pub fn ssh_watch_loop<T, F>(
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

/// Starts a tunnel process and a watcher thread
///
/// # Returns
///
/// If the spawn was successful, it returns a tuple containing the SshStatus and the ExitCondition.
/// Otherwise it returns a status giving the reason for the failure.
pub fn start_and_watch_ssh_tunnel<T, F>(
    config: SshConfig,
    callback: Arc<Mutex<F>>,
    wait: bool,
) -> Result<(SshTunnel<T>, SshHandle)>
where
    T: ChildProc + Send + 'static,
    F: FnMut(SshStatus) + Send + 'static,
{
    let tunnel = if wait {
        start_wait_ssh_tunnel(config)
    } else {
        start_ssh_tunnel(config, callback.clone())
    }?;
    let watched_tunnel = tunnel.clone();
    log::debug!("Spawning watcher thread");
    let handle = std::thread::spawn(move || ssh_watch_loop(watched_tunnel, callback));

    Ok((tunnel, handle))
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
