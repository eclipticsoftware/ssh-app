#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
//use std::{thread, time::Duration};

use serde::Deserialize;
use tauri::command;
use tauri::{window::Window, State};

use ssh_tunnel::{SshTunnel, SshConfig, SshStatus, ExitStatus};

fn main() {
    let context = Context::new();

    let ctx_win_capt = context.clone();
    tauri::Builder::default()
        .on_page_load(move |window, _| {
            println!("Setting window");
            ctx_win_capt.0.lock().unwrap().window = Some(window);
        })
        .manage(context)
        .invoke_handler(tauri::generate_handler![start_tunnel, end_tunnel,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
    
}

#[derive(Deserialize, Debug)]
struct UserSettings<'a> {
    host: &'a str,
    user: &'a str,
    port: &'a str,
    key_path: &'a str,
}

impl UserSettings<'_> {

    fn to_config(&self) -> SshConfig {
        SshConfig::new(
            self.host,
            self.user,
            self.key_path,
            "localhost",
            self.port.parse().unwrap(),
            5432,
            10,
            &["-t", "-t"]
        )
    }
}

#[command]
fn start_tunnel(settings: UserSettings<'_>, context: State<'_, Context>) -> String {

    println!("Starting tunnel: {:?}", settings);
    let context_thread = context.0.clone();
    let mut ctxt = context.0.lock().unwrap();
    let result = ssh_tunnel::start_and_watch_ssh_tunnel(
        settings.to_config(),
        move |status| {
            let ctxt = context_thread.lock().unwrap();
            println!("Status: {:?}", status);
            match status {
                SshStatus::Dropped => {
                    println!("Dropped connection");
                    ctxt.window
                        .as_ref()
                        .unwrap()
                        .emit("tunnel_error", Some("DROPPED".to_string()))
                        .expect("emit drop failed");
                },
                SshStatus::Unreachable => {
                    println!("Unreachable");
                    ctxt.window
                        .as_ref()
                        .unwrap()
                        .emit("tunnel_error", Some("UNREACHABLE".to_string()))
                        .expect("emit unreachable failed");
                },
                SshStatus::Denied => {
                    println!("Denied");
                    ctxt.window
                        .as_ref()
                        .unwrap()
                        .emit("tunnel_error", Some("DENIED".to_string()))
                        .expect("emit denied failed");
                },
                SshStatus::Exited => {
                    println!("Exited cleanly");
                    ctxt.window
                        .as_ref()
                        .unwrap()
                        .emit("tunnel_error", Some("EXIT".to_string()))
                        .expect("emit eixt failed");
                },
                _ => {
                    println!("Unsupported status: {:?}", status);
                    ctxt.window
                        .as_ref()
                        .unwrap()
                        .emit("tunnel_error", Some("ERROR".to_string()))
                        .expect("emit exit failed");
                }
            }
        });
    
    match result {
        Ok((tnl, _hndl)) => {
            println!("Tunnel is started");
            ctxt.tunnel = Some(tnl);
            //ctxt.handle = Some(hndl);
        }
        Err(status) => {
            return match status {
                SshStatus::Unreachable => "UNREACHABLE".to_string(),
                SshStatus::Denied => "DENIED".to_string(),
                SshStatus::ProcError(err) => format!("ERROR: {err}"),
                _ => "ERROR: Unspecified".to_string()
            }
        }
    }

    if ctxt.window.is_none() {
        println!("Window doesn't exist!");
        "ERROR: Unspecified".to_string()
    } else {
        ctxt.window
            .as_ref()
            .unwrap()
            .emit("tunnel_connected", Some("SUCCESS".to_string()))
            .expect("emit conn failed");
        "SUCCESS".to_string()
    }
}

#[command]
fn end_tunnel(context: State<'_, Context>) {
    println!("Killing tunnel");
    let context = context.0.lock().unwrap();
    let mut tunnel = context.tunnel.as_ref().unwrap().lock().unwrap();
    match tunnel.kill() {
        Ok(_) => {
            println!("killed");
        },
        Err(err) => {
            println!("Not killed: {err}")
        }
    }
}
