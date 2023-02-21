use crate::{model::setting::Setting, utils::matcher::Matcher};
use notify_debouncer_mini::{new_debouncer, notify::*};
use std::{path::Path, thread, time::Duration};

pub fn watch_pending_path() {
    let path = Setting::get().storage.pending_path;
    thread::spawn(move || {
        watch(path.as_path());
    });
}

fn watch<P: AsRef<Path>>(path: P) {
    let (tx, rx) = std::sync::mpsc::channel();

    //  间隔30秒向通道发送信息
    let mut debouncer = new_debouncer(Duration::from_secs(120), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)
        .unwrap();

    // print all events, non returning
    for events in rx {
        if let Ok(e) = events {
            // Call Event
            e.iter()
                .filter_map(|e| e.path.is_file().then(|| &e.path))
                .filter_map(|path| Matcher::matchers_video(path))
                .for_each(|f| {
                    crate::service::scribe::process(f.0, f.1, f.2, f.3)
                        .unwrap_or_else(|err| eprint!("{:?}", err))
                });
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
