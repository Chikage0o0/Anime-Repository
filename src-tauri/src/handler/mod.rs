use crate::model::setting::Setting;
use once_cell::sync::Lazy;
use std::{path::PathBuf, thread};

mod pending_videos_list;
mod scan;
mod unrecognized_videos_list;

#[derive(Debug)]
pub enum Command {
    Stop,
    ScanPendingList,
    ScanUnrecognizedList,
    ScanPendingVideosFolder,
    InsertPendingVideos((PathBuf, PathBuf)),
}

static HANDLER_TX: Lazy<std::sync::mpsc::SyncSender<Command>> = Lazy::new(|| {
    let (tx, rx) = std::sync::mpsc::sync_channel(100);
    thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Stop => {
                    log::info!("Stop background thread");
                    break;
                }
                Command::ScanPendingList => {
                    pending_videos_list::process();
                }
                Command::ScanUnrecognizedList => {
                    unrecognized_videos_list::process();
                }
                Command::ScanPendingVideosFolder => {
                    scan::process();
                }
                Command::InsertPendingVideos((src_path, target_path)) => {
                    pending_videos_list::insert(src_path, target_path);
                }
            }
        }
    });
    tx
});

pub fn run() {
    log::info!("Start background thread");
    thread::spawn(|| loop {
        let tx = HANDLER_TX.clone();
        tx.send(Command::ScanPendingVideosFolder).unwrap();
        tx.send(Command::ScanUnrecognizedList).unwrap();
        tx.send(Command::ScanPendingList).unwrap();
        thread::sleep(std::time::Duration::from_secs(Setting::get_scan_interval()));
    });
}

pub fn stop() {
    HANDLER_TX.send(Command::Stop).unwrap();
}

pub fn get_handler_tx() -> std::sync::mpsc::SyncSender<Command> {
    HANDLER_TX.clone()
}
