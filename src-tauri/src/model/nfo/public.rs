use serde::{Deserialize, Serialize};
use serde_with::{rust::deserialize_ignore_any, skip_serializing_none};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ValueString {
    #[serde(rename = "$value", default)]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    #[serde(rename = "$value", default)]
    pub value: Vec<ValueActor>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValueActor {
    Name(ValueString),
    Role(ValueString),
    Order(ValueString),
    Thumb(ValueString),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ratings {
    #[serde(rename = "$value", default)]
    pub value: Vec<Rating>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rating {
    pub name: String,
    pub max: String,
    pub default: bool,
    #[serde(rename = "$value", default)]
    pub value: Vec<ValueRating>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValueRating {
    Value(ValueString),
    Votes(ValueString),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Thumb {
    pub aspect: Option<String>,
    pub r#type: Option<String>,
    pub season: Option<String>,
    pub preview: Option<String>,
    #[serde(rename = "$value", default)]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fanart {
    #[serde(rename = "$value", default)]
    pub value: Vec<Thumb>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Uniqueid {
    pub r#type: Option<String>,
    pub default: Option<bool>,
    #[serde(rename = "$value", default)]
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Resume {
    #[serde(rename = "$value", default)]
    pub value: Vec<ValueResume>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValueResume {
    Position(ValueString),
    Total(ValueString),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}
