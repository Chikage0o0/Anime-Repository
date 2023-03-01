use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

use crate::model::setting::Setting;

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
