use std::thread;

use crate::model::setting::Setting;

mod pending_videos_list;
mod scan;
mod unrecognized_videos_list;

#[derive(Debug)]
enum Command {
    Stop,
    ScanPendingList,
    ScanUnrecognizedList,
    ScanPendingVideos,
}

lazy_static::lazy_static! {
    static ref STOP_TX: std::sync::mpsc::SyncSender<Command>= {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
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
                    Command::ScanPendingVideos => {
                        scan::process();
                    }
                }
            }
        });
        tx
    };

}

pub fn run() {
    log::info!("Start background thread");
    thread::spawn(|| loop {
        let tx = STOP_TX.clone();
        tx.send(Command::ScanPendingVideos).unwrap();
        tx.send(Command::ScanUnrecognizedList).unwrap();
        tx.send(Command::ScanPendingList).unwrap();
        thread::sleep(std::time::Duration::from_secs(Setting::get_scan_interval()));
    });
}

pub fn stop() {
    STOP_TX.send(Command::Stop).unwrap();
}
