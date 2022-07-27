#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use std::result;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, time::Duration};

use serde::Deserialize;
use tauri::api::path;
use tauri::command;
use tauri::{window::Window, RunEvent, State};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use ssh_tunnel::{
    config::SshConfig,
    logger,
    status::{Result, SshStatus},
    tunnel::{ChildProc, SshTunnel, TunnelChild},
    SshHandle,
};

fn main() {
    let mut logpath = path::home_dir().unwrap_or_else(|| PathBuf::from("."));

    logpath.push(".eclo-ssh-client.log");

    let logpath = logpath
        .into_os_string()
        .into_string()
        .expect("Failed to create log path");

    if let Err(err) = logger::configure_logger(&logpath) {
        panic!("Failed to configure logger: {err}");
    }
    log::debug!("Running Eclo SSH Client");

    let context = Context::new();
    let ctx_win_capt = context.clone(); // context for the on_page_load callback

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        // Gets a handle to the window when the page loads
        .on_page_load(move |window, _| {
            log::debug!("Setting window");
            ctx_win_capt
                .lock()
                .map_err(|err| {
                    log::error!("Failed to unlock context!!!");
                    err
                })
                .expect("Failed to unlock context")
                .window = Some(window);
        })
        // Tells the framework that it needs to manage the context
        .manage(context.clone())
        // Sets the handler functions
        .invoke_handler(tauri::generate_handler![start_tunnel, end_tunnel,])
        // Builds the app
        .build(tauri::generate_context!())
        .map_err(|err| {
            log::error!("Failed to build tauri application");
            err
        })
        .expect("Failed to build tauri application");

    // Set the activation policy to regular for mac
    #[cfg(target_os = "macos")]
    app.set_activation_policy(ActivationPolicy::Regular);

    // Run the app with a callback to handle the ExitRequested event. This allows the app to kill the tunnel
    // when the user closes it while the tunnel process is still running.
    app.run(move |_app_handle, event| {
        if let RunEvent::ExitRequested { api: _, .. } = event {
            log::debug!("Exiting app...");
            kill_tunnel(context.clone());
        }
    })
}

/// The maximum number of reconnect tries to make while attempting to reconnect.
const MAX_RETRIES: u32 = 5;

/// Maintains the app context, with all of the parameters needed to track the system state.
struct ContextInner {
    /// Container for the tunnel process
    tunnel: Option<SshTunnel<TunnelChild>>,

    /// Handle for the app window
    window: Option<Window>,

    /// The current status of the ssh tunnel
    status: SshStatus,

    /// Number of connection retries left
    retries: u32,

    /// Whether the tunnel is currently trying to reconnect
    reconnecting: bool,
}

impl ContextInner {
    fn new() -> Self {
        ContextInner {
            tunnel: None,
            window: None,
            status: SshStatus::Ready,
            retries: 0,
            reconnecting: false,
        }
    }
}

/// Wraps a threadsafe context in a tuple struct to enable
struct Context(Arc<Mutex<ContextInner>>);

impl Context {
    fn new() -> Self {
        Context(Arc::new(Mutex::new(ContextInner::new())))
    }

    /// Returns a context wth a clone of the inner context
    fn clone(&self) -> Self {
        Context(self.0.clone())
    }

    /// Locks the inner context and returns its reference
    ///
    /// This will not panic (unless the internal [lock](std::sync::Mutex::lock) panics), but the possible
    /// [PoisonError](std::sync::PoisonError) must be handled. Since there is not much to do if the lock for the context is
    /// poisoned but crash, it's probably easier and preferrable, to use [panic_lock](Context::panic_lock).
    fn lock(&self) -> Result<MutexGuard<ContextInner>> {
        self.0
            .lock()
            .map_err(|err| SshStatus::AppError(format!("Failed to lock context: {err}")))
    }

    /// Locks the inner context and returns its reference
    ///
    /// # Panics
    ///
    /// Panics if the lock fails. If this happens, it means that the lock has been poisoned, which means that we will
    /// no longer have any way to communicate to the front end. In this case, we may as well crash.
    fn panic_lock(&self) -> MutexGuard<ContextInner> {
        match self.lock() {
            Ok(inner) => inner,
            Err(status) => {
                log::error!("Panicked trying to lock: {status}");
                panic!("{status}");
            }
        }
    }

