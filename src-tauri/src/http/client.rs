use std::sync::Mutex;

use crate::model::setting::Setting;
use once_cell::sync::Lazy;
use reqwest::{header::HeaderMap, Result, StatusCode};

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
}

static HTTP_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| Mutex::new(Client::new()));

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    fn new() -> Self {
        let mut builder = reqwest::ClientBuilder::new();
        if let Some(proxy) = Setting::get_proxy() {
            if let Ok(proxy) = reqwest::Proxy::all(proxy) {
                builder = builder.proxy(proxy);
            } else {
                todo!()
            }
        }
        builder = builder.connect_timeout(std::time::Duration::from_secs(3));
        Self {
            client: builder.build().unwrap(),
        }
    }

    pub fn get() -> Self {
        HTTP_CLIENT.lock().unwrap().clone()
    }

    pub fn rebuild() {
        let mut c = HTTP_CLIENT.lock().unwrap();
        *c = Client::new();
        log::info!("HTTP client set {:?}", &*c);
    }

    pub async fn get_string(
        &self,
        url: String,
        headers: HeaderMap,
    ) -> Result<(String, StatusCode)> {
        let res = self.client.get(&url).headers(headers).send().await?;
        let status = res.status();
        let text = res.text().await?;
        Ok((text, status))
    }

    pub async fn get_bytes(
        &self,
        url: String,
        headers: HeaderMap,
    ) -> Result<(Vec<u8>, StatusCode)> {
        let res = self.client.get(&url).headers(headers).send().await?;
        let status = res.status();
        let bytes = res.bytes().await?;
        Ok((bytes.to_vec(), status))
    }

    pub async fn post_string(
        &self,
        url: String,
        headers: HeaderMap,
        body: String,
    ) -> Result<(String, StatusCode)> {
        let res = self
            .client
            .post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        let status = res.status();
        let text = res.text().await?;
        Ok((text, status))
    }
}
