#![allow(dead_code, unused_variables)]
mod pending_videos_list;
mod scan;
mod unrecognized_videos_list;

use crate::model::setting::Setting;
use std::thread;

pub fn run() {
    thread::spawn(|| {
        let path = Setting::get_pending_path();
        log::info!("Watching pending path: {:?}", path);
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Can't create pending path");
        }
        if !path.is_dir() {
            panic!("Pending path is not a directory");
        }
        scan::process(path);
    });

    thread::spawn(|| loop {
        unrecognized_videos_list::process();
        pending_videos_list::process();
        thread::sleep(std::time::Duration::from_secs(Setting::get_scan_interval()));
    });
}
