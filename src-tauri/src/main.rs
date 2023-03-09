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

use crate::{controller::*, model::setting::Setting, utils::tauri::*};
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use tauri::{Manager, SystemTray};

pub static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();
#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    init_log();

    log::info!("start app");

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .system_tray(SystemTray::new().with_menu(get_tray_menu()))
        .on_system_tray_event(tray_event)
        .invoke_handler(tauri::generate_handler![
            get_setting,
            save_setting,
            get_title,
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

    // 静默启动时不创建窗口
    if !Setting::get_slient_boot() {
        create_window(&app.handle())
    }

    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Ready { .. } => {
            handler::run();
        }
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Exit {} => {
            handler::stop(|| exit_app());
        }
        _ => {}
    });
}

fn init_log() {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    };
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;

    // 测试环境下输出到控制台，正式环境下输出到文件
    let config = if cfg!(debug_assertions) {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S)} | {({h({l})}):5.5} | {t}:{L} - {m}{n}",
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
        // 日志文件夹
        let log_path = PathBuf::from(tauri::api::path::config_dir().unwrap())
            .join("AnimeRepository")
            .join("log");

        // 自动归档日志
        let window_size = 3; // log0, log1, log2
        let fixed_window_roller = FixedWindowRoller::builder()
            .build(log_path.join("old-{}.log").to_str().unwrap(), window_size)
            .unwrap();

        let size_limit = 5 * 1024 * 1024; // 5MB as max log file size to roll
        let size_trigger = SizeTrigger::new(size_limit);
        let compound_policy =
            CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

        let file = RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {t} - {m}{n}",
            )))
            .build(log_path.join("latest.log"), Box::new(compound_policy))
            .unwrap();

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
