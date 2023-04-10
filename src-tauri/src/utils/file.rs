use std::{
    fs,
    path::{Path, PathBuf},
};
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

pub fn move_video_file<P: AsRef<Path>>(from: P, to: P) -> Result<(), fs_extra::error::Error> {
    if let Some(p) = to.as_ref().parent() {
        std::fs::create_dir_all(p).unwrap();
    }
    let option = file::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
    };
    file::move_file(from.as_ref(), to.as_ref(), &option)?;
    log::info!("move file from {:?} to {:?}", from.as_ref(), to.as_ref());
    find_same_name_sub(from, to).iter().for_each(|(from, to)| {
        let result = file::move_file(from, to, &option);
        match result {
            Ok(_) => log::info!("move file from {:?} to {:?}", from, to),
            Err(err) => log::error!("move file from {:?} to {:?} failed: {:?}", from, to, err),
        }
    });
    Ok(())
}

fn find_same_name_sub<P: AsRef<Path>>(
    video_src_path: P,
    video_target_path: P,
) -> Vec<(PathBuf, PathBuf)> {
    let mut list = Vec::new();
    let video_src_path = video_src_path.as_ref();
    let forder = video_src_path.parent().unwrap();
    let video_src_stem = video_src_path.file_stem().unwrap().to_str().unwrap();
    let forder_file_list = fs::read_dir(forder).unwrap();
    forder_file_list
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().unwrap().is_file())
        .filter(|f| {
            matches!(
                f.path()
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
                "srt" | "ass" | "ssa" | "smi" | "vtt" | "idx" | "sub"
            )
        })
        .filter_map(|f| {
            let sub_path = f.path();
            let sub_stem = sub_path.file_stem().unwrap().to_str().unwrap();
            if sub_stem.starts_with(video_src_stem) {
                let sub_target_path =
                    map_sub_target_path(video_src_stem, &video_target_path.as_ref(), &sub_path);
                Some((sub_path, sub_target_path))
            } else {
                None
            }
        })
        .for_each(|f| {
            list.push(f);
        });

    list
}

fn map_sub_target_path(video_src_stem: &str, video_target_path: &Path, sub_path: &Path) -> PathBuf {
    let sub_stem = sub_path.file_stem().unwrap().to_str().unwrap();
    let sub_ext = sub_path.extension().unwrap().to_str().unwrap();
    let video_target_stem = video_target_path.file_stem().unwrap().to_str().unwrap();
    let sub_stem = sub_stem.replace(video_src_stem, video_target_stem);
    video_target_path.with_file_name(format!("{}.{}", sub_stem, sub_ext))
}

pub fn create_shortcut<P: AsRef<Path>>(src: P, target: P) -> Result<(), std::io::Error> {
    //if path is relative, convert to absolute path
    let src = if src.as_ref().is_relative() {
        std::fs::canonicalize(src.as_ref())?
    } else {
        src.as_ref().to_path_buf()
    };
    let target = if target.as_ref().is_relative() {
        std::fs::canonicalize(target.as_ref())?
    } else {
        target.as_ref().to_path_buf()
    };
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::symlink_file;
        symlink_file(&src, &target)?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::symlink;
        symlink(&src, &target)?;
    }
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::fs::symlink;
        symlink(&src, &target)?;
    }
    log::info!("create shortcut from {:?} to {:?}", &src, &target);
    Ok(())
}

pub fn is_video<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    if !path.exists() || !path.is_file() || path.is_symlink() {
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
pub fn move_video_file_with_queue(src_path: PathBuf, target_path: PathBuf) {
    if let Ok(_) = move_video_file(&src_path, &target_path) {
        pending_videos::delete(src_path.clone());
        create_shortcut(&target_path, &src_path)
            .unwrap_or_else(|err| log::error!("Create shortcut failed: {:?}", err));
        crate::controller::send_storage_notification(
            target_path.file_name().unwrap().to_str().unwrap(),
        );
    } else {
        crate::data::pending_videos::insert(src_path, target_path);
    }
}
