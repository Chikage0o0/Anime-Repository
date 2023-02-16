use crate::model::nfo::public::{Provider, ProviderKnown, Uniqueid};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;

lazy_static! {
    static ref DB: sled::Db = sled::open("config/scribe").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn get(&self) -> Result<Value, ScribeError> {
        if let Some(x) = &DB.get(bincode::serialize(self).unwrap()).unwrap() {
            Ok(bincode::deserialize(&x.to_vec()[..]).unwrap())
        } else {
            Err(ScribeError::KeyNotFound(json!(self).to_string()))
        }
    }

    pub fn insert(&self, value: &Value) -> sled::Result<()> {
        DB.insert(
            bincode::serialize(self).unwrap(),
            bincode::serialize(&value).unwrap(),
        )?;
        Ok(())
    }

    pub fn delete(&self) -> sled::Result<()> {
        DB.remove(bincode::serialize(self).unwrap())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub tvshow_regex: String,
    pub season: u64,
    pub episode_offset: i64,
    pub episode_position: u8,
    pub episode_regex: String,
}

pub fn list() -> Vec<(Key, Value)> {
    DB.iter()
        .filter(|f| f.is_ok())
        .map(|f| {
            let tmp = f.unwrap();
            (
                bincode::deserialize(&tmp.0.to_vec()[..]).unwrap(),
                bincode::deserialize(&tmp.1.to_vec()[..]).unwrap(),
            )
        })
        .collect::<Vec<(Key, Value)>>()
}

#[derive(thiserror::Error, Debug)]
pub enum ScribeError {
    #[error("Key `{0}` not found in database")]
    KeyNotFound(String),
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn main() -> sled::Result<()> {
        dbg!(list());

        Ok(())
    }
}
