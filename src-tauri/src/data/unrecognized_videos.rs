use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::model::nfo::ProviderKnown;

lazy_static! {
    static ref DB: sled::Db = sled::open("config/unrecognized_videos").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// The type of video that is unrecognized
/// Movie: (id, provider, lang)
/// TvShow: (id, provider, lang,title, season, episode)
pub enum VideoData {
    Movie(Option<String>, Option<ProviderKnown>, Option<String>),
    TvShow(
        Option<String>,
        Option<ProviderKnown>,
        Option<String>,
        Option<String>,
        Option<u64>,
        Option<u64>,
    ),
    None,
}

impl VideoData {
    pub fn new() -> Self {
        Self::None
    }
}

pub fn get_all() -> Vec<(PathBuf, VideoData)> {
    DB.iter()
        .filter_map(|f| {
            if let Some(tmp) = f.ok() {
                let path = String::from_utf8(tmp.0.to_vec()).unwrap();
                Some((
                    PathBuf::from(path),
                    bincode::deserialize(&tmp.1.to_vec()[..]).unwrap(),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<(PathBuf, VideoData)>>()
}

pub fn get<P: AsRef<Path>>(path: P) -> Option<VideoData> {
    if let Some(value) = DB.get(path.as_ref().to_str().unwrap()).unwrap() {
        let path = String::from_utf8(value.to_vec()).unwrap();
        Some(bincode::deserialize(&value.to_vec()[..]).unwrap())
    } else {
        None
    }
}

pub fn insert<P: AsRef<Path>>(path: P, video_data: VideoData) {
    DB.insert(
        path.as_ref().to_str().unwrap(),
        bincode::serialize(&video_data).unwrap(),
    )
    .unwrap();
}

pub fn delete<P: AsRef<Path>>(path: P) {
    DB.remove(path.as_ref().to_str().unwrap()).unwrap();
}

pub fn delete_all() {
    DB.clear().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = std::fs::remove_dir_all("config/unrecognized_videos");
        let _ = std::fs::create_dir_all("config/unrecognized_videos");
    }

    #[test]
    fn test_insert() {
        init();
        let path = PathBuf::from("test.mp4");
        let video_data = VideoData::new();
        insert(path, video_data);
        let list = get_all();
        assert_eq!(list.len(), 1);
        assert!(list[0].0.to_str().unwrap().contains("test.mp4"));
    }
}
