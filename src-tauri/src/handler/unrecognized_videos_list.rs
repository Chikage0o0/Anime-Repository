use crate::{
    data::unrecognized_videos::get_all, service::unrecognized_videos::delete, utils::file::is_video,
};

pub(super) fn process() {
    let list = get_all();

    list.into_iter().for_each(|(path, _)| {
        if !path.exists() || !path.is_file() || path.is_symlink() || !is_video(&path) {
            delete(&path);
        }
    })
}
