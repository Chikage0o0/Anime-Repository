use reqwest::{header::HeaderMap, StatusCode};
use serde_json::json;
use std::ops::Deref;

use crate::model::setting::Setting;

use super::client::Client;

pub struct OPENAIClient {
    client: super::client::Client,
}

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json;charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        ("Bearer ".to_string() + &Setting::get_openai_key().unwrap_or_default())
            .parse()
            .unwrap(),
    );
    headers
}

impl Deref for OPENAIClient {
    type Target = super::client::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Default for OPENAIClient {
    fn default() -> Self {
        Self::new()
    }
}

impl OPENAIClient {
    fn new() -> Self {
        Self {
            client: Client::get(),
        }
    }

    pub async fn get_completion(&self, filename: &str) -> reqwest::Result<(String, StatusCode)> {
        let url = "https://api.openai.com/v1/chat/completions".to_string();

        let body = json!({
          "model": "gpt-3.5-turbo",
          "messages": [
            {"role": "system", "content": "You are an api.I will give you a movie/tvshow file name.You need to generate a Json.\nFormat of json:{\"title\":string,\"season\":number|null,\"episode\":number|null}"},
            {"role": "user", "content": filename}
          ],
          "max_tokens": 100,
          "temperature": 0.1
        });
        self.post_string(url, get_header(), body.to_string()).await
    }
}

#[cfg(test)]
mod test {
    use tauri::async_runtime::block_on;

    use super::*;
    #[test]
    fn test_get_completion() {
        let client = OPENAIClient::new();
        let prompt = "[Ohys-Raws] Ooyuki Umi no Kaina - 08 (AT-X 1280x720 x264 AAC).mp4";
        let result = block_on(client.get_completion(prompt));
        assert!(result.is_ok());
        let (body, status) = result.unwrap();
        dbg!(status);
        let response = serde_json::from_str::<serde_json::Value>(&body).unwrap();
        let s = response.get("choices").unwrap().as_array().unwrap()[0]
            .get("message")
            .unwrap()
            .get("content")
            .unwrap()
            .as_str()
            .unwrap();
        dbg!(serde_json::from_str::<serde_json::Value>(s).unwrap());
    }
}
