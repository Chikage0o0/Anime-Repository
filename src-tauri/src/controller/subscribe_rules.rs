use serde::Serialize;

use crate::{
    data::subscribe_rules::{list, Key, Value},
    model::nfo::ProviderKnown,
    service::subscribe,
};

#[derive(Serialize)]
pub struct SubscribeRule {
    id: String,
    provider: ProviderKnown,
    title: String,
    tvshow_regex: String,
    season: u64,
    episode_offset: i64,
    episode_position: u8,
    episode_regex: String,
    lang: String,
}

impl From<(Key, Value)> for SubscribeRule {
    fn from(value: (Key, Value)) -> Self {
        Self {
            id: value.0.id,
            provider: value.0.provider,
            title: value.1.title,
            tvshow_regex: value.1.tvshow_regex,
            season: value.1.season,
            episode_offset: value.1.episode_offset,
            episode_position: value.1.episode_position + 1,
            episode_regex: value.1.episode_regex,
            lang: value.1.lang,
        }
    }
}

#[tauri::command]
pub fn get_subscribe_rules() -> Vec<SubscribeRule> {
    list().into_iter().map(|x| SubscribeRule::from(x)).collect()
}

#[tauri::command]
pub fn delete_subscribe_rule(id: String, provider: ProviderKnown) -> Result<(), String> {
    subscribe::remove(Key { id, provider }).map_err(|e| e.to_string())
}

// USE TMDB API TO GET TVSHOW TITLE
#[tauri::command]
pub async fn get_tvshow_title(
    id: &str,
    provider: ProviderKnown,
    lang: &str,
) -> Result<String, String> {
    subscribe::get_tvshow_title(id, provider, lang)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_subscribe_rule(id: String, provider: ProviderKnown) -> Result<SubscribeRule, String> {
    let key = Key { id, provider };
    let value = key.get().map_err(|e| e.to_string())?;
    Ok(SubscribeRule::from((key, value)))
}

#[tauri::command]
pub async fn insert_subscribe_rule(
    id: String,
    provider: ProviderKnown,
    title: String,
    tvshow_regex: String,
    season: u64,
    episode_offset: i64,
    episode_position: u8,
    episode_regex: String,
    lang: String,
) -> Result<(), String> {
    subscribe::insert((
        Key { id, provider },
        Value {
            title,
            tvshow_regex,
            season,
            episode_offset,
            episode_position: episode_position - 1,
            episode_regex,
            lang,
        },
    ))
    .await
    .map_err(|e| e.to_string())
}
