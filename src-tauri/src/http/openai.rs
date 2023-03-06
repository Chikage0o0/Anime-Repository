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
    pub fn new() -> Self {
        Self {
            client: Client::get(),
        }
    }

    pub async fn match_file(&self, filename: &str) -> reqwest::Result<(String, StatusCode)> {
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
