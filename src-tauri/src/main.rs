#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::mem;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, time::Duration};

//use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::Deserialize;
use tauri::api::process::{Command, CommandChild, CommandEvent};
use tauri::async_runtime::{block_on, Receiver};
use tauri::command;
use tauri::{window::Window, RunEvent, State};

use tokio::sync::mpsc::error::TryRecvError;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use ssh_tunnel::{ChildProc, ExitCondition, SshConfig, SshHandle, SshStatus, SshTunnel, TunnelChild};

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
        if let RunEvent::ExitRequested { api: _, .. } = event {
            println!("Exiting app...");
            kill_tunnel(ctx_app.clone());
        }
    })
}

struct ContextInner {
    tunnel: Option<SshTunnel<TunnelChild>>,
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

struct TauriChild {
    child: Option<CommandChild>,
    rx: Receiver<CommandEvent>,
    stderr: String,
}

impl TauriChild {
    fn read_stdout_line(&mut self) -> Option<String> {
        while let Some(event) = block_on(self.rx.recv()) {
            match event {
                CommandEvent::Stdout(line) => return Some(line),
                CommandEvent::Stderr(line) => {
                    println!("Got stderr: {line}");
                    self.stderr.push_str(&line);
                }
                _ => {
                    println!("Got other event: {:?}", event);
                }
            }
        }
        None
    }

    /// Capture remaining stderr data and return it
    fn read_stderr(&mut self) -> Option<String> {
        println!("Reading stderr: so far we have:");
        println!("{}", self.stderr);
        while let Some(event) = block_on(self.rx.recv()) {
            if let CommandEvent::Stdout(line) = event {
                self.stderr.push_str(&line);
            }
        }
        if !self.stderr.is_empty() {
            Some(self.stderr.clone())
        } else {
            None
        }
    }
}

impl ChildProc for TauriChild {
    fn new(config: SshConfig) -> Result<SshTunnel<Self>, SshStatus> {
        let (rx, child) = Command::new("ssh")
            .args(config.to_args())
            .env_clear()
            .spawn()
            .map_err(|err| SshStatus::ProcError(err.to_string()))?;

        Ok(Arc::new(Mutex::new(TauriChild {
            child: Some(child),
            rx,
            stderr: String::new(),
        })))
    }

    // A few bytes will be printed to stdout once the login is complete. Wait for that output,
    // or throw an error if it fails to happen
    fn wait_for_start(&mut self) -> Option<SshStatus> {
        let mut len = 0;
        if let Some(line) = self.read_stdout_line() {
            println!("Received stdout: {line}");
            len = line.len();
        } else {
            println!("Failed to capture stdout");
        }

        if len < 15 {
            if let Some(err_msg) = self.read_stderr() {
                Some(SshStatus::from_stderr(&err_msg))
            } else {
                Some(SshStatus::ProcError("Failed to capture stderr".to_string()))
            }
        } else {
            None
        }
    }

    fn exited(&mut self) -> Option<ExitCondition> {
        match self.rx.try_recv() {
            Ok(event) => match event {
                CommandEvent::Stderr(line) => {
                    println!("Got stderr: {line}");
                    self.stderr.push_str(&line);
                    None
                }
                CommandEvent::Stdout(line) => {
                    println!("Stdout: {line}");
                    None
                }
                CommandEvent::Error(err) => {
                    println!("Command error: {err}");
                    Some(ExitCondition::ProcError)
                }
                CommandEvent::Terminated(payload) => {
                    println!("Termination payload: {:?}", payload);
                    if let Some(code) = payload.code {
                        Some(FromPrimitive::from_i32(code).expect("Exit code should always be i32"))
                    } else {
                        Some(ExitCondition::ProcError)
                    }
                }
                _ => {
                    println!("Unknown event: {:?}", event);
                    Some(ExitCondition::ProcError)
                }
            },
            Err(TryRecvError::Empty) => {
                //println!("Empty event buffer");
                None
            }
            Err(TryRecvError::Disconnected) => {
                println!("Disconnected without notice");
                Some(ExitCondition::ProcError)
            }
        }
    }

    //Capture stderr to discover exit reason
    fn exit_status(&mut self) -> SshStatus {
        if let Some(err_msg) = self.read_stderr() {
            println!("Error: {}", err_msg);
            SshStatus::from_stderr(&err_msg)
        } else {
            SshStatus::ProcError("Failed to capture stderr".to_string())
        }
    }

    fn kill(&mut self) {
        let mut child: Option<CommandChild> = None;
        mem::swap(&mut child, &mut self.child);
        if let Some(child) = child {
            match child.kill() {
                Ok(_) => {
                    println!("killed");
                }
                Err(err) => {
                    println!("Not killed: {err}")
                }
            }
        }
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
            &["-vvv", "-T"],
        ))
    }
}

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

fn spawn_new_tunnel(
    config: SshConfig,
    context: Context,
) -> Result<(SshTunnel<TunnelChild>, SshHandle), SshStatus> {
    let context_thread = context.clone();
    println!("Spawning new tunnel");
    ssh_tunnel::start_and_watch_ssh_tunnel(config.clone(), move |status| {
        println!("Status: {:?}", status);
        let status = match status {
            SshStatus::Dropped => {
                // Attempt to reconnect in separate thread
                let context_retry = context_thread.clone();
                thread::spawn(move || {
                    attempt_reconnect(config, context_retry, 5);
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
            .expect("emit drop failed");
    })
}

fn manage_spawn_result(
    result: Result<(SshTunnel<TunnelChild>, SshHandle), SshStatus>,
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
    tunnel.kill();
}

#[command]
fn end_tunnel(context: State<'_, Context>) {
    kill_tunnel((*context).clone());
}
