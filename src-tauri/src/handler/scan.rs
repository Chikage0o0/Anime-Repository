use crate::{
    data::{
        pending_videos,
        unrecognized_videos::{self, UnrecognizedVideosDataError},
    },
    model::setting::Setting,
    utils::{self, file, matcher},
};
use notify_debouncer_mini::{notify::Error, DebouncedEvent};
use std::{path::Path, sync::mpsc::Receiver, time::UNIX_EPOCH};
use tauri::async_runtime::block_on;

pub fn process(wrx: &Receiver<Result<Vec<DebouncedEvent>, Vec<Error>>>) {

    log::debug!("Processing pending videos folder");
    while let Ok(events_result) = wrx.try_recv() {
        match events_result {
            Ok(events) => {
                for event in events {
                    let file_path = event.path;
                    if filter_file(&file_path) {
                        match_file(&file_path);
                    }else  if file_path.is_dir() && !file_path.is_symlink() && file_path.exists(){
                        file::walk_file(file_path.clone())
                            .iter()
                            .filter(|file_path| filter_file(file_path))
                            .for_each(|file_path| match_file(file_path));
                    }
                }
            }
            Err(errors) => {
                for error in errors {
                    log::error!("Watch error: {}", error);
                }
            }
        }
    }
    Setting::set_last_scan(utils::get_now_time());
}

// 第一次启动时使用轮询的方式扫描
pub fn first_boot() {
    let pending_path=Setting::get_pending_path();
    log::debug!("BOOT:Scan pending videos folder");
    if !pending_path.exists() {
        std::fs::create_dir_all(&pending_path).expect("Can't create pending path");
    }
    if !pending_path.is_dir() {
        panic!("Pending path is not a directory");
    }
    file::walk_file(pending_path.clone())
        .iter()
        .filter_map(|file_path| {
        // 排除非视频文件以及已经加入处理队列的文件
        (filter_file(file_path)                
        // 排除上次扫描之前的文件
            && file_path.metadata().ok()
            .and_then(|m|m.accessed().ok())
            .and_then(|f|f.duration_since(UNIX_EPOCH).ok())
            .and_then(|t|Some(t.as_secs()> Setting::get_last_scan())).unwrap_or_default())
            .then(|| file_path)
        })
        // 对视频文件进行匹配
        .for_each(|file_path| match_file(file_path));
    Setting::set_last_scan(utils::get_now_time());
}

fn filter_file<P: AsRef<Path>>(file_path: P) -> bool {
    file::is_video(&file_path)
        && pending_videos::get(&file_path).is_none()
        && matches!(
            unrecognized_videos::get(&file_path),
            Err(UnrecognizedVideosDataError::KeyNotFound(_))
        )
}

fn match_file(file_path: &Path) {
    match matcher::Matcher::matchers_video(&file_path) {
        Some((key, file_path, episode)) => {
            log::info!("Found Subscribe video: {:?}", file_path);
            block_on(crate::service::subscribe::process(&key, file_path, episode))
                .unwrap_or_else(|err| log::error!("{:?}", err))
        }
        // 未匹配到的视频文件
        None => {
            match block_on(matcher::internal_matcher(&file_path)){
                Ok(_) => {
                    log::info!("Use Internal Matcher match the file: {:?}", file_path);
                    return;
                }
                Err(e) => {
                    log::warn!("Use internal matcher match the {:?} failed: {:?}",file_path, e);
                }
            }

            if Setting::get_use_openai() {
                // 未匹配的文件，且在根目录，则使用 OpenAI 进行匹配
                if file_path
                    .parent()
                    .and_then(|f| (f == &Setting::get_pending_path()).then(|| ()))
                    .is_some()
                {
                    let result = block_on(utils::openai::process(&file_path));
                    match result {
                        Ok(_)=>{
                            log::info!("Use OpenAI match the file: {:?}", file_path);
                        return;
                        }
                        Err(e) => {
                            log::warn!("Use OpenAI match the {:?} failed: {:?}", file_path, e);
                        }
                        
                    }
                }
            }

            log::info!("Found Unrecognized video: {:?}", file_path);
            if let Err(e) = block_on(crate::service::unrecognized_videos::insert(
                file_path,
                unrecognized_videos::VideoData::Undefined,
            )) {
                log::error!(
                    "Insert {:?} to Unrecognized Video database failed: {:?}",
                    file_path,
                    e
                );
            }
        }
    }
}
