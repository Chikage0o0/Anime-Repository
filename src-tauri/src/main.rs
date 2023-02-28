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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    handler::run();
    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
