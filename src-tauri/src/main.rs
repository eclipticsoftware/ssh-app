#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, time::Duration};

use serde::Deserialize;
use tauri::command;
use tauri::{window::Window, State, RunEvent};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use ssh_tunnel::{SshConfig, SshHandle, SshStatus, SshTunnel};

fn main() {
    let context = Context::new();

    let ctx_win_capt = context.clone();
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
        if let RunEvent::ExitRequested {api: _, ..} = event {
            println!("Exiting app...");
            kill_tunnel(ctx_app.clone());
        }
    })
}

struct ContextInner {
    tunnel: Option<SshTunnel>,
    //handle: Option<JoinHandle<(SshStatus, ExitStatus)>>,
    window: Option<Window>,
}

impl ContextInner {
    fn new() -> Self {
        ContextInner {
            tunnel: None,
            //handle: None,
            window: None,
        }
    }
}

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

#[derive(Deserialize, Debug)]
struct UserSettings<'a> {
    host: &'a str,
    user: &'a str,
    port: &'a str,
    key_path: &'a str,
}

impl UserSettings<'_> {
    fn to_config(&self) -> Result<SshConfig, std::num::ParseIntError> {

        let port = self.port.parse()?;
        Ok(SshConfig::new(
            self.host,
            self.user,
            self.key_path,
            "localhost",
            port,
            5432,
            10,
            &["-t", "-t"],
        ))
    }
}

#[command]
fn start_tunnel(settings: UserSettings<'_>, context: State<'_, Context>) -> String {
    let config = match settings.to_config() {
        Ok(cfg) => {
            cfg
        },
        Err(_err) => {
            return SshStatus::ConfigError("Illegal port value".to_string()).to_signal();
        }
    };
    
    println!("Starting tunnel: {:?}", settings);
    let result = spawn_new_tunnel(config, (*context).clone());
    manage_spawn_result(result, (*context).clone())
}

fn spawn_new_tunnel(
    config: SshConfig,
    context: Context,
) -> Result<(SshTunnel, SshHandle), SshStatus> {
    let context_thread = context.clone();
    println!("Spawning new tunnel");
    ssh_tunnel::start_and_watch_ssh_tunnel(config.clone(), move |status| {
        println!("Status: {:?}", status);
        let status = match status {
            SshStatus::Dropped => { // Attempt to reconnect in separate thread
                let context_retry = context_thread.clone();
                thread::spawn(move || {
                    attempt_reconnect(config, context_retry, 5);
                });
                SshStatus::Retrying
            },
            _ => {
                status
            }
        };

        let ctxt = context_thread.lock();
        ctxt.window
            .as_ref()
            .unwrap()
            .emit("tunnel_status", Some(status.to_signal()))
            .expect("emit drop failed");
    })
}

fn manage_spawn_result(
    result: Result<(SshTunnel, SshHandle), SshStatus>,
    context: Context,
) -> String {
    println!("Tunnel spawned");
    let mut ctxt = context.lock();
    let status_signal = match result {
        Ok((tnl, _hndl)) => {
            println!("Tunnel running");
            ctxt.tunnel = Some(tnl);
            //ctxt.handle = Some(hndl);
            SshStatus::Connected.to_signal()
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

fn attempt_reconnect(config: SshConfig, context: Context, tries: u8) -> String {
    println!("Attempting to reconnect. {tries} tries left");
    let result = spawn_new_tunnel(config.clone(), context.clone());
    if result.is_ok() {
        manage_spawn_result(result, context)
    } else if tries == 0 {
        manage_spawn_result(Err(SshStatus::Dropped), context)
    } else {
        thread::sleep(Duration::from_secs(3));
        attempt_reconnect(config, context, tries - 1)
    }
}

fn kill_tunnel(context: Context) {
    let context = context.lock();
    if context.tunnel.is_none() {
        return;
    }
    println!("Killing tunnel");
    let mut tunnel = context.tunnel.as_ref().unwrap().lock().unwrap();
    match tunnel.kill() {
        Ok(_) => {
            println!("killed");
        }
        Err(err) => {
            println!("Not killed: {err}")
        }
    }
}

#[command]
fn end_tunnel(context: State<'_, Context>) {
    kill_tunnel((*context).clone());
}
