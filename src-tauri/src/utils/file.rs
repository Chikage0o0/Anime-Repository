use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::data::pending_videos;
use fs_extra::file;

pub fn walk_file<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    WalkDir::new(path.as_ref())
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file())
        .map(|f| f.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}

pub fn move_file<P: AsRef<Path>>(from: P, to: P) -> Result<(), fs_extra::error::Error> {
    if let Some(p) = to.as_ref().parent() {
        std::fs::create_dir_all(p).unwrap();
    }
    log::info!("move file from {:?} to {:?}", from.as_ref(), to.as_ref());
    let option = file::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
    };
    file::move_file(from.as_ref(), to.as_ref(), &option)?;
    Ok(())
}

pub fn create_shortcut<P: AsRef<Path>>(src: P, target: P) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::symlink_file;
        symlink_file(src.as_ref(), target.as_ref())?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::symlink;
        symlink(src.as_ref(), target.as_ref())?;
    }
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::fs::symlink;
        symlink(src.as_ref(), target.as_ref())?;
    }
    log::info!(
        "create shortcut from {:?} to {:?}",
        src.as_ref(),
        target.as_ref()
    );
    Ok(())
}

pub fn is_video<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if !path.is_file() || path.is_symlink() || !path.exists() {
        return false;
    }
    let ext = path.extension().unwrap_or_default().to_str().unwrap();

    let is_video = match ext.to_lowercase().as_ref() {
        "mp4" | "mkv" | "avi" | "flv" | "wmv" | "mov" | "mpg" | "mpeg" | "m4v" | "webm" => true,
        _ => false,
    };
    is_video
}

// If move file failed, insert it database
pub fn move_file_with_queue(src_path: PathBuf, target_path: PathBuf) {
    if let Ok(_) = move_file(&src_path, &target_path) {
        pending_videos::delete(src_path.clone());
        create_shortcut(&target_path, &src_path)
            .unwrap_or_else(|err| log::error!("Create shortcut failed: {:?}", err));
        super::tauri::send_storage_notification(target_path.file_name().unwrap().to_str().unwrap());
    } else {
        crate::data::pending_videos::insert(src_path, target_path);
    }
}
