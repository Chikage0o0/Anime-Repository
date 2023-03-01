use crate::{
    data::subscribe_rules::{Key, Value},
    http::tmdb,
    model::nfo::ProviderKnown,
    utils::matcher::Matcher,
};
use reqwest::StatusCode;
use std::{fmt::Debug, path::Path};

use super::nfo::{tvshow, NfoServiceError};

pub fn insert((key, value): (Key, Value)) -> Result<(), SubscribeServiceError> {
    log::info!("Inserting subscribe {:?}{:?}", key, value);
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.insert(&value)?;
    matcher.insert();

    std::thread::spawn(move || {
        for i in matcher.match_all_videos().iter() {
            process(&key, &i.0, i.1).unwrap();
            crate::service::unrecognized_videos::delete(&i.0).unwrap();
        }
    });

    Ok(())
}

pub fn remove(key: Key) -> Result<(), SubscribeServiceError> {
    key.delete()?;
    Matcher::delete(&key.id, key.provider);
    log::info!("Remove scribe: {:?}", key);
    Ok(())
}

pub async fn get_tvshow_title(
    id: &str,
    provider: ProviderKnown,
    lang: &str,
) -> Result<String, SubscribeServiceError> {
    match provider {
        ProviderKnown::TMDB => {
            let response = tmdb::get_tvshow_info(id, lang)
                .await
                .map_err(|e| SubscribeServiceError::NetworkError(e.to_string()))?;
            match response.1 {
                StatusCode::OK => {
                    let data: serde_json::Value = serde_json::from_str(&response.0).unwrap();
                    if let Some(name) = data.get("name") {
                        return Ok(name.as_str().unwrap().to_string());
                    }
                }
                _ => {
                    return Err(SubscribeServiceError::NetworkError(format!(
                        "Failed to get tvshow title,{}",
                        response.1
                    ))
                    .into())
                }
            }
        }
        ProviderKnown::IMDB => {
            return Err(SubscribeServiceError::NetworkError(
                "IMDB provider not implemented yet".to_string(),
            ))
        }
    };

    Err(SubscribeServiceError::NetworkError(
        "Failed to get tvshow title".to_string(),
    ))
}

pub fn process<P: AsRef<Path>>(
    key: &Key,
    path: P,
    episode: u64,
) -> Result<(), SubscribeServiceError> {
    let value = key.get()?;

    if let Err(e) = tvshow::process(
        &key.id,
        key.provider,
        &value.title,
        &value.lang,
        value.season,
        episode,
        &path,
    ) {
        crate::service::unrecognized_videos::insert(
            path,
            crate::data::unrecognized_videos::VideoData::Undefined,
        )?;
        return Err(SubscribeServiceError::ProcessTvshowInfoError(e));
    }
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum SubscribeServiceError {
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    NfoCreateError(#[from] NfoServiceError),
    #[error(transparent)]
    SledError(#[from] crate::data::subscribe_rules::SubscribeRulesDataError),
    #[error(transparent)]
    ProcessTvshowInfoError(#[from] super::nfo::tvshow::TvshowNfoServiceError),
    #[error(transparent)]
    UnrecognizedVideoServiceError(
        #[from] crate::service::unrecognized_videos::UnrecognizedVideosServiceError,
    ),
    #[error("`{0}`")]
    NetworkError(String),
}
