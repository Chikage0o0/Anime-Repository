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
            .title("Anime Repository")
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
                handler::stop(|| exit_app());
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
                .title("Anime Repository")
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
    log::info!("exit app");
    crate::APP_HANDLE.get().unwrap().exit(0);
}
