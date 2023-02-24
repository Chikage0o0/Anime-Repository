use crate::{
    data::unrecognized_videos::{delete, get_all, VideoData},
    service::nfo::{movie, tvshow},
};

pub(super) fn process() {
    let list = get_all();

    list.into_iter()
        .for_each(|(path, video_data)| match video_data {
            VideoData::Movie(id, provider, lang) => {
                if id.is_some() && provider.is_some() && lang.is_some() {
                    let id = id.unwrap();
                    match movie::process(&id, provider.unwrap(), &lang.unwrap(), &path) {
                        Ok(_) => delete(&path),
                        Err(e) => {
                            log::error!(
                                "Movie id:{:?} provide:{:?} process error: {:?}",
                                &id,
                                provider.unwrap(),
                                e
                            )
                        }
                    }
                }
            }
            VideoData::TvShow(id, provider, title, lang, season, episode) => {
                if id.is_some()
                    && provider.is_some()
                    && title.is_some()
                    && lang.is_some()
                    && season.is_some()
                    && episode.is_some()
                {
                    let id = id.unwrap();
                    let title = title.unwrap();
                    match tvshow::process(
                        &id,
                        provider.unwrap(),
                        &title,
                        &lang.unwrap(),
                        season.unwrap(),
                        episode.unwrap(),
                        &path,
                    ) {
                        Ok(_) => delete(&path),
                        Err(e) => {
                            log::error!(
                                "Tvshow {} S{:02}E{:02} process error: {:?}",
                                title,
                                season.unwrap(),
                                episode.unwrap(),
                                e
                            );
                        }
                    }
                }
            }
            VideoData::None => (),
        })
}
