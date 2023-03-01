use crate::{
    data::{
        pending_videos,
        unrecognized_videos::{self, UnrecognizedVideosDataError},
    },
    model::setting::Setting,
    utils::{self, file, matcher::Matcher},
};
use std::time::UNIX_EPOCH;
use tauri::async_runtime::block_on;

pub fn process() {
    let path = Setting::get_pending_path();
    log::info!("Watching pending path: {:?}", &path);
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Can't create pending path");
    }
    if !path.is_dir() {
        panic!("Pending path is not a directory");
    }
    file::walk_file(path)
        .iter()
        .filter_map(|path| {
            // 排除非视频文件以及已经加入处理队列的文件
            (file::is_video(&path)
                && pending_videos::get(&path).is_none()
                && matches!(
                    unrecognized_videos::get(&path),
                    Err(UnrecognizedVideosDataError::KeyNotFound(_))
                )
                // 排除上次扫描之前的文件
                && path.metadata().unwrap().accessed().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs() > Setting::get_last_scan())
            .then(|| path)
        })
        // 对视频文件进行匹配
        .for_each(|path| match Matcher::matchers_video(&path) {
            Some((key, path, episode)) => {
                log::info!("Found Subscribe video: {:?}", path);
              block_on( crate::service::subscribe::process(&key, path, episode)) 
                    .unwrap_or_else(|err| log::error!("{:?}", err))
            }
            // 未匹配到的视频文件
            None => {
                log::info!("Found Unrecognized video: {:?}", path);
                if let Err(e) = block_on(crate::service::unrecognized_videos::insert(
                    path,
                    unrecognized_videos::VideoData::Undefined,
                )) {
                    log::error!(
                        "Insert {:?} to Unrecognized Video database failed: {:?}",
                        path,
                        e
                    );
                }
            }
        });
    Setting::set_last_scan(utils::get_now_time());
}
