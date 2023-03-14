use crate::{
    data::unrecognized_videos::get_all, model::setting::Setting,
    service::unrecognized_videos::delete, utils::file::is_video,
};

pub fn process() {
    log::debug!("Start to process Unrecognized Video database");
    let list = get_all();

    list.into_iter().for_each(|(path, _)| {
        if !path.exists() || !path.is_file() || path.is_symlink() || !is_video(&path) {
            if let Err(e) = delete(&path) {
                log::error!(
                    "Delete {:?} from Unrecognized Video database failed: {:?}",
                    path,
                    e
                );
            } else {
                log::debug!("Delete {:?} from Unrecognized Video database", path);
            }
        }
    })
}
