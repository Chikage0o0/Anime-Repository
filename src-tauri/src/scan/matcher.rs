use crate::{data::scribe::Key, model::nfo::public::ProviderKnown};
use regex::Regex;
use std::path::{Path, PathBuf};

struct Matcher {
    id: String,
    provider: ProviderKnown,
    tvshow_regex: Regex,
    season: u64,
    episode_offset: i64,
    episode_position: u8,
    episode_regex: Regex,
}

impl TryFrom<Key> for Matcher {
    type Error = MatcherError;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        let value = key.get()?;
        Ok(Self {
            id: key.id,
            provider: key.provider,
            season: value.season,
            tvshow_regex: Regex::new(&value.tvshow_regex)?,
            episode_offset: value.episode_offset,
            episode_position: value.episode_position,
            episode_regex: Regex::new(&value.episode_regex)?,
        })
    }
}

impl Matcher {
    /// FullPath match tvshow_regex
    /// FileName match episode_regex + episode_offset
    fn match_video<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> Result<(PathBuf, u64, u64), MatcherError> {
        if self
            .tvshow_regex
            .is_match(file_path.as_ref().to_str().unwrap_or(""))
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

    fn match_all_videos(&self) -> Vec<Option<(u64, u64)>> {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MatcherError {
    #[error(transparent)]
    DataGetError(#[from] crate::data::scribe::ScribeError),
    #[error(transparent)]
    RegexBuildError(#[from] regex::Error),
    #[error(transparent)]
    InvaildEpisode(#[from] std::num::TryFromIntError),
    #[error("Can't match `{0}`")]
    FileNotMatch(std::path::PathBuf),
    #[error("`{0}` not a file")]
    NotAFile(std::path::PathBuf),
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
}
