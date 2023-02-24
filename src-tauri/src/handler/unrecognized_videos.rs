use crate::data::unrecognized_videos::{get_all, VideoData};

pub fn process() {
    let list = get_all();

    list.iter().for_each(|(path, video_data)| match video_data {
        VideoData::Movie(id, provider) => {
            if id.is_some() && provider.is_some() {
                todo!()
            }
        }
        VideoData::TvShow(id, provider, season, episode) => {
            if id.is_some() && provider.is_some() && season.is_some() && episode.is_some() {
                todo!()
            }
        }
        VideoData::None => (),
    })
}
