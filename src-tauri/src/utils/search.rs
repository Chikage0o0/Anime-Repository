use reqwest::StatusCode;

use crate::{
    http::tmdb::TMDBClient,
    model::{nfo::ProviderKnown, setting::Setting},
};

pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub year: Option<String>,
    pub poster: Option<String>,
    pub overview: Option<String>,
}

pub async fn search(
    title: &str,
    provider: ProviderKnown,
    r#type: super::r#Type,
) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    log::info!("Searching for {} on {:?}", title, &provider);
    match provider {
        ProviderKnown::TMDB => {
            let client = TMDBClient::new();
            let result: (String, StatusCode);
            match r#type {
                super::r#Type::Tvshow => {
                    result = client
                        .search_tvshows(&title, &Setting::get_default_lang(), 1)
                        .await?;
                }
                super::r#Type::Movie => {
                    result = client
                        .search_movie(&title, &Setting::get_default_lang(), 1)
                        .await?;
                }
            }
            let (body, status) = result;
            if let StatusCode::OK = status {
                let response = serde_json::from_str::<serde_json::Value>(&body)?;
                let result = response.get("results").and_then(|f| f.as_array());

                if let Some(result) = result {
                    let result = result
                        .iter()
                        .map(|f| {
                            let id = f.get("id").and_then(|f| f.as_u64()).unwrap().to_string();
                            let title = f
                                .get("name")
                                .and_then(|f| f.as_str())
                                .unwrap_or_else(|| f.get("title").and_then(|f| f.as_str()).unwrap())
                                .to_string();
                            let year = f
                                .get("first_air_date")
                                .and_then(|f| f.as_str())
                                .or_else(|| f.get("release_date").and_then(|f| f.as_str()))
                                .map(|f| f.split('-').next().unwrap().to_string());
                            let poster = f
                                .get("poster_path")
                                .and_then(|f| f.as_str())
                                .map(|f| format!("https://image.tmdb.org/t/p/w500/{}", f));
                            let overview = f
                                .get("overview")
                                .and_then(|f| f.as_str())
                                .map(|f| f.to_string());
                            SearchResult {
                                id,
                                title,
                                year,
                                poster,
                                overview,
                            }
                        })
                        .collect::<Vec<SearchResult>>();
                    return Ok(result);
                }
            };
        }
        ProviderKnown::IMDB => todo!(),
    }
    Err("Search failure".into())
}
