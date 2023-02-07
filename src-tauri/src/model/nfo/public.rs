use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKnown {
    TMDB,
    IMDB,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    fn get_id(&self, provider: Provider) -> Option<&String>;
    /// 根据NFO获取默认的ID
    fn get_default_id(&self) -> Option<(&String, &Provider)>;
    fn read_from_file() -> Self;
}
