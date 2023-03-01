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

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use crate::controller::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    handler::run();
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                if let Some(window) = app.get_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                    return;
                }

                tauri::window::WindowBuilder::new(
                    app,
                    "main".to_string(),
                    tauri::WindowUrl::App("index.html".into()),
                )
                .title("Anime-Repository")
                .center()
                .fullscreen(false)
                .min_inner_size(600.0, 600.0)
                .decorations(false)
                .inner_size(1000.0, 600.0)
                .resizable(true)
                .build()
                .unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.close();
                    }
                    app.exit(0)
                }
                "open" => {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                        return;
                    }

                    tauri::window::WindowBuilder::new(
                        app,
                        "main".to_string(),
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .title("Anime-Repository")
                    .center()
                    .fullscreen(false)
                    .min_inner_size(600.0, 600.0)
                    .decorations(false)
                    .inner_size(1000.0, 600.0)
                    .resizable(true)
                    .build()
                    .unwrap();
                }
                _ => {}
            },
            _ => {}
        })
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
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
