use actix_web::{get, put, web, Responder, Result};

use crate::model::setting::Setting;

#[get("/api/setting")]
pub async fn get_setting() -> impl Responder {
    web::Json(Setting::get())
}

#[put("/api/setting")]
pub async fn save_setting(setting: web::Json<Setting>) -> Result<impl Responder> {
    Setting::apply(setting.0).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(web::Json(()))
}
