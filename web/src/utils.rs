use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use spin_sdk::http::{Params, Request, Response};
use std::collections::HashMap;
use url::{ParseError, Url};

/// Just generate random url as object id. In a real project, you probably want to use
/// an url which contains the database id for easy retrieval (or store the random id in db).
pub async fn generate_object_id(domain: &str) -> Result<Url, ParseError> {
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    Url::parse(&format!("https://{}/objects/{}", domain, id))
}

pub async fn get_http_headers_map(req: &Request) {
    let headers = req.headers();
    for header in headers {
        println!("{header:?}");
    }
}

pub async fn get_req_query_hash(req: &Request) -> HashMap<String, String> {
    let parsed_url = Url::parse(req.uri()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    hash_query
}

pub async fn not_found(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder().status(404).build())
}

pub async fn unauthorized() -> Result<Response> {
    let json_str = r#"{
        "error": "invalid_signature",
        "error_description": "The signature in the request is not valid."
    }"#;
    let json_val: serde_json::Value = serde_json::from_str(json_str).unwrap();

    Ok(Response::builder()
        .status(401)
        .header("Content-Type", "application/json")
        .body(json_val.to_string())
        .build())
}

pub async fn get_current_time_in_RFC_1123() -> String {
    use chrono::{DateTime, Utc};
    let current_time: DateTime<Utc> = Utc::now();
    current_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}
