#![allow(dead_code)]
pub mod episode;
pub mod movie;
pub mod tvshow;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    pub name: String,
    pub role: String,
    pub order: Option<u64>,
    pub thumb: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ratings {
    #[serde(default)]
    pub rating: Vec<Rating>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rating {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@max")]
    pub max: i64,
    #[serde(rename = "@default", default)]
    pub default: bool,
    pub value: f64,
    pub votes: i64,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Thumb {
    #[serde(rename = "@aspect")]
    pub aspect: Option<String>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@season")]
    pub season: Option<i64>,
    #[serde(rename = "@preview")]
    pub preview: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fanart {
    pub thumb: Vec<Thumb>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Uniqueid {
    #[serde(rename = "@type")]
    pub r#type: Provider,
    #[serde(rename = "@default", default)]
    pub default: bool,
    #[serde(rename = "$value")]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Resume {
    pub position: String,
    pub tolal: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKnown {
    TMDB,
    IMDB,
}

impl From<ProviderKnown> for Provider {
    fn from(value: ProviderKnown) -> Self {
        Provider::Known(value)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Provider {
    Known(ProviderKnown),
    Unknown(String),
}

pub fn get_img_url(path: &str) -> String {
    format!("https://image.tmdb.org/t/p/original{}", path)
}

pub fn get_date() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%m:%S").to_string()
}

pub trait Nfo {
    fn new(id: &str, provider: Provider) -> Self;

    /// 根据NFO获取指定提供商的ID
    fn get_id(&self, provider: Provider) -> Option<String>;
    /// 根据NFO获取默认的ID
    fn get_default_id(&self) -> Option<(String, Provider)>;
}

#[derive(thiserror::Error, Debug)]
pub enum NfoGetError {
    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
    #[error("Error status code: {0},{1:?}")]
    ServerError(StatusCode, Option<String>),
    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),
}

fn get_json((json, status_code): (String, StatusCode)) -> Result<String, NfoGetError> {
    if status_code.is_success() {
        Ok(json)
    } else {
        Err(NfoGetError::ServerError(status_code, Some(json)))
    }
}

fn thumb_extension(url: &str, defalut: &str) -> String {
    let ext = url.split('.').last().unwrap_or(defalut);
    format!(".{}", ext)
}

fn get_logo(data: &Value, data_fallback: &Value) -> Option<String> {
    fn get_logo_url(logos: &Value) -> Option<String> {
        logos.as_array().and_then(|f| {
            f.iter()
                .flat_map(|value| value.get("file_path").and_then(|value| value.as_str()))
                .map(|value| get_img_url(value))
                .next()
        })
    }

    fn get_png_logo_url(logos: &Value) -> Option<String> {
        logos.as_array().and_then(|f| {
            f.iter()
                .flat_map(|value| {
                    value
                        .get("file_path")
                        .and_then(|value| value.as_str())
                        .filter(|value| value.ends_with("png"))
                        .map(|value| get_img_url(value))
                })
                .next()
        })
    }

    data.get("images")
        .and_then(|f| f.get("logos"))
        .and_then(|f| get_png_logo_url(f))
        .or_else(|| {
            data_fallback
                .get("images")
                .and_then(|f| f.get("logos"))
                .and_then(|f| get_png_logo_url(f))
        })
        .or_else(|| {
            data.get("images")
                .and_then(|f| f.get("logos"))
                .and_then(|f| get_logo_url(f))
        })
        .or_else(|| {
            data_fallback
                .get("images")
                .and_then(|f| f.get("logos"))
                .and_then(|f| get_logo_url(f))
        })
}
