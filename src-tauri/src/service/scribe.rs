use crate::{
    data::{
        pending_videos::insert_pending_video,
        scribe::{Key, Value},
    },
    model::{
        nfo::{episode::Episode, public::Nfo, tvshow::Tvshow},
        setting,
    },
    utils::{file, matcher::Matcher},
};
use quick_xml::se::Serializer;
use serde::Deserialize;
use std::{
    fmt::Debug,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

fn write_nfo<P, C>(path: P, data: &C) -> Result<(), ScribeServiceError>
where
    P: AsRef<Path>,
    C: serde::Serialize,
{
    if let Some(path) = path.as_ref().parent() {
        std::fs::create_dir_all(path)?;
    }

    let mut file = File::create(path)?;
    let mut ser = Serializer::new(String::new());
    ser.indent(' ', 4);
    let data = data.serialize(ser)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}

fn insert_scribe((key, value): (Key, Value)) -> Result<(), ScribeServiceError> {
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.insert(&value)?;
    matcher.insert();
    let result = matcher.match_all_videos();
    result
        .into_iter()
        .for_each(|(path, season, episode)| todo!("{:?} {:?} {:?}", path, season, episode));
    Ok(())
}

fn read_nfo<P, C>(path: P) -> Result<C, ScribeServiceError>
where
    P: AsRef<Path>,
    C: for<'a> Deserialize<'a>,
{
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let data = quick_xml::de::from_str(&buffer)?;
    Ok(data)
}

pub fn process(
    key: Key,
    path: PathBuf,
    season: u64,
    episode: u64,
) -> Result<(), ScribeServiceError> {
    let value = key.get().unwrap();

    use tauri::async_runtime::block_on;

    let tvshow_title = value.title.clone();
    let tvshow_path = setting::Setting::get()
        .storage
        .repository_path
        .join(&tvshow_title);
    let tvshow_nfo_path = tvshow_path.join("tvshow.nfo");

    let mut tvshow_nfo: Tvshow;
    if tvshow_nfo_path.exists() {
        tvshow_nfo = read_nfo(&tvshow_nfo_path)?;
        block_on(tvshow_nfo.update(&value.lang));
    } else {
        tvshow_nfo = Tvshow::new(&key.id, key.provider.into());
        block_on(tvshow_nfo.update(&value.lang));
    }

    write_nfo(&tvshow_nfo_path, &tvshow_nfo).unwrap();

    let mut episode_nfo = Episode::new(&key.id, key.provider.into());
    block_on(episode_nfo.update(&value.lang, season, episode));

    let episode_title = episode_nfo.title.clone();

    let episode_folder_path = tvshow_path.join(format!("Season {:02}", season));
    let episode_nfo_path = episode_folder_path.join(format!(
        "{} - S{:02}E{:02} - {}.nfo",
        &tvshow_title, season, episode, &episode_title
    ));
    let episode_path = episode_folder_path.join(format!(
        "{} - S{:02}E{:02} - {}.{}",
        &tvshow_title,
        season,
        episode,
        &episode_title,
        path.extension().unwrap().to_str().unwrap()
    ));

    if let Err(_err) = file::move_file(&path, &episode_path) {
        insert_pending_video(path, episode_path);
        // eprintln!("Error: {}", err)
    }
    write_nfo(&episode_nfo_path, &episode_nfo).unwrap();

    Ok(())
}

fn remove_scribe((key, value): (Key, Value)) -> Result<(), ScribeServiceError> {
    let matcher: Matcher = (key.clone(), value.clone()).try_into()?;
    key.delete()?;
    matcher.delete();
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ScribeServiceError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    SerializeError(#[from] quick_xml::DeError),
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    SledError(#[from] crate::data::scribe::ScribeDataError),
}
