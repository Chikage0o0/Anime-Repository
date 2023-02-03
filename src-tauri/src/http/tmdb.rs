use reqwest::header::HeaderMap;
use std::future::Future;

use super::client;
const KEY:&str="Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhNmI3YjFiOWQwNjk2MGZlYmQ0NzcwYzU3MTkyYjQ4MyIsInN1YiI6IjYzYjcwOWMwZjQ0ZjI3MDBiZGRlNWE5MyIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.vTPT0JlthLSrd6ZhJYKa84HoL7wFm9K1q6xetWfp458";

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json;charset=utf-8".parse().unwrap(),
    );
    headers.insert(reqwest::header::AUTHORIZATION, KEY.parse().unwrap());
    headers
}

pub fn get_movie_info(id: usize, lang: String) -> impl Future<Output = String> {
    let url = format!("https://api.themoviedb.org/3/movie/{}?language={}&append_to_response=images,credits&include_image_language={}",id,lang,&lang[0..2]);
    client::get_string(url, get_header())
}

pub fn get_tvshow_info(id: usize, lang: String) -> impl Future<Output = String> {
    let url = format!("https://api.themoviedb.org/3/tv/{}?language={}&append_to_response=images,aggregate_credits&include_image_language={}",id,lang,&lang[0..2]);
    client::get_string(url, get_header())
}

pub fn get_tv_episode_info(
    id: usize,
    season: usize,
    episode: usize,
    lang: String,
) -> impl Future<Output = String> {
    let url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}?language={}",
        id, season, episode, lang
    );
    client::get_string(url, get_header())
}

pub fn search_movie(key: String, lang: String, page: usize) -> impl Future<Output = String> {
    let url = format!(
        "https://api.themoviedb.org/3/search/movie?query={}&language={}&page={}&include_adult=true",
        key, lang, page
    );
    client::get_string(url, get_header())
}

pub fn search_tvshows(key: String, lang: String, page: usize) -> impl Future<Output = String> {
    let url = format!(
        "https://api.themoviedb.org/3/search/tv?query={}&language={}&page={}&include_adult=true",
        key, lang, page
    );
    client::get_string(url, get_header())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_movie_info() {
        use tauri::async_runtime::block_on;
        let info = block_on(get_movie_info(655431, String::from("zh-CN")));
        println!("{:#?}", info);
    }
}