// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
mod commands;
use commands::{get_matrice, set_recording};
mod state;
use state::SonarState;

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(SonarState::new())))
        .invoke_handler(tauri::generate_handler![get_matrice,set_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}