use crate::model::setting::Setting;
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Result, StatusCode};

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

pub async fn get_string(url: String, headers: HeaderMap) -> Result<(String, StatusCode)> {
    let res = HTTP_CLIENT.get(&url).headers(headers).send().await?;
    let status = res.status();
    let text = res.text().await?;
    Ok((text, status))
}

pub async fn get_bytes(url: String, headers: HeaderMap) -> Result<(Vec<u8>, StatusCode)> {
    let res = HTTP_CLIENT.get(&url).headers(headers).send().await?;
    let status = res.status();
    let bytes = res.bytes().await?;
    Ok((bytes.to_vec(), status))
}
