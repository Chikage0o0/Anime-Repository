use std::path::PathBuf;

use crate::{
    data::unrecognized_videos::{get_all, VideoData},
    model::nfo::ProviderKnown,
};

#[tauri::command]
pub fn get_unrecognized_videos_list() -> Vec<(PathBuf, VideoData)> {
    get_all()
}

#[tauri::command]
pub fn delete_unrecognized_video_info(path: PathBuf) -> Result<(), String> {
    crate::service::unrecognized_videos::delete(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_unrecognized_video_info(
    r#type: String,
    path: PathBuf,
    id: String,
    provider: ProviderKnown,
    lang: String,
    title: String,
    season: u64,
    episode: u64,
) -> Result<(), String> {
    if r#type == "movie" {
        crate::service::unrecognized_videos::insert(path, VideoData::Movie(id, provider, lang))
            .map_err(|e| e.to_string())?;
    } else if r#type == "tvshow" {
        crate::service::unrecognized_videos::insert(
            path,
            VideoData::Tvshow(id, provider, lang, title, season, episode),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}