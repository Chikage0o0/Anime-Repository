use crate::{
    data::pending_videos::get,
    model::setting::Setting,
    utils::{file, matcher::Matcher},
};
use notify_debouncer_mini::{new_debouncer, notify::*, DebouncedEventKind};
use std::{path::Path, thread, time::Duration};

pub fn scan_pending_path() {
    let path = Setting::get().storage.pending_path;
    thread::spawn(move || {
        log::info!("Watching pending path: {:?}", path);
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Can't create pending path");
        }
        if !path.is_dir() {
            panic!("Pending path is not a directory");
        }
        process(path);
    });
}

fn process<P: AsRef<Path>>(path: P) {
    let (tx, rx) = std::sync::mpsc::channel();

    //  间隔30秒向通道发送信息
    let mut debouncer = new_debouncer(
        Duration::from_secs(Setting::get().storage.pending_path_scan_interval),
        None,
        tx,
    )
    .unwrap();

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
                        && get(path).is_none()
                        && file::is_video(path))
                    .then(|| path)
                })
                .filter_map(|path| Matcher::matchers_video(path))
                .for_each(|f| {
                    crate::service::scribe::process(f.0, f.1, f.2, f.3)
                        .unwrap_or_else(|err| log::error!("{:?}", err))
                });
        }
    }
}
