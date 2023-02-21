use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn walk_file<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.file_type().is_file())
        .map(|f| f.path().to_path_buf())
        .collect::<Vec<PathBuf>>()
}

pub fn move_file<P: AsRef<Path>>(from: P, to: P) -> Result<(), std::io::Error> {
    if let Some(p) = to.as_ref().parent() {
        std::fs::create_dir_all(p).unwrap();
    }
    std::fs::rename(from, to)?;
    Ok(())
}
