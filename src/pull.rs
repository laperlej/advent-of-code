use reqwest;
use std::env;

async fn fetch_input(year: u32, day: u32, cookie: String) {
    let client = reqwest::Client::new();
    let header_map = get_request_headers(cookie);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let resp = client.get(url).headers(header_map).send().await.unwrap();
    let body = resp.text().await.unwrap();
    print!("{}", body);
}

pub fn get_request_headers(cookie: String) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str(format!{"session={}", cookie}.as_str()).unwrap(),
    );
    headers
}

pub async fn pull(year: u32, day:u32) {
    let cookie = env::var("AOC_COOKIE").unwrap();
    fetch_input(year, day, cookie).await;
}
