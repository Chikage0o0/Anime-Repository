use actix_web::{delete, get, post, web, Responder, Result};
use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    data::unrecognized_videos::{get_all, VideoData},
    handler::get_handler_tx,
    model::nfo::ProviderKnown,
};

#[get("/api/unrecognized_videos")]
pub async fn get_unrecognized_videos_list() -> impl Responder {
    let list = get_all();
    web::Json(list)
}

#[get("/api/unrecognized_videos/refresh")]
pub async fn refresh_unrecognized_videos_list() -> Result<impl Responder> {
    get_handler_tx()
        .send(crate::handler::Command::ScanPendingVideosFolder)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(()))
}
#[delete("/api/unrecognized_videos/{path}")]
pub async fn delete_unrecognized_video_info(info: web::Path<PathBuf>) -> Result<impl Responder> {
    let path = info.into_inner();
    crate::service::unrecognized_videos::delete(path)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(()))
}

#[derive(Deserialize)]
pub struct UnrecognizedVideoInfo {
    path: PathBuf,
    r#type: String,
    id: String,
    provider: ProviderKnown,
    lang: String,
    title: String,
    season: u64,
    episode: u64,
}

#[post("/api/unrecognized_videos")]
pub async fn update_unrecognized_video_info(
    info: web::Json<UnrecognizedVideoInfo>,
) -> Result<impl Responder> {
    let UnrecognizedVideoInfo {
        path,
        r#type,
        id,
        provider,
        lang,
        title,
        season,
        episode,
    } = info.into_inner();
    if r#type == "movie" {
        crate::service::unrecognized_videos::insert(path, VideoData::Movie(id, provider, lang))
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    } else if r#type == "tvshow" {
        crate::service::unrecognized_videos::insert(
            path,
            VideoData::Tvshow(id, provider, lang, title, season, episode),
        )
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }
    Ok(web::Json(get_all()))
}
