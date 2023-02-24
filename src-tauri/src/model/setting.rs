use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use sys_locale::get_locale;

const SETTING_PATH: &str = "config/setting.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub ui: UI,
    pub storage: Storage,
    pub network: Network,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UI {
    pub lang: String,
    pub theme: Theme,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub pending_path: PathBuf,
    pub pending_path_scan_interval: u64,
    pub repository_path: PathBuf,
}

/// 网络相关配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    use_proxy: bool,
    proxy: Option<String>,
}
lazy_static! {
    static ref CONFIG: Mutex<Setting> = Mutex::new(Setting::get_from_file().unwrap());
}
impl Setting {
    fn new() -> Result<Setting, std::io::Error> {
        use tauri::api::path::{download_dir, video_dir};

        let mut video_dir = video_dir().unwrap();
        video_dir.push("AnimeRepository");

        let mut download_dir = download_dir().unwrap();
        download_dir.push("AnimeRepository");

        let setting = Setting {
            ui: UI {
                lang: get_locale().unwrap_or_else(|| String::from("en-US")),
                theme: Theme::Auto,
            },
            storage: Storage {
                pending_path: download_dir,
                pending_path_scan_interval: 60,
                repository_path: video_dir,
            },
            network: Network {
                use_proxy: false,
                proxy: None,
            },
        };

        setting.write_to_file()?;
        log::info!("Setting init: {:?}", &setting);
        Ok(setting)
    }

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

    fn get_from_file() -> Result<Setting, SettingError> {
        let f = File::open(SETTING_PATH);
        let setting = match f {
            Ok(mut file) => {
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents).unwrap();
                toml::from_str(&file_contents)?
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Setting::new()
                    .unwrap_or_else(|e| panic!("Problem creating the setting: {:?}", e)),
                other_error => panic!("Problem opening the setting: {:?}", other_error),
            },
        };

        Ok(setting)
    }

    pub fn get_proxy() -> Option<String> {
        let setting = Setting::get();
        if setting.network.use_proxy && setting.network.proxy.is_some() {
            setting.network.proxy
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
}

#[derive(thiserror::Error, Debug)]
pub enum SettingError {
    #[error(transparent)]
    DeserializeError(#[from] toml::de::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
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
        setting.network.use_proxy = false;
        setting.network.proxy = Some("http://127.0.0.1:8080".to_string());
        assert!(Setting::apply(setting).is_ok());

        let setting = Setting::get();
        assert_eq!(
            setting.network.proxy,
            Some("http://127.0.0.1:8080".to_string())
        );
    }
}
