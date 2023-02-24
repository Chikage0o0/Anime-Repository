use super::nfo::*;
use crate::{
    data::scribe::{Key, Value},
    utils::matcher::Matcher,
};

use std::fmt::Debug;

fn insert((key, value): (Key, Value)) -> Result<(), ScribeServiceError> {
    log::info!("Inserting scribe {:?}", key);
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.insert(&value)?;
    matcher.insert();
    let result = matcher.match_all_videos();
    result
        .into_iter()
        .for_each(|(path, season, episode)| todo!("{:?} {:?} {:?}", path, season, episode));
    Ok(())
}

fn remove((key, value): (Key, Value)) -> Result<(), ScribeServiceError> {
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.delete()?;
    matcher.delete();
    log::debug!("Remove scribe: {:?}", key);
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
}
