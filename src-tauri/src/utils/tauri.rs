use crate::{handler, model::setting::Setting};
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn get_tray_menu() -> SystemTrayMenu {
    let quit_str;
    let open_str;
    let lang = Setting::get_lang();
    match lang.as_str() {
        "zh_CN" => {
            quit_str = "退出";
            open_str = "打开";
        }
        "ja_JP" => {
            quit_str = "終了";
            open_str = "開く";
        }
        "ko_KR" => {
            quit_str = "끝내기";
            open_str = "열기";
        }
        "ru_RU" => {
            quit_str = "Выход";
            open_str = "Открыть";
        }
        "fr_FR" => {
            quit_str = "Quitter";
            open_str = "Ouvrir";
        }
        "de_DE" => {
            quit_str = "Beenden";
            open_str = "Öffnen";
        }
        "es_ES" => {
            quit_str = "Salir";
            open_str = "Abrir";
        }
        "pt_PT" => {
            quit_str = "Sair";
            open_str = "Abrir";
        }
        "it_IT" => {
            quit_str = "Esci";
            open_str = "Apri";
        }
        "nl_NL" => {
            quit_str = "Stoppen";
            open_str = "Openen";
        }
        "pl_PL" => {
            quit_str = "Wyjdź";
            open_str = "Otwórz";
        }
        "tr_TR" => {
            quit_str = "Çıkış";
            open_str = "Aç";
        }
        _ => {
            quit_str = "Quit";
            open_str = "Open";
        }
    }

    let quit = CustomMenuItem::new("quit".to_string(), quit_str);
    let open = CustomMenuItem::new("open".to_string(), open_str);

    SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
}

pub fn tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => create_window(app),
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                handler::stop(|| exit_app());
            }
            "open" => create_window(app),
            _ => {}
        },
        _ => {}
    }
}

pub fn send_event(window: &str, event: &str, data: impl serde::Serialize + Clone) {
    if let Some(app_handle) = crate::APP_HANDLE.get() {
        if let Some(window) = app_handle.get_window(window) {
            let _ = window.emit(event, data);
        }
    }
}

pub fn exit_app() {
    Setting::write_to_file().unwrap_or_else(|err| {
        log::error!(target: "app", "write setting to file failed: {}", err);
    });
    log::info!("exit app");
    crate::APP_HANDLE.get().unwrap().exit(0);
}

pub fn reboot_app() {
    Setting::write_to_file().unwrap_or_else(|err| {
        log::error!(target: "app", "write setting to file failed: {}", err);
    });
    log::info!("restart app");
    crate::APP_HANDLE.get().unwrap().restart();
}

pub fn create_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let builder = tauri::window::WindowBuilder::new(
        app_handle,
        "main".to_string(),
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Anime Repository")
    .center()
    .fullscreen(false)
    .min_inner_size(600.0, 600.0);

    let app = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let window = app.clone();
        app.once_global("show_window", move |_event| {
            if let Some(window) = window.get_window("main") {
                tauri::async_runtime::spawn(async move {
                    use std::time::Duration;
                    use tokio::time::sleep;
                    sleep(Duration::from_millis(100)).await;
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                });
            }
        });
    });

    #[cfg(target_os = "windows")]
    {
        use window_shadows::set_shadow;

        match builder
            .decorations(false)
            .inner_size(1000.0, 600.0)
            .visible(false)
            .build()
        {
            Ok(_) => {
                if let Some(window) = app_handle.get_window("main") {
                    let _ = set_shadow(&window, true);
                }
            }
            Err(err) => log::error!(target: "app", "{err}"),
        }
    }
}

pub fn set_auto_launch(switch: bool) -> Result<(), String> {
    let auto = get_auto_launch();

    let result = if switch {
        auto.enable()
    } else {
        auto.disable()
    };
    result.map_err(|err| err.to_string())?;

    log::info!(target: "app", "set auto launch: {}", switch);

    Ok(())
}

fn get_auto_launch() -> auto_launch::AutoLaunch {
    use auto_launch::AutoLaunch;
    use tauri::api::process::current_binary;
    use tauri::Env;

    let binding = current_binary(&Env::default()).unwrap();
    let app_path = binding.to_str().unwrap();

    let app_path = if app_path.starts_with(r"\\?\") {
        &app_path[4..]
    } else {
        app_path
    };

    let app_name = "anime-repository";
    let args: &[&str; 0] = &[];
    let auto = AutoLaunch::new(app_name, app_path, args);

    auto
}

pub fn send_storage_notification(file_name: &str) {
    use tauri::api::notification::Notification;

    let title;
    match Setting::get_lang().as_str() {
        "zh_CN" => title = "Anime Repository: 新视频已经入库！",
        "ja_JP" => title = "Anime Repository: 新しい動画が追加されました！",
        _ => title = "Anime Repository: New video has been added!",
    }

    let context = tauri::generate_context!();
    Notification::new(&context.config().tauri.bundle.identifier)
        .title(title)
        .body(format!("{}", file_name))
        .show()
        .unwrap();
}
