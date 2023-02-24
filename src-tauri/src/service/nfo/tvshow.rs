use super::*;
use crate::{
    data::{pending_videos::insert, scribe::Key},
    model::{
        nfo::{episode::Episode, public::Nfo, tvshow::Tvshow},
        setting,
    },
};
use std::{fmt::Debug, path::PathBuf};

// TODO: 异步
pub fn process(
    key: Key,
    path: PathBuf,
    season: u64,
    episode: u64,
) -> Result<(), TvshowNfoServiceError> {
    log::info!("Processing {:?}", path);
    let value = key.get().unwrap();

    use tauri::async_runtime::block_on;

    let tvshow_title = value.title.clone();
    let tvshow_path = setting::Setting::get()
        .storage
        .repository_path
        .join(&tvshow_title);
    let tvshow_nfo_path = tvshow_path.join("tvshow.nfo");

    let mut tvshow_nfo: Tvshow;
    if tvshow_nfo_path.exists() {
        tvshow_nfo = read_nfo(&tvshow_nfo_path)?;
    } else {
        tvshow_nfo = Tvshow::new(&key.id, key.provider.into());
    }
    // 从网络获取信息
    block_on(tvshow_nfo.update(&value.lang)).unwrap();

    write_nfo(&tvshow_nfo_path, &tvshow_nfo).unwrap();
    tvshow_nfo
        .get_thumb(&tvshow_path)
        .iter()
        .for_each(|(path, thumb)| download_thumb(&path, &thumb).unwrap());

    let mut episode_nfo = Episode::new(&key.id, key.provider.into());
    block_on(episode_nfo.update(&value.lang, season, episode));

    let episode_title = episode_nfo.title.clone();

    let episode_folder_path = tvshow_path.join(if season == 0 {
        "Specials".to_string()
    } else {
        format!("Season {:02}", season)
    });
    let episode_nfo_path = episode_folder_path.join(format!(
        "{} - S{:02}E{:02} - {}.nfo",
        &tvshow_title, season, episode, &episode_title
    ));
    let episode_path = episode_folder_path.join(format!(
        "{} - S{:02}E{:02} - {}.{}",
        &tvshow_title,
        season,
        episode,
        &episode_title,
        path.extension().unwrap().to_str().unwrap()
    ));

    // 添加到待处理列表
    insert(&path, &episode_path);

    write_nfo(&episode_nfo_path, &episode_nfo).unwrap();
    if let Some(thumb) = episode_nfo.get_thumb() {
        download_thumb(
            episode_folder_path.join(format!(
                "{} - S{:02}E{:02} - {}-thumb.jpg",
                &tvshow_title, season, episode, &episode_title
            )),
            &thumb,
        )?;
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum TvshowNfoServiceError {
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    NfoCreateError(#[from] NfoServiceError),
    #[error(transparent)]
    SledError(#[from] crate::data::scribe::ScribeDataError),
}
