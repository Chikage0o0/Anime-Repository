use crate::{
    data::{
        pending_videos,
        unrecognized_videos::{self, UnrecognizedVideosDataError},
    },
    model::setting::Setting,
    utils::{self, file, matcher::Matcher},
};
use notify_debouncer_mini::{new_debouncer, notify::*, DebouncedEventKind};
use std::{
    path::Path,
    time::{Duration, UNIX_EPOCH},
};

pub(super) fn process<P: AsRef<Path>>(path: P) {
    let (tx, rx) = std::sync::mpsc::channel();

    //  间隔30秒向通道发送信息
    let mut debouncer =
        new_debouncer(Duration::from_secs(Setting::get_scan_interval()), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    for events in rx {
        if let Ok(e) = events {
            // Call Event
            e.iter()
                .filter_map(|e| {
                    let path = &e.path;
                    // 排除非视频文件以及已经加入处理队列的文件
                    (e.kind == DebouncedEventKind::Any
                        && pending_videos::get(path).is_none()
                        && matches!(
                            unrecognized_videos::get(path),
                            Err(UnrecognizedVideosDataError::KeyNotFound(_))
                        )
                        && file::is_video(path))
                    .then(|| path)
                })
                // 对视频文件进行匹配
                .for_each(|path| match Matcher::matchers_video(path) {
                    Some((key, path, episode)) => {
                        log::info!("Found Subscribe video: {:?}", path);
                        crate::service::subscribe::process(&key, path, episode)
                            .unwrap_or_else(|err| log::error!("{:?}", err))
                    }
                    // 未匹配到的视频文件
                    None => {
                        log::info!("Found Unrecognized video: {:?}", path);
                        if let Err(e) = crate::service::unrecognized_videos::insert(
                            path,
                            unrecognized_videos::VideoData::Undefined,
                        ) {
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
    }
}

pub fn boot_scan<P: AsRef<Path>>(path: P) {
    file::walk_file(path.as_ref())
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
                && path.metadata().unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs() > Setting::get_last_scan())
            .then(|| path)
        })
        // 对视频文件进行匹配
        .for_each(|path| match Matcher::matchers_video(&path) {
            Some((key, path, episode)) => {
                log::info!("Found Subscribe video: {:?}", path);
                crate::service::subscribe::process(&key, path, episode)
                    .unwrap_or_else(|err| log::error!("{:?}", err))
            }
            // 未匹配到的视频文件
            None => {
                log::info!("Found Unrecognized video: {:?}", path);
                if let Err(e) = crate::service::unrecognized_videos::insert(
                    path,
                    unrecognized_videos::VideoData::Undefined,
                ) {
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
