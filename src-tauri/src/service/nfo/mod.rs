pub mod tvshow;

use crate::http::client;
use quick_xml::se::Serializer;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn download_thumb<P: AsRef<Path>>(path: P, url: &str) -> Result<(), NfoServiceError> {
    log::info!("Downloading thumb {:?}", url);
    use tauri::async_runtime::block_on;
    let img = block_on(client::get_bytes(url.to_string(), HeaderMap::new()));
    if let Some(path) = path.as_ref().parent() {
        std::fs::create_dir_all(path)?;
    }

    let mut file = std::fs::File::create(path.as_ref())?;
    file.write_all(&img)?;
    Ok(())
}

fn read_nfo<P, C>(path: P) -> Result<C, NfoServiceError>
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

fn write_nfo<P, C>(path: P, data: &C) -> Result<(), NfoServiceError>
where
    P: AsRef<Path>,
    C: serde::Serialize,
{
    log::info!("Writing nfo {:?}", path.as_ref());
    if let Some(path) = path.as_ref().parent() {
        std::fs::create_dir_all(path)?;
    }

    let mut file = File::create(path)?;
    let mut ser = Serializer::new(
        String::from(r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>"#) + "\n",
    );
    ser.indent(' ', 4);
    let data = data.serialize(ser)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum NfoServiceError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    SerializeError(#[from] quick_xml::DeError),
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
}
