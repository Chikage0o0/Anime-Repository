use crate::model::nfo::public::{Provider, ProviderKnown, Uniqueid};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub tvshow_regex: String,
    pub season: u64,
    pub episode_offset: i64,
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

pub fn get(key: &Key) -> Option<Value> {
    if let Some(x) = &DB.get(bincode::serialize(&key).unwrap()).unwrap() {
        Some(bincode::deserialize(&x.to_vec()[..]).unwrap())
    } else {
        None
    }
}

pub fn insert(key: &Key, value: &Value) -> sled::Result<()> {
    DB.insert(
        bincode::serialize(&key).unwrap(),
        bincode::serialize(&value).unwrap(),
    )?;
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn main() -> sled::Result<()> {
        // key and value types can be `Vec<u8>`, `[u8]`, or `str`.
        let key = Key {
            id: "dad".to_string(),
            provider: ProviderKnown::TMDB,
        };

        // `generate_id`
        let value = Value {
            tvshow_regex: "da21d".to_string(),
            season: 1,
            episode_offset: 0,
            episode_regex: "df".to_string(),
        };

        dbg!(get(&key));

        dbg!(list());

        Ok(())
    }
}
