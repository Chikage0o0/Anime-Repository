use crate::model::setting::Setting;
use reqwest::{header::HeaderMap, Result, StatusCode};

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        let mut builder = reqwest::ClientBuilder::new();
        if let Some(proxy) = Setting::get_proxy() {
            if let Ok(proxy) = reqwest::Proxy::all(proxy) {
                builder = builder.proxy(proxy);
            } else {
                todo!()
            }
        }
        Self {
            client: builder.build().unwrap(),
        }
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
