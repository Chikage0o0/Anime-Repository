#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod controller;
mod data;
mod handler;
mod http;
mod model;
mod service;
mod utils;

use crate::controller::*;
use once_cell::sync::OnceCell;

use tauri::SystemTray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
pub static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();

fn main() {
    env_logger::init();

    let app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(utils::tauri::get_tray_menu()))
        .on_system_tray_event(utils::tauri::tray_event)
        .invoke_handler(tauri::generate_handler![
            get_setting,
            save_setting,
            get_tvshow_title,
            get_subscribe_rule,
            get_subscribe_rules,
            delete_subscribe_rule,
            insert_subscribe_rule,
            get_unrecognized_videos_list,
            delete_unrecognized_video_info,
            update_unrecognized_video_info,
            refresh_unrecognized_videos_list,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    APP_HANDLE.set(app.handle()).unwrap();

    app.run(move |app_handle, event| match event {
        tauri::RunEvent::Ready { .. } => {
            handler::run();
        }
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit {} => {
            handler::stop();
            app_handle.exit(0);
        }
        _ => {}
    });
}