    /// Emits a status signal to the front end
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * We fail to acquire a lock on the context
    /// * We fail to get a handle to the window
    /// * We fail to emit the status signal
    ///
    /// A failure to successfully emit a status means that we cannot alert the user of the problem, so we may as well crash.
    fn emit_status(&self, status: SshStatus) {
        let mut inner = self.panic_lock();
        // Only emit the status if it's different than the old status
        if inner.status == status {
            return;
        }

        inner.status = status.clone();

        log::info!("Updating status: {status}");
        if let Some(window) = inner.window.as_ref() {
            if let Err(err) = window.emit("tunnel_status", Some(status.to_signal())) {
                log::error!("Panicked while emitting status: {err}");
                panic!("Tunnel status emit failed: {err}");
            }
        } else {
            log::error!("Panicked while trying to get the window reference");
            panic!("Failed to get window reference");
        };
    }

    /// Sets the context's tunnel
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn set_tunnel(&self, tunnel: SshTunnel<TunnelChild>) {
        self.panic_lock().tunnel = Some(tunnel);
    }

    /// Retrieves the context's tunnel
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn get_tunnel(&self) -> Option<SshTunnel<TunnelChild>> {
        self.panic_lock().tunnel.as_ref().cloned()
    }

    /// Returns the number of retries left
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn get_retries(&self) -> u32 {
        self.panic_lock().retries
    }

    /// Returns the current number of retires and then decrements it (like the --x operator in c)
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn decr_retries(&self) -> u32 {
        let mut inner = self.panic_lock();
        if inner.retries > 0 {
            inner.retries -= 1;
        }
        inner.retries
    }

    /// Sets the number of retries left
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    // fn set_retries(&self, retries: u32) {
    //     self.panic_lock().retries = retries;
    // }

    /// Returns true if we are in the reconnecting state
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn reconnecting(&self) -> bool {
        self.panic_lock().reconnecting
    }

    /// Puts the context into the reconnecting state
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn start_reconnect(&self) {
        let mut inner = self.panic_lock();
        inner.reconnecting = true;
        inner.retries = MAX_RETRIES;
    }

    /// Takes the context out of the reconnecting state
    ///
    /// # Panics
    ///
    /// Panics if the lock on the context fails. (See panic_lock())
    fn stop_reconnect(&self) {
        self.panic_lock().reconnecting = false;
    }
}

/// User settings passed down from the GUI
#[derive(Deserialize, Debug)]
struct UserSettings<'a> {
    host: &'a str,
    user: &'a str,
    port: &'a str,
    key_path: &'a str,
}

impl UserSettings<'_> {
    /// Converts the user settings to an SshConfig object
    fn to_config(&self) -> result::Result<SshConfig, std::num::ParseIntError> {
        let port = self.port.parse()?;
        let flags = vec!["-t", "-t"];

        Ok(SshConfig::new(
            self.host,
            self.user,
            self.key_path,
            "localhost",
            port,
            5432,
            10,
            &flags,
        ))
    }
}

/// Command hook to start the tunnel process
///
/// This will spawn a new process and then check the results. The `settings` parameter originates in the JS front end, and
/// the `context` parameter is the same that is given to the [mange](tauri::Builder::manage) method in the [main] function,
/// and is passed in by the Tauri app framework.
///
/// # Returns
///
/// Returns a string containing the status signal that the JS front end can use to check the status of the command. This will
/// always either be "CONNECTING" or "ERROR: <err message>".
#[command]
fn start_tunnel(settings: UserSettings<'_>, context: State<'_, Context>) -> String {
    let config = match settings.to_config() {
        Ok(cfg) => cfg,
        Err(_err) => {
            let status = SshStatus::ConfigError("Illegal port value".to_string());
            context.emit_status(status.clone());
            return status.to_signal();
        }
    };

    log::info!("Starting tunnel: {:?}", settings);
    let result = spawn_new_tunnel(config, (*context).clone());
    manage_spawn_result(result, (*context).clone())
}

