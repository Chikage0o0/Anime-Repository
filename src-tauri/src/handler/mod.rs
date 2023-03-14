use crate::model::setting::Setting;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use once_cell::sync::Lazy;
use std::{
    sync::mpsc::{self},
    thread,
    time::Duration,
};

mod pending_videos_list;
mod scan;
mod unrecognized_videos_list;

pub enum Command {
    ScanPendingList,
    ScanUnrecognizedList,
    ScanPendingVideosFolder,
}

static HANDLER_TX: Lazy<mpsc::SyncSender<Command>> = Lazy::new(|| {
    let (tx, rx) = mpsc::sync_channel(100);

    // Watcher Pending Videos Folder
    thread::spawn(move || {
        scan::first_boot();
        let (wtx, wrx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_secs(2), None, wtx).unwrap();
        debouncer
            .watcher()
            .watch(&Setting::get_pending_path(), RecursiveMode::Recursive)
            .unwrap();
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::ScanPendingList => {
                    pending_videos_list::process();
                }
                Command::ScanUnrecognizedList => {
                    unrecognized_videos_list::process();
                }
                Command::ScanPendingVideosFolder => {
                    scan::process(&wrx);
                }
            }
        }
    });
    tx
});

pub fn run() {
    log::info!("Start background thread");
    let tx = HANDLER_TX.clone();
    thread::spawn(move || loop {
        tx.send(Command::ScanPendingVideosFolder).unwrap();
        tx.send(Command::ScanUnrecognizedList).unwrap();
        tx.send(Command::ScanPendingList).unwrap();
        thread::sleep(std::time::Duration::from_secs(Setting::get_scan_interval()));
    });
}

pub fn get_handler_tx() -> std::sync::mpsc::SyncSender<Command> {
    HANDLER_TX.clone()
}
