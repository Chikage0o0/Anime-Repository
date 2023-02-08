#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use model::setting::{Setting, SettingError};

mod http;
mod model;
mod scan;

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
    scan::watch::watch_pending_path();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_setting, save_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
