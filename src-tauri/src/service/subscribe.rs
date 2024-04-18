use crate::{
    data::subscribe_rules::{Key, Value},
    http::tmdb::TMDBClient,
    model::nfo::ProviderKnown,
    utils::matcher::Matcher,
};
use reqwest::StatusCode;
use std::{fmt::Debug, path::Path};

use super::nfo::{tvshow, NfoServiceError};

pub async fn insert((key, value): (Key, Value)) -> Result<(), SubscribeServiceError> {
    log::info!("Inserting subscribe {:?}{:?}", key, value);
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.insert(&value)?;
    Matcher::reload();

    tokio::spawn(async move {
        for i in matcher.match_all_videos().iter() {
            match process(&key, &i.0, i.1).await {
                Ok(_) => {
                    let _ = crate::service::unrecognized_videos::delete(&i.0);
                }
                Err(e) => {
                    crate::service::unrecognized_videos::insert(
                        &i.0,
                        crate::data::unrecognized_videos::VideoData::Undefined,
                    )
                    .await
                    .unwrap_or_else(|e| log::error!("Insert unrecognized video error: {:?}", e));
                    log::error!("Process video error: {:?}", e);
                }
            }
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

pub async fn get_title(
    id: &str,
    provider: ProviderKnown,
    lang: &str,
    r#type: &str,
) -> Result<String, SubscribeServiceError> {
    match r#type {
        "tvshow" => match provider {
            ProviderKnown::TMDB => {
                let response = TMDBClient::default()
                    .get_tvshow_info(id, lang)
                    .await
                    .map_err(|e| SubscribeServiceError::NetworkError(e.to_string()))?;
                match response.1 {
                    StatusCode::OK => {
                        let data: serde_json::Value = serde_json::from_str(&response.0).unwrap();
                        match data.get("name") {
                            Some(name) => Ok(name.as_str().unwrap_or_default().to_string()),
                            None => Err(SubscribeServiceError::NetworkError(
                                "Failed to get title".to_string(),
                            ))?,
                        }
                    }
                    _ => Err(SubscribeServiceError::NetworkError(format!(
                        "Failed to get title,{}",
                        response.1
                    )))?,
                }
            }
            ProviderKnown::IMDB => Err(SubscribeServiceError::NetworkError(
                "IMDB provider not implemented yet".to_string(),
            ))?,
        },
        "movie" => match provider {
            ProviderKnown::TMDB => {
                let response = TMDBClient::default()
                    .get_movie_info(id, lang)
                    .await
                    .map_err(|e| SubscribeServiceError::NetworkError(e.to_string()))?;
                match response.1 {
                    StatusCode::OK => {
                        let data: serde_json::Value = serde_json::from_str(&response.0).unwrap();
                        match data.get("title") {
                            Some(name) => Ok(name.as_str().unwrap_or_default().to_string()),
                            None => Err(SubscribeServiceError::NetworkError(
                                "Failed to get title".to_string(),
                            ))?,
                        }
                    }
                    _ => {
                        return Err(SubscribeServiceError::NetworkError(format!(
                            "Failed to get title,{}",
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
        },
        _ => Err(SubscribeServiceError::NetworkError(
            "Unknown type".to_string(),
        ))?,
    }
}

pub async fn process<P: AsRef<Path>>(
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
    )
    .await
    {
        crate::service::unrecognized_videos::insert(
            path,
            crate::data::unrecognized_videos::VideoData::Undefined,
        )
        .await?;
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
