use crate::{
    data::pending_videos::{delete, get_all},
    utils::file,
};

pub fn process() {
    log::debug!("Pending videos list process");
    let list = get_all();

    list.iter().for_each(|(src_path, target_path)| {
        if src_path.is_file() && !src_path.is_symlink() {
            if let Ok(_) = file::move_video_file(src_path, target_path) {
                delete(src_path.to_path_buf());
                file::create_shortcut(&target_path, &src_path)
                    .unwrap_or_else(|err| log::error!("Create shortcut failed: {:?}", err));
                crate::controller::send_storage_notification(
                    target_path
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default(),
                );
            }
        } else {
            eprintln!("{} not exists", src_path.to_str().unwrap_or_default());
            delete(src_path.to_path_buf());
        }
    });
}
