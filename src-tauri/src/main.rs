#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use model::setting::{Setting, SettingError};

mod data;
mod handler;
mod http;
mod model;
mod service;
mod utils;

#[tauri::command]
async fn get_setting() -> Setting {
    Setting::get()
}

#[tauri::command]
fn save_setting(setting: Setting) -> Result<(), SettingError> {
    Setting::apply(setting)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    env_logger::init();
    handler::watcher::watch_pending_path();
    handler::pending_videos_list::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_setting, save_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
