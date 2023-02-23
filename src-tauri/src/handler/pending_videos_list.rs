use crate::{
    data::pending_videos::{delete_pending_video, get_pending_videos},
    utils::file,
};

pub fn process() {
    let list = get_pending_videos();

    list.iter().for_each(|(src_path, target_path)| {
        if src_path.exists() {
            if let Ok(time) = src_path.metadata().unwrap().modified() {
                // Ignore edited files within 5s
                if time.elapsed().unwrap().as_secs() < 5 {
                    return;
                }
            }
            if let Ok(_) = file::move_file(src_path, target_path) {
                delete_pending_video(src_path.to_path_buf());
            }
        } else {
            eprintln!("{} not exists", src_path.to_str().unwrap());
            delete_pending_video(src_path.to_path_buf());
        }
    });
}

// Run processes at 5 minute intervals
pub fn run() {
    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        process();
    });
}
