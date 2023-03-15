use crate::service::nfo;
use crate::{
    data::subscribe_rules::{list, Key, Value},
    model::{nfo::ProviderKnown, setting::Setting},
    utils::file::walk_file,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Mutex;
use std::{
    num::ParseIntError,
    path::{Path, PathBuf},
};

static MATCHERS: Lazy<Mutex<Vec<Matcher>>> = Lazy::new(|| Mutex::new(Matcher::get_all()));

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
            match self
                .episode_regex
                .captures_iter(file_name)
                .collect::<Vec<_>>()
                .get(self.episode_position as usize)
            {
                Some(cap) => {
                    let episode =
                        cap.get(0).unwrap().as_str().parse::<u64>()? + self.episode_offset as u64;
                    Ok((file_path.as_ref().to_path_buf(), episode))
                }
                _ => {
                    log::warn!("Tvshow Episode not match: {}", file_path.as_ref().display());
                    Err(MatcherError::FileNotMatch(file_path.as_ref().to_path_buf()))
                }
            }
        } else {
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
        log::info!("Insert matcher: {:?}", self);
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
    SledError(#[from] crate::data::subscribe_rules::SubscribeRulesDataError),
    #[error(transparent)]
    RegexBuildError(#[from] regex::Error),
    #[error(transparent)]
    InvaildEpisode(#[from] std::num::TryFromIntError),
    #[error("Can't match `{0}`")]
    FileNotMatch(std::path::PathBuf),
    #[error("`{0}` not a file")]
    NotFile(std::path::PathBuf),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Process error `{0}`")]
    ProcessError(String),
}

static INTERNAL_MATCHER: Lazy<pcre2::bytes::Regex> = Lazy::new(|| {
    let regex = r###"
        ^
        # get the title of this movie or series
        (?<title>
            [-\w'\"]+
            # match separator to later replace into correct title
            (?<separator> [\s.] )
            # note this must be lazy for the engine to work ltr not rtl
            (?: [-\w'\"]+\2 )*?
        )(?:
            # if this is an episode, lets match the season
            # number one way or another. if not, the year of the movie
            # make sure this is not just a number in the title followed by our separator.
            # like, iron man 3 2013 or my.fictional.24.series
            (?! \d+ \2 )
            # now try to match the season number
            (?: s (?: \2? )? )?
            (?<season> \d\d? )
            (?: e|x (?:\2? )? )
            (?<episode> \d\d? )
            # needed to validate the last token is a dot, or whatever.
            (?: e\d\d? (?:-e?\d\d?)? | x\d\d? )? 
        |
            # this is likely a movie, match the year
            [(\[]?(?<year>\d{4})[)\]]?
        )"###;
    pcre2::bytes::RegexBuilder::new()
        .caseless(true)
        .extended(true)
        .build(regex)
        .unwrap()
});

/// 内部匹配器，用于匹配文件名中的标题、季、集、年份
pub async fn internal_matcher<P: AsRef<Path>>(file: P) -> Result<(), MatcherError> {
    let file_name = file
        .as_ref()
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let caps = INTERNAL_MATCHER.captures(file_name.as_bytes());
    if let Ok(Some(caps)) = caps {
        let extract = |name: &str| {
            caps.name(name)
                .and_then(|f| String::from_utf8(f.as_bytes().to_vec()).ok())
        };

        let title = extract("title");
        let separator = extract("separator");
        let season = extract("season");
        let episode = extract("episode");
        let year = extract("year");

        if let Some(mut title) = title {
            // 替换分隔符
            if separator.is_some() {
                title = title.replace(&separator.unwrap(), " ");
            }
            let r#type = if season.is_some() && episode.is_some() {
                super::r#Type::Tvshow
            } else {
                super::r#Type::Movie
            };
            let provider = Setting::get_default_provider();
            // 搜索
            let search_result = super::search::search(&title, provider, r#type).await;
            match search_result {
                Ok(v) => {
                    let lang = &Setting::get_default_lang();
                    match r#type {
                        super::Type::Tvshow => {
                            // 电视剧 默认第一个结果
                            if let Some(result) = v.get(0) {
                                let id = result.id.clone();
                                let title = result.title.clone();

                                let path = file.as_ref().to_path_buf();
                                nfo::tvshow::process(
                                    &id,
                                    provider,
                                    &title,
                                    lang,
                                    season.unwrap().parse::<u64>()?,
                                    episode.unwrap().parse::<u64>()?,
                                    &path,
                                )
                                .await
                                .map_err(|e| MatcherError::ProcessError(e.to_string()))?;
                                return Ok(());
                            } else {
                                log::error!("Can't find `{}`", title);
                            }
                        }
                        // 电影 如果有年份就匹配年份
                        super::Type::Movie => {
                            let result;
                            if year.is_some() {
                                result = v.iter().find(|f| f.year == year);
                            } else {
                                result = v.get(0);
                            }

                            if let Some(result) = result {
                                let id = result.id.clone();
                                let path = file.as_ref().to_path_buf();
                                nfo::movie::process(&id, provider, lang, &path)
                                    .await
                                    .map_err(|e| MatcherError::ProcessError(e.to_string()))?;
                                return Ok(());
                            } else {
                                log::error!("Can't find `{}`", title);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Can't search for `{}`: {}", title, e);
                }
            }
        }
    }
    return Err(MatcherError::FileNotMatch(file.as_ref().to_path_buf()));
}
