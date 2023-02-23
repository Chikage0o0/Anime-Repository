use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref DB: sled::Db = sled::open("config/pending_videos").unwrap();
}

pub fn get_pending_videos() -> Vec<(PathBuf, PathBuf)> {
    DB.iter()
        .filter_map(|f| {
            if let Some(tmp) = f.ok() {
                let src_path: String = String::from_utf8(tmp.0.to_vec()).unwrap();
                let target_path: String = String::from_utf8(tmp.1.to_vec()).unwrap();
                Some((PathBuf::from(src_path), PathBuf::from(target_path)))
            } else {
                None
            }
        })
        .collect::<Vec<(PathBuf, PathBuf)>>()
}

pub fn get_pending_video<P: AsRef<Path>>(path: P) -> Option<(PathBuf, PathBuf)> {
    if let Some(value) = DB.get(path.as_ref().to_str().unwrap()).unwrap() {
        let target_path: String = String::from_utf8(value.to_vec()).unwrap();
        Some((PathBuf::from(path.as_ref()), PathBuf::from(target_path)))
    } else {
        None
    }
}

pub fn insert_pending_video<P: AsRef<Path>>(src_path: P, target_path: P) {
    DB.insert(
        src_path.as_ref().to_str().unwrap(),
        target_path.as_ref().to_str().unwrap(),
    )
    .unwrap();
}

pub fn delete_pending_video(path: PathBuf) {
    DB.remove(path.to_str().unwrap()).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = std::fs::remove_dir_all("config/pending_videos");
        let _ = std::fs::create_dir_all("config/pending_videos");
    }

    #[test]
    fn test_insert() {
        init();
        let src_path = PathBuf::from("test.mp4");
        let target_path = PathBuf::from("test.mp4");
        insert_pending_video(src_path, target_path);
        let list = get_pending_videos();
        assert_eq!(list.len(), 1);
        assert!(list[0].0.to_str().unwrap().contains("test.mp4"));
        assert!(list[0].1.to_str().unwrap().contains("test.mp4"));
        let _ = std::fs::remove_dir_all("config/pending_videos");
    }
}
