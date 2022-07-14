#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex, MutexGuard};

use serde::Deserialize;
use tauri::command;
use tauri::{window::Window, State};

use ssh_tunnel::{SshTunnel, SshConfig, SshStatus};

fn main() {
    let context = Context::new();

    let ctx_win_capt = context.clone();
    tauri::Builder::default()
        .on_page_load(move |window, _| {
            println!("Setting window");
            ctx_win_capt.lock().window = Some(window);
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
    spawn_new_tunnel(settings.to_config(), (*context).clone())
}

fn spawn_new_tunnel(config: SshConfig, context: Context) -> String {
    let context_thread = context.clone();
    println!("Spawning new tunnel");
    let result = ssh_tunnel::start_and_watch_ssh_tunnel(
        config,
        move |status| {
            println!("Status: {:?}", status);
            let ctxt = context_thread.lock();
            ctxt.window
                .as_ref()
                .unwrap()
                .emit("tunnel_status", Some(status.to_signal()))
                .expect("emit drop failed");

        });

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


#[command]
fn end_tunnel(context: State<'_, Context>) {
    println!("Killing tunnel");
    let context = context.lock();
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
