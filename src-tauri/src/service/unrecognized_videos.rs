use std::path::Path;

use crate::data::unrecognized_videos::VideoData;

pub fn delete<P: AsRef<Path>>(path: P) -> Result<(), UnrecognizedVideosServiceError> {
    let path = path.as_ref().to_str().unwrap();
    crate::data::unrecognized_videos::delete(path)?;
    Ok(())
}

pub fn insert<P: AsRef<Path>>(
    path: P,
    video_data: VideoData,
) -> Result<(), UnrecognizedVideosServiceError> {
    let path = path.as_ref().to_str().unwrap();
    crate::data::unrecognized_videos::insert(path, video_data)?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum UnrecognizedVideosServiceError {
    #[error(transparent)]
    SledError(#[from] crate::data::unrecognized_videos::UnrecognizedVideosDataError),
}
