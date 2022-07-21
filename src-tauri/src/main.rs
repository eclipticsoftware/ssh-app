#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, time::Duration};

use serde::Deserialize;
use tauri::command;
use tauri::{window::Window, RunEvent, State};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use ssh_tunnel::{ChildProc, SshConfig, SshHandle, SshStatus, SshTunnel, TunnelChild};

fn main() {
    let context = Context::new();

    let ctx_win_capt = context.clone();

    #[allow(unused_mut)]
    let mut app = tauri::Builder::default()
        .on_page_load(move |window, _| {
            println!("Setting window");
            ctx_win_capt.lock().window = Some(window);
        })
        .manage(context.clone())
        .invoke_handler(tauri::generate_handler![start_tunnel, end_tunnel,])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(ActivationPolicy::Regular);

    let ctx_app = context.clone();
    app.run(move |_app_handle, event| {
        if let RunEvent::ExitRequested { api: _, .. } = event {
            println!("Exiting app...");
            kill_tunnel(ctx_app.clone());
        }
    })
}


/// Maintains the app context
struct ContextInner {
    /// Container for the tunnel process
    tunnel: Option<SshTunnel<TunnelChild>>,
    /// Handle for the app window
    window: Option<Window>,
}

impl ContextInner {
    fn new() -> Self {
        ContextInner {
            tunnel: None,
            window: None,
        }
    }
}

/// Threadsafe context
struct Context(Arc<Mutex<ContextInner>>);

impl Context {
    fn new() -> Self {
        Context(Arc::new(Mutex::new(ContextInner::new())))
    }

    fn clone(&self) -> Self {
        Context(self.0.clone())
    }

    fn lock(&self) -> MutexGuard<ContextInner> {
        self.0.lock().unwrap()
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
        let flags = if cfg!(target_os = "windows") {
            vec!["-T"]
        } else {
            vec!["-t", "-t"]
        };

        Ok(SshConfig::new(
            self.host,
            self.user,
            self.key_path,
            "localhost",
            port,
            5432,
            10,
            &flags
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
            return SshStatus::ConfigError("Illegal port value".to_string()).to_signal();
        }
    };

    println!("Starting tunnel: {:?}", settings);
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
    let context_thread = context.clone();
    let config_thread = Arc::new(Mutex::new(config.clone()));
    let callback = Arc::new(Mutex::new(
        move |status| {
        println!("Status: {:?}", status);
        let status = match status {
            SshStatus::Dropped => {
                // Attempt to reconnect in separate thread
                let context_retry = context_thread.clone();
                let config_thread = config_thread.clone();
                thread::spawn(move || {
                    attempt_reconnect(config_thread, context_retry, 5);
                });
                SshStatus::Reconnecting
            }
            _ => status,
        };

        let ctxt = context_thread.lock();
        ctxt.window
            .as_ref()
            .unwrap()
            .emit("tunnel_status", Some(status.to_signal()))
            .expect("emit status failed");
    }
    ));
    ssh_tunnel::start_and_watch_ssh_tunnel(config, callback, false)
}


/// Checks the result sent from `spawn_new_tunnel()` or `attempt_reconnect()` and updates the status on the front end
fn manage_spawn_result(
    result: Result<(SshTunnel<TunnelChild>, SshHandle), SshStatus>,
    context: Context,
) -> String {
    let mut ctxt = context.lock();
    let status_signal = match result {
        Ok((tnl, _hndl)) => {
            ctxt.tunnel = Some(tnl);
            //ctxt.handle = Some(hndl);
            SshStatus::Connecting.to_signal()
        }
        Err(status) => {
            println!("error: {:?}", status);
            status.to_signal()
        }
    };

    ctxt.window
        .as_ref()
        .unwrap()
        .emit("tunnel_status", Some(status_signal.clone()))
        .expect("emit status failed");

    status_signal
}


/// Attempts to reconnect to the ssh server if the tunnel process is dropped
///
/// Will make `tries` attempts with a 3 second wait between each attempt.
fn attempt_reconnect(config: Arc<Mutex<SshConfig>>, context: Context, tries: u8) -> String {
    println!("Attempting to reconnect. {tries} tries left");
    let cfg = {
        config.lock().expect("This should not happen: failed to lock config").clone()
    };
    let result = spawn_new_tunnel(cfg, context.clone());
    if result.is_ok() {
        manage_spawn_result(result, context)
    } else if tries == 0 {
        manage_spawn_result(Err(SshStatus::Dropped), context)
    } else {
        thread::sleep(Duration::from_secs(3));
        attempt_reconnect(config, context, tries - 1)
    }
}


/// Kills the tunnel process if it's running
fn kill_tunnel(context: Context) {
    let context = context.lock();
    if context.tunnel.is_none() {
        return;
    }
    println!("Killing tunnel");
    let mut tunnel = context.tunnel.as_ref().unwrap().lock().unwrap();
    tunnel.kill();
}


/// Cammand to shut down the tunnel
#[command]
fn end_tunnel(context: State<'_, Context>) {
    kill_tunnel((*context).clone());
}
