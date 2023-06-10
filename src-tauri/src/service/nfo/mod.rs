pub mod movie;
pub mod tvshow;
use crate::http::client::Client;
use quick_xml::se::Serializer;
use reqwest::{header::HeaderMap, StatusCode};
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

async fn download_thumb<P: AsRef<Path>>(path: P, url: &str) -> Result<(), NfoServiceError> {
    log::info!("Downloading thumb {:?}", url);
    let img = match Client::get()
        .get_bytes(url.to_string(), HeaderMap::new())
        .await
    {
        Ok(res) => match res.1 {
            reqwest::StatusCode::OK => res.0,
            _ => {
                log::error!(
                    "Error downloading {} thumb: {}",
                    path.as_ref().display(),
                    res.1
                );
                return Err(NfoServiceError::DownloadThumbServerError(res.1));
            }
        },
        Err(e) => {
            log::error!("Error downloading {} thumb: {}", path.as_ref().display(), e);
            return Err(NfoServiceError::DownloadThumbClientError(e));
        }
    };

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
    let mut writer =
        String::from(r#"<?xml version="1.0" encoding="utf-8" standalone="yes"?>"#) + "\n";
    let mut ser = Serializer::new(&mut writer);
    ser.indent(' ', 4);
    data.serialize(ser)?;

    file.write_all(writer.as_bytes())?;

    Ok(())
}

fn make_vaild_pathname(name: &str) -> String {
    name.replace("/", "／")
        .replace("\\", "＼")
        .replace(":", "：")
        .replace("*", "＊")
        .replace("?", "？")
        .replace("\"", "＂")
        .replace("<", "＜")
        .replace(">", "＞")
        .replace("|", "｜")
}

#[derive(thiserror::Error, Debug)]
pub enum NfoServiceError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    SerializeError(#[from] quick_xml::DeError),
    #[error(transparent)]
    RegexBuildError(#[from] crate::utils::matcher::MatcherError),
    #[error(transparent)]
    DownloadThumbClientError(#[from] reqwest::Error),
    #[error("Error downloading thumb")]
    DownloadThumbServerError(StatusCode),
}
