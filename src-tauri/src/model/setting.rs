use config::Config;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use sys_locale::get_locale;

use crate::utils;
use crate::utils::tauri::reboot_app;

use super::nfo::ProviderKnown;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    ui: UI,
    storage: Storage,
    scraper: Scraper,
    network: Network,
    system: System,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Scraper {
    use_openai: bool,
    openai_key: String,
    default_lang: String,
    default_provider: ProviderKnown,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct System {
    auto_launch: bool,
    silent_start: bool,
    scan_interval: u64,
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
    repository_path: PathBuf,
    pending_path_last_scan: u64,
}

/// 网络相关配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    use_proxy: bool,
    proxy: String,
    openai_domain: String,
    retry_times: u8,
}

static CONFIG: Lazy<Mutex<Setting>> = Lazy::new(|| Mutex::new(Setting::new().unwrap()));

impl Setting {
    pub fn write_to_file() -> Result<(), std::io::Error> {
        log::debug!("Writing setting to file");
        let path = PathBuf::from(tauri::api::path::config_dir().unwrap())
            .join("AnimeRepository")
            .join("setting.toml");

        if let Some(p) = path.parent() {
            fs::create_dir_all(p).unwrap();
        }

        let mut file = File::create(path)?;
        let toml_str = toml::to_string(&Self::get()).unwrap();
        file.write_all(toml_str.as_bytes())?;
        Ok(())
    }

    fn new() -> Result<Setting, SettingError> {
        use tauri::api::path::{download_dir, video_dir};
        let mut video_dir = video_dir().unwrap();
        video_dir.push("AnimeRepository");

        let mut download_dir = download_dir().unwrap();
        download_dir.push("AnimeRepository");

        let lang = get_locale().unwrap_or_else(|| String::from("en-US"));

        let setting_file = PathBuf::from(tauri::api::path::config_dir().unwrap())
            .join("AnimeRepository")
            .join("setting.toml");

        let s = Config::builder()
            .set_default("ui.lang", lang.replace("-", "_"))?
            .set_default("ui.theme", "Auto")?
            .set_default("ui.primary_color", "gray")?
            .set_default("storage.pending_path", download_dir.to_str())?
            .set_default("storage.repository_path", video_dir.to_str())?
            .set_default("storage.pending_path_last_scan", utils::get_now_time())?
            .set_default("scraper.use_openai", false)?
            .set_default("scraper.openai_key", "")?
            .set_default("scraper.default_lang", lang)?
            .set_default("scraper.default_provider", "tmdb")?
            .set_default("network.use_proxy", false)?
            .set_default("network.proxy", "")?
            .set_default("network.openai_domain", "api.openai.com")?
            .set_default("network.retry_times", 3)?
            .set_default("system.auto_launch", false)?
            .set_default("system.silent_start", false)?
            .set_default("system.scan_interval", 60)?
            .add_source(config::File::with_name(setting_file.to_str().unwrap()).required(false))
            .build()?;

        Ok(s.try_deserialize()?)
    }
}
impl Setting {
    pub fn get() -> Setting {
        CONFIG.lock().unwrap().clone()
    }

    pub fn get_retry_times() -> u8 {
        CONFIG.lock().unwrap().network.retry_times
    }

    pub fn get_proxy() -> Option<String> {
        let network = CONFIG.lock().unwrap().network.clone();
        if network.use_proxy && network.proxy.len() > 0 {
            Some(network.proxy)
        } else {
            None
        }
    }

    pub fn apply(setting: Setting) -> Result<(), SettingError> {
        let mut need_set_auto_run: Option<bool> = None;
        let mut need_rebuild_client = false;
        let mut need_reboot = false;
        {
            let mut old_setting = CONFIG.lock().unwrap();
            if old_setting.storage.pending_path != setting.storage.pending_path {
                need_reboot = true;
            }
            if old_setting.system.auto_launch != setting.system.auto_launch {
                need_set_auto_run = Some(setting.system.auto_launch);
            }
            if old_setting.network.use_proxy != setting.network.use_proxy
                || old_setting.network.proxy != setting.network.proxy
            {
                need_rebuild_client = true;
            }
            *old_setting = setting;
        }
        if let Some(auto_run) = need_set_auto_run {
            crate::utils::tauri::set_auto_launch(auto_run).map_err(|e| {
                log::error!("Failed to set auto run: {}", e);
                SettingError::SetAutoRunFailed(e)
            })?;
        }
        if need_reboot {
            crate::handler::stop(|| reboot_app());
        }
        if need_rebuild_client {
            crate::http::client::Client::rebuild();
        }
        Self::write_to_file()?;
        log::info!("Setting applied");

        Ok(())
    }

    pub fn get_pending_path() -> PathBuf {
        CONFIG.lock().unwrap().storage.pending_path.clone()
    }

    pub fn get_repository_path() -> PathBuf {
        CONFIG.lock().unwrap().storage.repository_path.clone()
    }

    pub fn get_use_openai() -> bool {
        CONFIG.lock().unwrap().scraper.use_openai
    }

    pub fn get_openai_key() -> Option<String> {
        let scraper = &CONFIG.lock().unwrap().scraper;
        if scraper.use_openai {
            Some(scraper.openai_key.clone())
        } else {
            None
        }
    }

    pub fn get_openai_domain() -> String {
        CONFIG.lock().unwrap().network.openai_domain.clone()
    }

    pub fn get_default_lang() -> String {
        CONFIG.lock().unwrap().scraper.default_lang.clone()
    }

    pub fn get_default_provider() -> ProviderKnown {
        CONFIG.lock().unwrap().scraper.default_provider.clone()
    }

    pub fn get_last_scan() -> u64 {
        CONFIG.lock().unwrap().storage.pending_path_last_scan
    }

    pub fn get_scan_interval() -> u64 {
        CONFIG.lock().unwrap().system.scan_interval
    }

    pub fn set_last_scan(time: u64) {
        let mut setting = CONFIG.lock().unwrap();
        setting.storage.pending_path_last_scan = time;
    }

    pub fn get_lang() -> String {
        CONFIG.lock().unwrap().ui.lang.clone()
    }

    pub fn get_slient_boot() -> bool {
        CONFIG.lock().unwrap().system.silent_start
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
    #[error("Failed to set auto run: {0}")]
    SetAutoRunFailed(String),
}

impl serde::Serialize for SettingError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
