use std::path::Path;

use reqwest::StatusCode;

use super::search::search;
use crate::http::openai::OPENAIClient;
use crate::model::setting::Setting;
use crate::service::nfo;

async fn get_match_result<P: AsRef<Path>>(
    file_name: P,
) -> Result<(String, Option<u64>, Option<u64>), Box<dyn std::error::Error>> {
    let client = OPENAIClient::new();

    let prompt = file_name.as_ref().file_stem().unwrap().to_str().unwrap();
    let result = client.match_file(prompt).await;
    log::debug!("OpenAI result: {:?}", result);
    let (body, status) = result?;
    if let StatusCode::OK = status {
        let response = serde_json::from_str::<serde_json::Value>(&body)?;
        let match_result = response
            .get("choices")
            .and_then(|f| f.as_array())
            .and_then(|f| f.get(0))
            .and_then(|f| f.get("message"))
            .and_then(|f| f.get("content"))
            .and_then(|f| f.as_str());

        if let Some(match_result) = match_result {
            let match_result = serde_json::from_str::<serde_json::Value>(match_result)?;
            if let Some(title) = match_result.get("title").and_then(|f| f.as_str()) {
                let mut season = match_result.get("season").and_then(|f| f.as_u64());
                let episode = match_result.get("episode").and_then(|f| f.as_u64());
                if season == None && episode != None {
                    season = Some(1);
                }
                return Ok((title.to_string(), season, episode));
            }
        }
    }

    Err("OpenAI parsing failure".into())
}

pub async fn process<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let (title, mut season, episode) = get_match_result(&path).await?;
    let r#type = match episode {
        Some(_) => "tvshow",
        None => "movie",
    };

    if episode.is_some() && season.is_none() {
        season = Some(1);
    }

    let provider = Setting::get_default_provider();
    let search_result = search(&title, provider, r#type).await?;
    let id: &str;
    let title: &str;

    match search_result.get(0) {
        Some(result) => {
            id = &result.id;
            title = &result.title;
        }
        None => {
            return Err("No result found".into());
        }
    }
    let lang = &Setting::get_default_lang();

    match r#type {
        "tvshow" => {
            nfo::tvshow::process(
                id,
                provider,
                title,
                lang,
                season.unwrap(),
                episode.unwrap(),
                &path,
            )
            .await?
        }
        "movie" => nfo::movie::process(id, provider, lang, &path).await?,
        _ => return Err("Unknown type".into()),
    };
    Ok(())
}
