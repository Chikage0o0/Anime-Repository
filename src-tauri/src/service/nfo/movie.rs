use super::*;
use crate::{
    data::pending_videos::insert,
    model::{
        nfo::{movie::Movie, Nfo, ProviderKnown},
        setting,
    },
};
use std::path::Path;

pub fn process<P: AsRef<Path>>(
    id: &str,
    provider: ProviderKnown,
    lang: &str,
    path: P,
) -> Result<(), MovieNfoServiceError> {
    let path = path.as_ref();
    log::info!("Processing {:?}", path);

    use tauri::async_runtime::block_on;

    let mut movie_nfo = Movie::new(&id, provider.clone().into());
    if let Err(e) = block_on(movie_nfo.update(lang)) {
        log::error!("Get movie nfo error: {:?}", e);
        return Err(MovieNfoServiceError::NetworkError(e));
    }

    let movie_title = movie_nfo.title.clone();
    let year = movie_nfo.get_year();
    let folder_name = if let Some(year) = year {
        format!("{} ({})", movie_title, year)
    } else {
        movie_title
    };

    let movie_folder_path = setting::Setting::get_repository_path()
        .join("Movies")
        .join(&folder_name);
    let movie_nfo_path = movie_folder_path.join("movie.nfo");
    let movie_path = movie_folder_path.join(format!(
        "{}.{}",
        &folder_name,
        path.extension().unwrap().to_str().unwrap()
    ));

    // 添加到待处理列表
    insert(&path, &movie_path.as_path());

    write_nfo(&movie_nfo_path, &movie_nfo)?;
    movie_nfo
        .get_thumb(&movie_folder_path)
        .iter()
        .for_each(|(path, thumb)| download_thumb(&path, &thumb).unwrap());

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum MovieNfoServiceError {
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    NfoCreateError(#[from] NfoServiceError),
    #[error(transparent)]
    SledError(#[from] crate::data::subscribe_rules::SubscribeDataError),
    #[error(transparent)]
    NetworkError(#[from] crate::model::nfo::NfoGetError),
}
