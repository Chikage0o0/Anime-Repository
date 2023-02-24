use crate::{
    data::scribe::{list, Key, Value},
    model::{nfo::ProviderKnown, setting::Setting},
    utils::file::walk_file,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

lazy_static! {
    static ref MATCHERS: Mutex<Vec<Matcher>> = Mutex::new(Matcher::get_all());
}

#[derive(Clone, Debug)]
pub struct Matcher {
    pub id: String,
    pub provider: ProviderKnown,
    pub tvshow_regex: Regex,
    pub season: u64,
    pub episode_offset: i64,
    pub episode_position: u8,
    pub episode_regex: Regex,
}

impl TryFrom<Key> for Matcher {
    type Error = MatcherError;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        let value = key.get()?;
        Ok(Self {
            id: key.id.clone(),
            provider: key.provider,
            season: value.season,
            tvshow_regex: Regex::new(&value.tvshow_regex)?,
            episode_offset: value.episode_offset,
            episode_position: value.episode_position,
            episode_regex: Regex::new(&value.episode_regex)?,
        })
    }
}

impl TryFrom<(Key, Value)> for Matcher {
    type Error = MatcherError;

    fn try_from((key, value): (Key, Value)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: key.id.clone(),
            provider: key.provider,
            season: value.season,
            tvshow_regex: Regex::new(&value.tvshow_regex)?,
            episode_offset: value.episode_offset,
            episode_position: value.episode_position,
            episode_regex: Regex::new(&value.episode_regex)?,
        })
    }
}

impl From<Matcher> for Key {
    fn from(matcher: Matcher) -> Self {
        Key {
            id: matcher.id,
            provider: matcher.provider,
        }
    }
}

impl Matcher {
    /// FullPath match tvshow_regex
    /// FileName match episode_regex + episode_offset
    fn match_video<P: AsRef<Path>>(&self, file_path: P) -> Result<(PathBuf, u64), MatcherError> {
        if !file_path.as_ref().is_file() || file_path.as_ref().is_symlink() {
            return Err(MatcherError::NotFile(file_path.as_ref().to_path_buf()));
        }

        if self
            .tvshow_regex
            .is_match(file_path.as_ref().to_str().unwrap_or_default())
        {
            let file_name = file_path.as_ref().file_name().unwrap().to_str().unwrap();
            match self.episode_regex.captures(file_name) {
                Some(caps) if caps.len() == 1 => Ok((
                    file_path.as_ref().to_path_buf(),
                    (caps
                        .get(self.episode_position.into())
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap()
                        + self.episode_offset)
                        .try_into()?,
                )),
                _ => {
                    log::warn!("Tvshow Episode not match: {}", file_path.as_ref().display());
                    Err(MatcherError::FileNotMatch(file_path.as_ref().to_path_buf()))
                }
            }
        } else {
            log::warn!("Tvshow Name not match: {}", file_path.as_ref().display());
            Err(MatcherError::FileNotMatch(file_path.as_ref().to_path_buf()))
        }
    }

    pub fn match_all_videos(&self) -> Vec<(PathBuf, u64)> {
        walk_file(Setting::get_pending_path())
            .iter()
            .filter_map(|f| self.match_video(f).ok())
            .collect::<Vec<(PathBuf, u64)>>()
    }

    pub fn matchers_video<P: AsRef<Path>>(file_path: P) -> Option<(Key, PathBuf, u64)> {
        let matchers = MATCHERS.lock().unwrap();
        for matcher in matchers.iter() {
            if let Ok((path, episode)) = matcher.match_video(file_path.as_ref()) {
                return Some((matcher.clone().into(), path, episode));
            }
        }
        None
    }

    fn get_all() -> Vec<Self> {
        log::debug!("Get all matchers");
        list()
            .into_iter()
            .filter_map(|f| -> Option<Matcher> {
                let tmp = f.clone().try_into();
                match tmp {
                    Ok(v) => Some(v),
                    Err(MatcherError::RegexBuildError(_)) => {
                        f.0.delete().unwrap();
                        None
                    }
                    _ => {
                        log::error!("Can't build matcher {:?}", &f);
                        f.0.delete().unwrap();
                        None
                    }
                }
            })
            .collect()
    }

    pub fn insert(&self) {
        let mut matchers = MATCHERS.lock().unwrap();
        matchers.push(self.clone());
        log::debug!("Insert matcher: {:?}", self);
    }

    pub fn delete(id: &str, provider: ProviderKnown) {
        let mut matchers = MATCHERS.lock().unwrap();
        matchers.retain(|m| m.id != id && m.provider != provider);
        log::info!("Delete matcher: {} {:?}", id, provider);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MatcherError {
    #[error(transparent)]
    SledError(#[from] crate::data::scribe::ScribeDataError),
    #[error(transparent)]
    RegexBuildError(#[from] regex::Error),
    #[error(transparent)]
    InvaildEpisode(#[from] std::num::TryFromIntError),
    #[error("Can't match `{0}`")]
    FileNotMatch(std::path::PathBuf),
    #[error("`{0}` not a file")]
    NotFile(std::path::PathBuf),
    #[error("`{0}` not a video file")]
    FileNotVideo(std::path::PathBuf),
}
