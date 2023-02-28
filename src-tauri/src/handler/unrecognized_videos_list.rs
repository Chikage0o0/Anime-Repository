use std::path::PathBuf;

use crate::{
    data::unrecognized_videos::get_all, model::setting::Setting,
    service::unrecognized_videos::delete, utils::file::is_video,
};
use lazy_static::lazy_static;

// get pending path
lazy_static! {
    static ref PENDING_PATH: PathBuf = Setting::get_pending_path();
}

pub(super) fn process() {
    let list = get_all();

    list.into_iter().for_each(|(path, _)| {
        if !path.exists()
            || !path.is_file()
            || path.is_symlink()
            || !is_video(&path)
            || !path.starts_with(&PENDING_PATH.as_path())
        {
            delete(&path);
        }
    })
}
