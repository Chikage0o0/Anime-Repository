use std::path::Path;

use crate::data::unrecognized_videos::VideoData;

pub fn delete<P: AsRef<Path>>(path: P) {
    let path = path.as_ref().to_str().unwrap();
    crate::data::unrecognized_videos::delete(path);
}

pub fn insert<P: AsRef<Path>>(path: P, video_data: VideoData) {
    let path = path.as_ref().to_str().unwrap();
    crate::data::unrecognized_videos::insert(path, video_data);
}
