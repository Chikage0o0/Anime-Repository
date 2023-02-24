use crate::{
    data::pending_videos::{delete, get_all},
    model::setting::Setting,
    utils::file,
};

pub fn process() {
    let list = get_all();

    list.iter().for_each(|(src_path, target_path)| {
        if src_path.is_file() && !src_path.is_symlink() {
            if let Ok(time) = src_path.metadata().unwrap().modified() {
                // Ignore edited files within 5s
                if time.elapsed().unwrap().as_secs() < 5 {
                    return;
                }
            }
            if let Ok(_) = file::move_file(src_path, target_path) {
                delete(src_path.to_path_buf());
                file::create_shortcut(&target_path, &src_path)
                    .unwrap_or_else(|err| log::error!("Create shortcut failed: {:?}", err));
            }
        } else {
            eprintln!("{} not exists", src_path.to_str().unwrap());
            delete(src_path.to_path_buf());
        }
    });
}

// Run processes at 5 minute intervals
pub fn run() {
    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(
            Setting::get().storage.pending_path_scan_interval,
        ));
        process();
    });
}
