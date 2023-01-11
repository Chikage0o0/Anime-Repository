#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use model::setting::Setting;

mod model;

#[tauri::command]
async fn get_setting() -> Setting {
    Setting::global().clone()
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
