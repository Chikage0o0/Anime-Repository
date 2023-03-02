use reqwest::{header::HeaderMap, StatusCode};
use std::ops::Deref;

use super::client::Client;
const KEY: &str = env!("TMDB_KEY");

pub struct TMDBClient {
    client: super::client::Client,
}

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json;charset=utf-8".parse().unwrap(),
    );
    headers.insert(reqwest::header::AUTHORIZATION, KEY.parse().unwrap());
    headers
}

impl Deref for TMDBClient {
    type Target = super::client::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl Default for TMDBClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TMDBClient {
    pub fn new() -> Self {
        Self {
            client: Client::get(),
        }
    }
    pub async fn get_movie_info(
        &self,
        id: &str,
        lang: &str,
    ) -> reqwest::Result<(String, StatusCode)> {
        let url = format!("https://api.themoviedb.org/3/movie/{}?language={}&append_to_response=images,credits&include_image_language={}",id,lang,&lang[0..2]);
        self.get_string(url, get_header()).await
    }

    pub async fn get_tvshow_info(
        &self,
        id: &str,
        lang: &str,
    ) -> reqwest::Result<(String, StatusCode)> {
        let url = format!("https://api.themoviedb.org/3/tv/{}?language={}&append_to_response=images,aggregate_credits&include_image_language={}",id,lang,&lang[0..2]);
        self.get_string(url, get_header()).await
    }

    pub async fn get_tv_episode_info(
        &self,
        id: &str,
        season: u64,
        episode: u64,
        lang: &str,
    ) -> reqwest::Result<(String, StatusCode)> {
        let url = format!(
            "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}?language={}",
            id, season, episode, lang
        );
        self.get_string(url, get_header()).await
    }

    pub async fn search_movie(
        &self,
        key: &str,
        lang: &str,
        page: u64,
    ) -> reqwest::Result<(String, StatusCode)> {
        let url = format!(
            "https://api.themoviedb.org/3/search/movie?query={}&language={}&page={}&include_adult=true",
            key, lang, page
        );
        self.get_string(url, get_header()).await
    }

    pub async fn search_tvshows(
        &self,
        key: &str,
        lang: &str,
        page: u64,
    ) -> reqwest::Result<(String, StatusCode)> {
        let url = format!(
            "https://api.themoviedb.org/3/search/tv?query={}&language={}&page={}&include_adult=true",
            key, lang, page
        );
        self.get_string(url, get_header()).await
    }
}
