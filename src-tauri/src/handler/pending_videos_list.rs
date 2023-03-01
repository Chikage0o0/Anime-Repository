use std::path::PathBuf;

use crate::{
    data::pending_videos::{delete, get_all},
    model::setting::Setting,
    utils::file,
};

pub fn process() {
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
                send_notification(target_path.file_name().unwrap().to_str().unwrap());
            }
        } else {
            eprintln!("{} not exists", src_path.to_str().unwrap());
            delete(src_path.to_path_buf());
        }
    });
}

// If move file failed, insert it database
pub fn insert(src_path: PathBuf, target_path: PathBuf) {
    if let Ok(_) = file::move_file(&src_path, &target_path) {
        delete(src_path.clone());
        file::create_shortcut(&target_path, &src_path)
            .unwrap_or_else(|err| log::error!("Create shortcut failed: {:?}", err));
        send_notification(target_path.file_name().unwrap().to_str().unwrap());
    } else {
        crate::data::pending_videos::insert(src_path, target_path);
    }
}

fn send_notification(file_name: &str) {
    use tauri::api::notification::Notification;

    let mut title = String::from("Anime-Repository:New file are in!");

    if Setting::get_lang() == "zh_CN" {
        title = String::from("Anime-Repository:新文件已经入库！");
    }

    let context = tauri::generate_context!();
    Notification::new(&context.config().tauri.bundle.identifier)
        .title(title)
        .body(format!("{}", file_name))
        .show()
        .unwrap();
}
