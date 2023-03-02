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

use std::path::PathBuf;

use crate::controller::*;

use once_cell::sync::OnceCell;

use tauri::SystemTray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
pub static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();

fn init_log() {
    use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger};
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::{append::console::ConsoleAppender, config::Root};

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[Anime-Repository] [{d(%Y-%m-%d %H:%M:%S)}] [{l}] {t} - {m}{n}",
        )))
        .build(
            PathBuf::from(tauri::api::path::config_dir().unwrap())
                .join("AnimeRepository")
                .join("log.txt"),
        )
        .unwrap();

    let config = if cfg!(debug_assertions) {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[Anime-Repository] [{d(%Y-%m-%d %H:%M:%S)}] [{h({l})}] {t} - {m}{n}",
            )))
            .build();
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .additive(false)
                    .build("app", LevelFilter::Debug),
            )
            .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
            .unwrap()
    } else {
        Config::builder()
            .appender(Appender::builder().build("file", Box::new(file)))
            .logger(
                Logger::builder()
                    .appender("file")
                    .additive(false)
                    .build("app", LevelFilter::Info),
            )
            .build(Root::builder().appender("file").build(LevelFilter::Info))
            .unwrap()
    };

    let _ = log4rs::init_config(config).unwrap();
}

fn main() {
    init_log();

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

    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Ready { .. } => {
            handler::run();
        }
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit {} => {
            handler::stop(|| utils::tauri::exit_app());
        }
        _ => {}
    });
}
