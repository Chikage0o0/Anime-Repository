use super::*;
use crate::{
    model::{
        nfo::{movie::Movie, Nfo, ProviderKnown},
        setting,
    },
    utils::file,
};
use std::path::Path;

pub async fn process<P: AsRef<Path>>(
    id: &str,
    provider: ProviderKnown,
    lang: &str,
    path: P,
) -> Result<(), MovieNfoServiceError> {
    let path = path.as_ref();
    log::info!("Processing {:?}", path);

    let mut movie_nfo = Movie::new(&id, provider.clone().into());
    if let Err(e) = movie_nfo.update(lang).await {
        log::error!("Get movie nfo error: {:?}", e);
        return Err(MovieNfoServiceError::NetworkError(e));
    }

    let movie_title = movie_nfo.title.clone();
    let year = movie_nfo.get_year();
    let folder_name = if let Some(year) = year {
        format!("{} ({})", make_vaild_pathname(&movie_title), year)
    } else {
        make_vaild_pathname(&movie_title)
    };

    let movie_folder_path = setting::Setting::get_movie_repository_path().join(&folder_name);
    let movie_nfo_path = movie_folder_path.join("movie.nfo");
    let movie_path = movie_folder_path.join(format!(
        "{}.{}",
        &folder_name,
        path.extension().unwrap().to_str().unwrap()
    ));

    file::move_video_file_with_queue(path.to_path_buf(), movie_path);

    write_nfo(&movie_nfo_path, &movie_nfo)?;
    // multi-thread download
    let mut donwload_pool = Vec::new();

    for (path, thumb) in movie_nfo.get_thumb(&movie_folder_path) {
        donwload_pool.push(tokio::spawn(
            async move { download_thumb(&path, &thumb).await },
        ));
    }

    for task in donwload_pool {
        let _ = task.await;
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum MovieNfoServiceError {
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    NfoCreateError(#[from] NfoServiceError),
    #[error(transparent)]
    SledError(#[from] crate::data::subscribe_rules::SubscribeRulesDataError),
    #[error(transparent)]
    NetworkError(#[from] crate::model::nfo::NfoGetError),
}
