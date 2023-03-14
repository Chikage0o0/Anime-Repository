use crate::{
    data::subscribe_rules::{list, Key, Value},
    model::nfo::ProviderKnown,
    service::subscribe,
};
use actix_web::{delete, get, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SubscribeRule {
    id: String,
    provider: ProviderKnown,
    title: String,
    tvshow_regex: String,
    season: u64,
    episode_offset: i64,
    episode_position: u8,
    episode_regex: String,
    lang: String,
}

impl From<(Key, Value)> for SubscribeRule {
    fn from(value: (Key, Value)) -> Self {
        Self {
            id: value.0.id,
            provider: value.0.provider,
            title: value.1.title,
            tvshow_regex: value.1.tvshow_regex,
            season: value.1.season,
            episode_offset: value.1.episode_offset,
            episode_position: value.1.episode_position + 1,
            episode_regex: value.1.episode_regex,
            lang: value.1.lang,
        }
    }
}

#[get("/api/subscribe_rules")]
pub async fn get_subscribe_rules() -> impl Responder {
    web::Json(
        list()
            .into_iter()
            .map(|x| SubscribeRule::from(x))
            .collect::<Vec<SubscribeRule>>(),
    )
}

#[delete("/api/subscribe_rule")]
pub async fn delete_subscribe_rule(info: web::Query<Key>) -> Result<impl Responder> {
    subscribe::remove(info.0).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(()))
}

#[get("/api/subscribe_rule")]
pub async fn get_subscribe_rule(info: web::Query<Key>) -> Result<impl Responder> {
    let key = info.0;
    let value = key
        .get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(SubscribeRule::from((key, value))))
}

#[derive(Deserialize)]
pub struct InsertJson {
    id: String,
    provider: ProviderKnown,
    title: String,
    tvshow_regex: String,
    season: u64,
    episode_offset: i64,
    episode_position: u8,
    episode_regex: String,
    lang: String,
}

#[post("/api/subscribe_rule")]
pub async fn insert_subscribe_rule(info: web::Json<InsertJson>) -> Result<impl Responder> {
    let InsertJson {
        id,
        provider,
        title,
        tvshow_regex,
        season,
        episode_offset,
        episode_position,
        episode_regex,
        lang,
    } = info.into_inner();
    subscribe::insert((
        Key { id, provider },
        Value {
            title,
            tvshow_regex,
            season,
            episode_offset,
            episode_position: episode_position - 1,
            episode_regex,
            lang,
        },
    ))
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(()))
}
