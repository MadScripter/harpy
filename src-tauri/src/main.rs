// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod commands;
mod constants;
mod downloader;
mod errors;
mod hasher;
mod models;
mod palia;
mod progress;
mod registry;
mod requisites;
mod service;
mod state;
mod tokens;
mod utils;

use state::AppState;
use tauri::{AppHandle, Manager, RunEvent};

use crate::commands::{init, install, login, query_disk_space};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            if argv.contains(&String::from("--uninstall")) {
                app.emit_all("app:uninstall", ()).unwrap();
            }
        }))
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            init,
            login,
            install,
            query_disk_space
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(on_run)
}

fn on_run(app: &AppHandle, event: RunEvent) {
    if let tauri::RunEvent::ExitRequested { api, .. } = event {
        // TODO: Cleanup before closing
    };
}
