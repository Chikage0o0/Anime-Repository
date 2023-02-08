use crate::model::setting::Setting;
use notify_debouncer_mini::{new_debouncer, notify::*};
use std::{path::PathBuf, thread, time::Duration};

pub fn watch_pending_path() {
    let path = Setting::get().storage.pending_path;
    thread::spawn(|| {
        watch(path);
    });
}

fn watch(path: PathBuf) {
    let (tx, rx) = std::sync::mpsc::channel();

    //  间隔30秒向通道发送信息
    let mut debouncer = new_debouncer(Duration::from_secs(120), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(&path, RecursiveMode::Recursive)
        .unwrap();

    // print all events, non returning
    for events in rx {
        if let Ok(_) = events {
            // Call Event
            println!("{:?}", events);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        watch_pending_path();
        thread::sleep(Duration::from_secs(600));
    }
}
