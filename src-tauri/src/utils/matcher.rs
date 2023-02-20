use crate::{
    data::scribe::{list, Key, Value},
    model::{nfo::public::ProviderKnown, setting::Setting},
    utils::file::walk_file,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

lazy_static! {
    static ref MATCHERS: Mutex<Vec<Matcher>> = Mutex::new(Matcher::get_all());
}

#[derive(Clone)]
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

impl From<Matcher> for (Key, Value) {
    fn from(matcher: Matcher) -> Self {
        (
            Key {
                id: matcher.id,
                provider: matcher.provider,
            },
            Value {
                season: matcher.season,
                tvshow_regex: matcher.tvshow_regex.as_str().to_string(),
                episode_offset: matcher.episode_offset,
                episode_position: matcher.episode_position,
                episode_regex: matcher.episode_regex.as_str().to_string(),
            },
        )
    }
}

impl Matcher {
    /// FullPath match tvshow_regex
    /// FileName match episode_regex + episode_offset
    fn match_video<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<(PathBuf, u64, u64), MatcherError> {
        if !file_path.as_ref().is_file() {
            return Err(MatcherError::NotFile(file_path.as_ref().to_path_buf()));
        }

        let ext = file_path
            .as_ref()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap();

        if !matches!(ext, "mkv" | "mp4" | "webm") {
            return Err(MatcherError::FileNotVideo(file_path.as_ref().to_path_buf()));
        }

        if self
            .tvshow_regex
            .is_match(file_path.as_ref().to_str().unwrap_or_default())
        {
            let file_name = file_path.as_ref().file_name().unwrap().to_str().unwrap();
            match self.episode_regex.captures(file_name) {
                Some(caps) if caps.len() == 1 => Ok((
                    file_path.as_ref().to_path_buf(),
                    self.season,
                    (caps
                        .get(self.episode_position.into())
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap()
                        + self.episode_offset)
                        .try_into()?,
                )),
                _ => Err(MatcherError::FileNotMatch(file_path.as_ref().to_path_buf())),
            }
        } else {
            Err(MatcherError::FileNotMatch(file_path.as_ref().to_path_buf()))
        }
    }

    pub fn match_all_videos(&self) -> Vec<(PathBuf, u64, u64)> {
        walk_file(Setting::get().storage.pending_path.as_path())
            .iter()
            .filter_map(|f| self.match_video(f).ok())
            .collect::<Vec<(PathBuf, u64, u64)>>()
    }

    pub fn matchers_video<P: AsRef<Path>>(file_path: P) -> Option<(PathBuf, u64, u64)> {
        let matchers = MATCHERS.lock().unwrap();
        for matcher in matchers.iter() {
            if let Ok((path, season, episode)) = matcher.match_video(file_path.as_ref()) {
                return Some((path, season, episode));
            }
        }
        None
    }

    fn get_all() -> Vec<Self> {
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
                    _ => panic!("Can't match {:?}", f),
                }
            })
            .collect()
    }

    pub fn insert(&self) -> Result<(), MatcherError> {
        let (key, value) = self.clone().into();
        key.insert(&value)?;
        let mut matchers = MATCHERS.lock().unwrap();
        matchers.push(self.clone());
        Ok(())
    }

    pub fn delete(&self) -> Result<(), MatcherError> {
        let (key, _) = self.clone().into();
        key.delete()?;
        let mut matchers = MATCHERS.lock().unwrap();
        matchers.retain(|m| m.id != self.id && m.provider != self.provider);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MatcherError {
    #[error(transparent)]
    SledError(#[from] crate::data::scribe::ScribeError),
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

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_try_from() {
        use crate::data::scribe::*;
        let key = Key {
            id: "207965".to_string(),
            provider: ProviderKnown::TMDB,
        };

        let value = Value {
            tvshow_regex: "Tensei Oujo to Tensai Reijou no Mahou Kakumei".to_string(),
            season: 1,
            episode_offset: 0,
            episode_position: 0,
            episode_regex: r"\d+".to_string(),
        };
        key.insert(&value).unwrap();
        let matcher: Matcher = key.try_into().unwrap();
        assert_eq!(matcher.id, "207965");
        assert_eq!(matcher.provider, ProviderKnown::TMDB);
        assert_eq!(matcher.season, 1);
    }
    #[test]
    fn test_match_video() {
        use crate::data::scribe::*;
        let key = Key {
            id: "207965".to_string(),
            provider: ProviderKnown::TMDB,
        };

        let value = Value {
            tvshow_regex: "Tensei Oujo to Tensai Reijou no Mahou Kakumei".to_string(),
            season: 1,
            episode_offset: 0,
            episode_position: 0,
            episode_regex: r"\d+".to_string(),
        };

        key.insert(&value).unwrap();

        let matcher: Matcher = key.try_into().unwrap();
        let result = matcher.match_video(r"C:\Users\chika\Downloads\AnimeRepository\[Lilith-Raws] Tensei Oujo to Tensai Reijou no Mahou Kakumei - 07 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4").unwrap();
        assert_eq!(
            result.0,
            PathBuf::from(
                r"C:\Users\chika\Downloads\AnimeRepository\[Lilith-Raws] Tensei Oujo to Tensai Reijou no Mahou Kakumei - 07 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4"
            )
        );
        assert_eq!(result.1, 1);
        assert_eq!(result.2, 7);
    }

    #[test]
    fn test_match_all_video() {
        use crate::data::scribe::*;
        let key = Key {
            id: "207965".to_string(),
            provider: ProviderKnown::TMDB,
        };

        let value = Value {
            tvshow_regex: "Tensei Oujo to Tensai Reijou no Mahou Kakumei".to_string(),
            season: 1,
            episode_offset: 0,
            episode_position: 0,
            episode_regex: r"\d+".to_string(),
        };

        key.insert(&value).unwrap();

        let matcher: Matcher = key.try_into().unwrap();
        let result = matcher.match_all_videos();
        dbg!(result);
    }
}
