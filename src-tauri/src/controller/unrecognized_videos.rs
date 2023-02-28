use std::path::PathBuf;

use crate::data::unrecognized_videos::{get_all, VideoData};

#[tauri::command]
pub fn get_unrecognized_videos_list() -> Vec<(PathBuf, VideoData)> {
    get_all()
}

#[tauri::command]
pub fn delete_unrecognized_video(path: PathBuf) {
    crate::service::unrecognized_videos::delete(path);
}

#[tauri::command]
pub fn insert_unrecognized_video(path: PathBuf, video_data: VideoData) {
    crate::service::unrecognized_videos::insert(path, video_data);
}
