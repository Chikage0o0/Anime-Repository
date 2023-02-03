use crate::model::setting::Setting;
use lazy_static::lazy_static;
use reqwest::{self, header::HeaderMap};

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = new_client();
}

fn new_client() -> reqwest::Client {
    let mut builder = reqwest::ClientBuilder::new();
    if let Some(proxy) = Setting::get_proxy() {
        if let Ok(proxy) = reqwest::Proxy::all(proxy) {
            builder = builder.proxy(proxy);
        } else {
            //TODO: 向前端发送错误
            todo!()
        }
    }
    builder.build().unwrap()
}

//TODO: 错误处理

pub async fn get_string(url: String, headers: HeaderMap) -> String {
    let body = HTTP_CLIENT
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    body
}

pub async fn get_bytes(url: String, headers: HeaderMap) -> Vec<u8> {
    let body = HTTP_CLIENT
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    body.to_vec()
}
