use crate::model::setting::{Setting, SettingError};

#[tauri::command]
pub fn get_setting() -> Setting {
    Setting::get()
}

#[tauri::command]
pub fn save_setting(setting: Setting) -> Result<(), SettingError> {
    Setting::apply(setting)
}
