use crate::{
    data::{pending_videos, unrecognized_videos::VideoData},
    model::setting::Setting,
};
use std::path::Path;

pub fn delete<P: AsRef<Path>>(path: P) -> Result<(), UnrecognizedVideosServiceError> {
    let path = path.as_ref();
    crate::data::unrecognized_videos::delete(path.to_str().unwrap())?;
    // 如果存在文件，同时删除文件
    if path.exists()
        && path.is_file()
        && pending_videos::get(&path).is_none()
        && path.starts_with(Setting::get_pending_path().as_path())
    {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub async fn insert<P: AsRef<Path>>(
    path: P,
    video_data: VideoData,
) -> Result<(), UnrecognizedVideosServiceError> {
    let path = path.as_ref().to_str().unwrap();
    match video_data {
        VideoData::Movie(id, provider, lang) => {
            crate::service::nfo::movie::process(&id, provider, &lang, path).await?;
            delete(path)?;
        }
        VideoData::Tvshow(id, provider, lang, title, season, episode) => {
            crate::service::nfo::tvshow::process(
                &id, provider, &title, &lang, season, episode, path,
            )
            .await?;
            delete(path)?;
        }
        VideoData::Undefined => {
            crate::data::unrecognized_videos::insert(path, video_data)?;
        }
    };

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum UnrecognizedVideosServiceError {
    #[error(transparent)]
    SledError(#[from] crate::data::unrecognized_videos::UnrecognizedVideosDataError),
    #[error(transparent)]
    TvshowNfoCreateError(#[from] crate::service::nfo::tvshow::TvshowNfoServiceError),
    #[error(transparent)]
    MovieNfoCreateError(#[from] crate::service::nfo::movie::MovieNfoServiceError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
