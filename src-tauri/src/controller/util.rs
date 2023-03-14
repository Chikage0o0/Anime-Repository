use crate::{model::nfo::ProviderKnown, service::subscribe};
use actix_web::{get, web, Responder, Result};

#[get("/api/get_title/{id}/{provider}/{lang}/{type}")]
pub async fn get_title(
    info: web::Path<(String, ProviderKnown, String, String)>,
) -> Result<impl Responder> {
    let (id, provider, lang, r#type) = info.into_inner();
    let title = subscribe::get_title(&id, provider, &lang, &r#type)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(title))
}
