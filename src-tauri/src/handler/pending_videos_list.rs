use crate::{
    data::pending_videos::{delete, get_all},
    utils::file,
};

pub(super) fn process() {
    //log::info!("Pending videos list process");
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
                use tauri::api::notification::Notification;

                let context = tauri::generate_context!();
                Notification::new(&context.config().tauri.bundle.identifier)
                    .title("Anime-Repository:New file are in!")
                    .body(format!(
                        "{}",
                        target_path.file_name().unwrap().to_str().unwrap()
                    ))
                    .show()
                    .unwrap();
            }
        } else {
            eprintln!("{} not exists", src_path.to_str().unwrap());
            delete(src_path.to_path_buf());
        }
    });
}
