use std::time::{SystemTime, UNIX_EPOCH};
pub mod file;
pub mod matcher;
pub mod openai;
pub mod search;
pub mod tauri;

pub fn get_now_time() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()
}

#[derive(Clone, Copy)]
pub enum r#Type {
    Tvshow,
    Movie,
}
