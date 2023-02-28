use config::Config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use sys_locale::get_locale;

const SETTING_PATH: &str = "config/setting.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    ui: UI,
    storage: Storage,
    network: Network,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct UI {
    lang: String,
    theme: Theme,
    primary_color: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Theme {
    Dark,
    Light,
    Auto,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Storage {
    pending_path: PathBuf,
    pending_path_scan_interval: u64,
    repository_path: PathBuf,
}

/// 网络相关配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    use_proxy: String,
    proxy: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Setting> = Mutex::new(Setting::new().unwrap());
}
impl Setting {
    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        log::info!("Writing setting to file");
        let path = Path::new(SETTING_PATH);
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).unwrap();
        }

        let mut file = File::create(path)?;
        let toml_str = toml::to_string(self).unwrap();
        file.write_all(toml_str.as_bytes())?;
        Ok(())
    }

    fn new() -> Result<Setting, SettingError> {
        use tauri::api::path::{download_dir, video_dir};
        let mut video_dir = video_dir().unwrap();
        video_dir.push("AnimeRepository");

        let mut download_dir = download_dir().unwrap();
        download_dir.push("AnimeRepository");

        let s = Config::builder()
            .set_default(
                "ui.lang",
                get_locale()
                    .unwrap_or_else(|| String::from("en-US"))
                    .replace("-", "_"),
            )?
            .set_default("ui.theme", "Auto")?
            .set_default("ui.primary_color", "gray")?
            .set_default("storage.pending_path", download_dir.to_str())?
            .set_default("storage.pending_path_scan_interval", 60)?
            .set_default("storage.repository_path", video_dir.to_str())?
            .set_default("network.use_proxy", "false")?
            .set_default("network.proxy", "")?
            .add_source(config::File::with_name(SETTING_PATH).required(false))
            .build()?;

        Ok(s.try_deserialize()?)
    }

    pub fn get_proxy() -> Option<String> {
        let network = CONFIG.lock().unwrap().network.clone();
        if network.use_proxy == "true" && network.proxy.len() > 0 {
            Some(network.proxy)
        } else {
            None
        }
    }

    pub fn get() -> Setting {
        CONFIG.lock().unwrap().clone()
    }

    pub fn apply(setting: Setting) -> Result<(), SettingError> {
        let mut old_setting = CONFIG.lock().unwrap();
        setting.write_to_file()?;
        *old_setting = setting;
        log::info!("Setting applied");
        Ok(())
    }

    pub fn get_scan_interval() -> u64 {
        CONFIG.lock().unwrap().storage.pending_path_scan_interval
    }

    pub fn get_pending_path() -> PathBuf {
        CONFIG.lock().unwrap().storage.pending_path.clone()
    }

    pub fn get_repository_path() -> PathBuf {
        CONFIG.lock().unwrap().storage.repository_path.clone()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SettingError {
    #[error(transparent)]
    DeserializeError(#[from] toml::de::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
}

impl serde::Serialize for SettingError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mut setting = Setting::get();
        setting.network.use_proxy = "false".to_string();
        setting.network.proxy = "http://127.0.0.1:8080".to_string();
        assert!(Setting::apply(setting).is_ok());

        let setting = Setting::get();
        assert_eq!(setting.network.proxy, "http://127.0.0.1:8080".to_string());
    }
}
