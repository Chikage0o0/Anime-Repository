#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};
pub mod file;
pub mod matcher;

pub fn get_now_time() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()
}
