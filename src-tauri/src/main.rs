#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, time::Duration};

use serde::Deserialize;
use tauri::api::path;
use tauri::command;
use tauri::{window::Window, RunEvent, State};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use ssh_tunnel::{ChildProc, SshConfig, SshHandle, SshStatus, SshTunnel, TunnelChild};

fn main() {
    let mut logpath = path::home_dir().unwrap_or_else(|| PathBuf::from("."));

    logpath.push(".eclo-ssh-client.log");

    let logpath = logpath
        .into_os_string()
        .into_string()
        .expect("Failed to create log path");

    ssh_tunnel::configure_logger(&logpath);
    log::debug!("Running Eclo SSH Client");

    let context = Context::new();

    let ctx_win_capt = context.clone();

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
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
        .manage(context.clone())
        .invoke_handler(tauri::generate_handler![start_tunnel, end_tunnel,])
        .build(tauri::generate_context!())
        .map_err(|err| {
            log::error!("Failed to build tauri application");
            err
        })
        .expect("Failed to build tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(ActivationPolicy::Regular);

    let ctx_app = context.clone();
    app.run(move |_app_handle, event| {
        if let RunEvent::ExitRequested { api: _, .. } = event {
            log::debug!("Exiting app...");
            kill_tunnel(ctx_app.clone());
        }
    })
}

const MAX_RETRIES: u32 = 5;

/// Maintains the app context
struct ContextInner {
    /// Container for the tunnel process
    tunnel: Option<SshTunnel<TunnelChild>>,

    /// Handle for the app window
    window: Option<Window>,

    status: SshStatus,

    /// Number of connection retries left
    retries: u32,

    /// Whether the tunnel is reconnecting
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

/// Threadsafe context
struct Context(Arc<Mutex<ContextInner>>);

impl Context {
    fn new() -> Self {
        Context(Arc::new(Mutex::new(ContextInner::new())))
    }

    /// Returns a context wth a clone of the inner context
    fn clone(&self) -> Self {
        Context(self.0.clone())
    }

    /// Locks the inner context
    fn lock(&self) -> Result<MutexGuard<ContextInner>, SshStatus> {
        self.0
            .lock()
            .map_err(|err| SshStatus::AppError(format!("Failed to lock context: {err}")))
    }

    /// Locks the inner context
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

/// User settings passed down from the front end
#[derive(Deserialize, Debug)]
struct UserSettings<'a> {
    host: &'a str,
    user: &'a str,
    port: &'a str,
    key_path: &'a str,
}

impl UserSettings<'_> {
    /// Converts the user settings to an SshConfig object
    fn to_config(&self) -> Result<SshConfig, std::num::ParseIntError> {
        let port = self.port.parse()?;
        let flags = vec!["-t", "-t"];

        // if cfg!(target_os = "windows") {
        //     vec!["-T"]
        // } else {
        //     vec!["-t", "-t"]
        // };

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
/// This will spawn a new process and then check the results
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
fn spawn_new_tunnel(
    config: SshConfig,
    context: Context,
) -> Result<(SshTunnel<TunnelChild>, SshHandle), SshStatus> {
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

/// Checks the result sent from `spawn_new_tunnel()` or `attempt_reconnect()` and updates the status on the front end
fn manage_spawn_result(
    result: Result<(SshTunnel<TunnelChild>, SshHandle), SshStatus>,
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

/// Spawns a reconnect attempt
fn spawn_reconnect(config: Arc<Mutex<SshConfig>>, context: Context) {
    thread::spawn(move || {
        attempt_reconnect(config, context);
    });
}

/// Attempts to reconnect to the ssh server if the tunnel process is dropped
///
/// If the reconnect succeeds then it will update the system
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

/// Cammand to shut down the tunnel
#[command]
fn end_tunnel(context: State<'_, Context>) {
    kill_tunnel((*context).clone());
}
