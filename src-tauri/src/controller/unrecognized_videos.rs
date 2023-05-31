use std::path::PathBuf;

use crate::{
    data::unrecognized_videos::{get_all, VideoData},
    handler::get_handler_tx,
    model::nfo::ProviderKnown,
};

#[tauri::command]
pub fn get_unrecognized_videos_list() -> Vec<(PathBuf, VideoData)> {
    get_all()
}

#[tauri::command]
pub fn refresh_unrecognized_videos_list() -> Result<(), String> {
    get_handler_tx()
        .send(crate::handler::Command::ScanPendingVideosFolder)
        .map_err(|e| e.to_string())?;
    get_handler_tx()
        .send(crate::handler::Command::ScanUnrecognizedList)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_unrecognized_video_info(path: PathBuf) -> Result<(), String> {
    crate::service::unrecognized_videos::delete(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_unrecognized_video_info(
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
            .await
            .map_err(|e| e.to_string())?;
    } else if r#type == "tvshow" {
        crate::service::unrecognized_videos::insert(
            path,
            VideoData::Tvshow(id, provider, lang, title, season, episode),
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
