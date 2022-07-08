#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::Deserialize;
use tauri::command;

#[derive(Deserialize)]
#[derive(Debug)]
struct UserSettings<'a> {
    host: &'a str,
    user: &'a str,
    port: &'a str,
    key_path: &'a str,
}

#[command]
fn command_start_tunnel(settings: UserSettings<'_>) {
    println!("Starting tunnel: {:?}", settings);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command_start_tunnel])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
