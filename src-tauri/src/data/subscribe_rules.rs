use std::path::PathBuf;

use crate::model::nfo::{Provider, ProviderKnown, Uniqueid};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;

lazy_static! {
    static ref DB: sled::Db = sled::open(
        PathBuf::from(tauri::api::path::config_dir().unwrap())
            .join("AnimeRepository")
            .join("subscribe_rules")
    )
    .unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    pub id: String,
    pub provider: ProviderKnown,
}

impl From<Key> for Uniqueid {
    fn from(value: Key) -> Self {
        Self {
            r#type: Provider::Known(value.provider),
            default: false,
            value: value.id,
        }
    }
}

impl Key {
    pub fn get(&self) -> Result<Value, SubscribeRulesDataError> {
        let serialized_self = bincode::serialize(self).unwrap();
        if let Some(x) = &DB.get(serialized_self).unwrap() {
            Ok(bincode::deserialize(&x.to_vec()[..]).unwrap())
        } else {
            Err(SubscribeRulesDataError::KeyNotFound(
                json!(self).to_string(),
            ))
        }
    }

    pub fn insert(&self, value: &Value) -> Result<(), SubscribeRulesDataError> {
        DB.insert(
            bincode::serialize(self).unwrap(),
            bincode::serialize(&value).unwrap(),
        )?;
        Ok(())
    }

    pub fn delete(&self) -> Result<(), SubscribeRulesDataError> {
        let serialized_self = bincode::serialize(self).unwrap();
        DB.remove(serialized_self)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Value {
    pub title: String,
    pub tvshow_regex: String,
    pub season: u64,
    pub episode_offset: i64,
    pub episode_position: u8,
    pub episode_regex: String,
    pub lang: String,
}

pub fn list() -> Vec<(Key, Value)> {
    DB.iter()
        .filter_map(|f| {
            if let Some(tmp) = f.ok() {
                Some((
                    bincode::deserialize(&tmp.0.to_vec()[..]).unwrap(),
                    bincode::deserialize(&tmp.1.to_vec()[..]).unwrap(),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<(Key, Value)>>()
}

#[derive(thiserror::Error, Debug)]
pub enum SubscribeRulesDataError {
    #[error("Key `{0}` not found in database")]
    KeyNotFound(String),
    #[error(transparent)]
    SledError(#[from] sled::Error),
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn main() -> sled::Result<()> {
        dbg!(list());

        Ok(())
    }

    #[test]
    fn insert() -> Result<(), SubscribeRulesDataError> {
        let key = Key {
            id: "tt1234567".to_string(),
            provider: ProviderKnown::IMDB,
        };
        let value = Value {
            title: "The Big Bang Theory".to_string(),
            tvshow_regex: "The Big Bang Theory".to_string(),
            season: 1,
            episode_offset: 0,
            episode_position: 0,
            episode_regex: r"\d+".to_string(),
            lang: "en_US".to_string(),
        };
        key.insert(&value)?;
        dbg!(list());
        Ok(())
    }
}
