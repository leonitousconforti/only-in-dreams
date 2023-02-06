use reqwest::header::{HeaderMap, HeaderValue, HOST, ORIGIN, REFERER};

const HOST_VALUE: &str = "ctf.acm.umn.edu";
const ORIGIN_VALUE: &str = "https://ctf.acm.umn.edu";
const REFERER_VALUE: &str = "https://ctf.acm.umn.edu/";

fn construct_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_static(HOST_VALUE));
    headers.insert(ORIGIN, HeaderValue::from_static(ORIGIN_VALUE));
    headers.insert(REFERER, HeaderValue::from_static(REFERER_VALUE));
    headers
}

pub fn get_nonce_for_page(
    client: &reqwest::blocking::Client,
    page: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .get("https://ctf.acm.umn.edu".to_owned() + page)
        .headers(construct_default_headers())
        .send()
        .unwrap()
        .text()
        .unwrap();

    let csrf_nonce_line = response
        .lines()
        .find(|&line| line.contains("'csrfNonce': "))
        .unwrap();

    let extract_nonce_from_line = regex::Regex::new("'csrfNonce':\\s\"([\\w]{64})\",").unwrap();

    let caps = extract_nonce_from_line.captures(csrf_nonce_line).unwrap();
    let csrf_nonce = caps.get(1).unwrap().as_str();

    Ok(csrf_nonce.into())
}
