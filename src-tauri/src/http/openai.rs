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
        let url = format!(
            "https://{}/v1/chat/completions",
            Setting::get_openai_domain()
        );

        let body = json!({
          "model": "gpt-3.5-turbo",
          "messages": [
            {"role": "system", "content": "I will give you a film/tvshow/anime file name.You need to return a Json.\nPay attention to the correct identification of the film/tvshow/anime name.\n{\"title\":string,\"season\":number|null,\"episode\":number|null}"},
            {"role": "user", "content": filename}
          ],
          "max_tokens": 100,
          "temperature": 0.1
        });
        self.post_string(url, get_header(), body.to_string()).await
    }
}
