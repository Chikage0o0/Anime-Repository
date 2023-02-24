use super::nfo::{tvshow, *};
use crate::{
    data::scribe::{Key, Value},
    utils::matcher::Matcher,
};

use std::{fmt::Debug, path::PathBuf};

fn insert((key, value): (Key, Value)) -> Result<(), ScribeServiceError> {
    log::info!("Inserting scribe {:?}", key);
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.insert(&value)?;
    matcher.insert();
    Ok(())
}

fn remove(key: Key) -> Result<(), ScribeServiceError> {
    key.delete()?;
    Matcher::delete(&key.id, key.provider);
    log::debug!("Remove scribe: {:?}", key);
    Ok(())
}

pub fn process(key: Key, path: PathBuf, episode: u64) -> Result<(), ScribeServiceError> {
    let value = key.get()?;

    tvshow::process(
        &key.id,
        key.provider,
        &value.title,
        &value.lang,
        value.season,
        episode,
        path,
    )?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ScribeServiceError {
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    NfoCreateError(#[from] NfoServiceError),
    #[error(transparent)]
    SledError(#[from] crate::data::scribe::ScribeDataError),
    #[error(transparent)]
    ProcessTvshowInfoError(#[from] super::nfo::tvshow::TvshowNfoServiceError),
}
