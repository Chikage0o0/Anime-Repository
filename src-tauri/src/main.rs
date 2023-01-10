#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Window;

mod model;

#[tauri::command]
fn get_setting(window: Window) {
    window
        .emit("get_setting", model::setting::Setting::get().unwrap())
        .unwrap();
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
