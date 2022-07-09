#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{thread, time::Duration};

use serde::Deserialize;
use tauri::command;
use tauri::{window::Window, State};

struct ContextInner {
    proc: Option<JoinHandle<()>>,
    window: Option<Window>,
    run: bool,
}

impl ContextInner {
    fn new() -> Self {
        ContextInner {
            proc: None,
            window: None,
            run: false,
        }
    }
}

struct Context(Arc<Mutex<ContextInner>>);

#[derive(Deserialize, Debug)]
struct UserSettings<'a> {
    host: &'a str,
    user: &'a str,
    port: &'a str,
    key_path: &'a str,
}

#[command]
fn start_tunnel(settings: UserSettings<'_>, proc: State<'_, Context>) {
    println!("Starting tunnel: {:?}", settings);
    let proc_thread = proc.0.clone();
    let mut proc = proc.0.lock().unwrap();
    proc.run = true;
    proc.proc = Some(thread::spawn(move || {
        let mut checks = 300;
        let mut dropped = true;
        while checks > 0 {
            thread::sleep(Duration::from_millis(100));
            if !proc_thread.lock().unwrap().run {
                println!("Stopping early");
                dropped = false;
                break;
            }
            checks -= 1;
        }
        if dropped {
            let mut proc = proc_thread.lock().unwrap();
            proc.run = false;
            proc.window
                .as_ref()
                .unwrap()
                .emit("tunnel_error", Some("DROPPED".to_string()))
                .expect("emit drop failed");
        }
        println!("Finishing tunnel");
    }));

    if proc.window.is_none() {
        println!("Window doesn't exist!");
    } else {
        proc.window
            .as_ref()
            .unwrap()
            .emit("tunnel_connected", Some("Connected".to_string()))
            .expect("emit conn failed");
    }
}

#[command]
fn end_tunnel(proc: State<'_, Context>) {
    println!("Killing tunnel");
    let mut proc = proc.0.lock().unwrap();
    if proc.run {
        println!("Stopping the thread");
        proc.run = false;
    }
}

fn main() {
    let context = Context(Arc::new(Mutex::new(ContextInner::new())));

    let ctx_win_capt = Context(context.0.clone());
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