/// Spawns a new tunnel process
///
/// The process status resolving callback that's passed into the process watching thread will check the status
/// and spawn an attempt_reconnect thread if it receives a `SshStatus::Dropped` status. It will then emit the
/// captured status back to the front end.
///
/// # Errors
///
/// If the tunnel process fails to spawn or if it fails to acquire a lock on the tunnel's mutex, it will return an
/// [SshStatus::AppError].
///
/// # Returns
///
/// For an explanation of the return values, see the documentation on [ssh_tunnel::start_and_watch_ssh_tunnel].
fn spawn_new_tunnel(
    config: SshConfig,
    context: Context,
) -> Result<(SshTunnel<TunnelChild>, SshHandle)> {
    log::debug!("Spawning new tunnel");
    let context_thread = context.clone();
    let config_thread = Arc::new(Mutex::new(config.clone()));
    let callback = Arc::new(Mutex::new(move |status| {
        let status = match status {
            SshStatus::Dropped => {
                // Attempt to reconnect in separate thread
                log::debug!("Dropped tunnel connection");
                context.start_reconnect();
                spawn_reconnect(config_thread.clone(), context_thread.clone());
                SshStatus::Reconnecting
            }
            SshStatus::Unreachable => {
                if context.decr_retries() > 0 {
                    // Continue trying to reconnect
                    log::debug!("Continuing reconnect attempt");
                    spawn_reconnect(config_thread.clone(), context_thread.clone());
                    SshStatus::Reconnecting
                } else if context.reconnecting() {
                    // We failed to reconnect, so we have dropped
                    log::debug!("Aborting reconnect attempt");
                    context.stop_reconnect();
                    SshStatus::Dropped
                } else {
                    SshStatus::Unreachable
                }
            }
            _ => status,
        };

        context_thread.emit_status(status)
    }));
    ssh_tunnel::start_and_watch_ssh_tunnel(config, callback, false)
}

/// Checks the result sent from [start_tunnel] or [attempt_reconnect] and updates the status on the front end.
///
/// If the process completely failed to spawn, then the error status will be forwarded (emitted) to the front end. Otherwise,
/// one of the **Transition** states ([SshStatus::Connecting] or [SshStatus::Reconnecting]) will be emitted, depending on the
/// current state of the context.
///
/// # Returns
///
/// A String signal version of the final status. This is only used by [start_tunnel], which returns this status for some
/// checks by the front end.
fn manage_spawn_result(
    result: Result<(SshTunnel<TunnelChild>, SshHandle)>,
    context: Context,
) -> String {
    let status = match result {
        Ok((tnl, _hndl)) => {
            context.set_tunnel(tnl);
            if context.reconnecting() {
                SshStatus::Reconnecting
            } else {
                SshStatus::Connecting
            }
        }
        Err(status) => {
            log::error!("Error during spawn: {:?}", status);
            status
        }
    };

    context.emit_status(status.clone());
    status.to_signal()
}

/// Spawns a reconnect attempt thread
fn spawn_reconnect(config: Arc<Mutex<SshConfig>>, context: Context) {
    thread::spawn(move || {
        attempt_reconnect(config, context);
    });
}

/// Makes a single attempt to reconnect to the ssh server if the tunnel process is dropped
///
/// The immediate result of the process spawn will be emitted to the JS front end. Once the ssh tunnel is fully connected (or
/// fails), the final status of that connection will be emitted.
fn attempt_reconnect(config: Arc<Mutex<SshConfig>>, context: Context) {
    thread::sleep(Duration::from_secs(3)); // Wait 3 seconds to try to reconnect

    let retries = context.get_retries();
    log::info!("Attempting to reconnect. {retries} retries left");

    let cfg = match config.lock() {
        Ok(cfg) => cfg,
        Err(err) => {
            context.emit_status(SshStatus::AppError(format!("Failed to lock config: {err}")));
            return;
        }
    };

    let result = spawn_new_tunnel(cfg.clone(), context.clone());
    manage_spawn_result(result, context);
}

/// Kills the tunnel process if it's running
///
/// The exit status of the process will be emitted to the JS front end automatically when the child process ends. If the
/// tunnel's mutex fails to lock (unlikely), an [SshStatus::AppError] will be emitted instead.
fn kill_tunnel(context: Context) {
    log::info!("Killing tunnel");
    match context.get_tunnel() {
        Some(tunnel) => match tunnel.lock() {
            Ok(mut child) => {
                child.kill();
            }
            Err(err) => {
                context.emit_status(SshStatus::AppError(format!("Failed to kill tunnel: {err}")))
            }
        },
        None => {} // Just ignore it if there is no tunnel
    }
}

/// Cammand to hook shut down the tunnel
///
/// This will kill the tunnel process, if it's still running. The `context` parameter is the same that is given to the
/// [mange](tauri::Builder::manage) method in the [main] function, and is passed in by the Tauri app framework.
#[command]
fn end_tunnel(context: State<'_, Context>) {
    kill_tunnel((*context).clone());
}
