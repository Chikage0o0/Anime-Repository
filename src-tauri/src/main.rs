mod controller;
mod data;
mod handler;
mod http;
mod model;
mod service;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../dist/assets"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "../dist"]
struct Index;

fn handle_embedded_assets(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn handle_embedded_index(path: &str) -> HttpResponse {
    match Index::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/{_:.*}")]
async fn index() -> impl Responder {
    handle_embedded_index("index.html")
}

#[actix_web::get("/assets/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_assets(path.as_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();
    handler::run();
    HttpServer::new(|| {
        App::new()
            .service(dist)
            .service(controller::get_setting)
            .service(controller::save_setting)
            .service(controller::delete_subscribe_rule)
            .service(controller::get_subscribe_rule)
            .service(controller::get_subscribe_rules)
            .service(controller::insert_subscribe_rule)
            .service(controller::get_title)
            .service(controller::get_unrecognized_videos_list)
            .service(controller::refresh_unrecognized_videos_list)
            .service(controller::delete_unrecognized_video_info)
            .service(controller::update_unrecognized_video_info)
            .service(index)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn init_log() {
    // set env debug
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}
