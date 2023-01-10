use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

const SETTING_PATH: &str = "setting.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    storage: Storage,
    network: Network,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
    pub pending_path: PathBuf,
    pub repository_path: PathBuf,
}

/// 网络相关配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub use_proxy: bool,
    pub proxy: Option<String>,
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref SETTING: Mutex<Setting> = Mutex::new(Setting::get().unwrap());
}

impl Setting {
    /// - `new` 获取一个默认的配置结构
    /// - 用于配置不存在时，生成默认的配置,并写入文件,并返回文件
    pub fn new() -> Result<Setting, std::io::Error> {
        use tauri::api::path::{download_dir, video_dir};

        let mut video_dir = video_dir().unwrap();
        video_dir.push("AnimeRepository");

        let mut download_dir = download_dir().unwrap();
        download_dir.push("AnimeRepository");

        let setting = Setting {
            storage: Storage {
                pending_path: download_dir,
                repository_path: video_dir,
            },
            network: Network {
                use_proxy: false,
                proxy: None,
            },
        };
        setting.write_to_file(SETTING_PATH)?;
        return Ok(setting);
    }

    /// 将配置写入文件
    pub fn write_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(toml::to_string(self).unwrap().as_bytes())?;
        return Ok(());
    }

    /// ### 获取配置
    pub fn get() -> Result<Setting, GetConfigError> {
        let f = File::open(SETTING_PATH);

        let setting = match f {
            Ok(mut file) => {
                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents).unwrap();
                toml::from_str(&file_contents)?
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match Setting::new() {
                    Ok(setting) => setting,
                    Err(e) => panic!("Problem creating the setting: {:?}", e),
                },
                other_error => panic!("Problem opening the setting: {:?}", other_error),
            },
        };

        return Ok(setting);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetConfigError {
    #[error(transparent)]
    DeserializeError(#[from] toml::de::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
