use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    pub name: String,
    pub role: String,
    pub order: Option<usize>,
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
    pub max: String,
    #[serde(rename = "@default", default)]
    pub default: bool,
    pub value: String,
    pub votes: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Thumb {
    #[serde(rename = "@aspect")]
    pub aspect: Option<String>,
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@season")]
    pub season: Option<i32>,
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
    pub r#type: String,
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

pub fn get_img_url(path: &str) -> String {
    format!("https://image.tmdb.org/t/p/original{}", path)
}

pub fn get_date() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%m:%s").to_string()
}
