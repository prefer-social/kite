//! Search for content in accounts, statuses and hashtags.  
//!
//! `GET /api/v2/search HTTP/1.1`  
//! Returns: Search
//! OAuth: Public (without resolve or offset), or User token + read:search  
//! Mastodon doc: <https://docs.joinmastodon.org/methods/search/>

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sparrow::mastodon::account::Account as MAccount;
use spin_sdk::http::{Method, Params, Request, Response};
use std::collections::HashMap;
use std::str;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return sparrow::http_response::HttpResponse::not_found().await,
    }
}

/// Search for content in accounts, statuses and hashtags.  
///
/// `GET /api/v2/search HTTP/1.1`  
/// Returns: Search
/// OAuth: Public (without resolve or offset), or User token + read:search  
/// Mastodon doc: <https://docs.joinmastodon.org/methods/search/>
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    //let auth = req.header("Authorization").unwrap().as_str().unwrap();
    //tracing::debug!(auth);

    // https://docs.joinmastodon.org/methods/search/#query-parameters
    let path_and_query = req.path_and_query().unwrap();
    let quary: HashMap<_, _> =
        Url::parse(format!("data://text{path_and_query}").as_str())
            .unwrap()
            .query_pairs()
            .into_owned()
            .collect();
    // query="/api/v2/search?q=apple&resolve=true"
    let search_term = quary.get("q").unwrap();

    let accounts_search_result = MAccount::search(search_term).await;
    let statuses_search_result =
        sparrow::mastodon::status::Status::search(search_term).await;
    let hashtags_search_result =
        sparrow::mastodon::tag::Tag::search(search_term).await;

    #[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    struct SearchResult {
        accounts: Vec<sparrow::mastodon::account::Account>,
        statuses: Vec<sparrow::mastodon::status::Status>,
        hashtags: Vec<sparrow::mastodon::tag::Tag>,
    }

    let search_result = SearchResult {
        accounts: accounts_search_result.unwrap(),
        statuses: statuses_search_result.unwrap(),
        hashtags: hashtags_search_result.unwrap(),
    };

    let search_result = serde_json::to_string(&search_result).unwrap();

    tracing::debug!(search_result);

    return Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(search_result)
        .build());
}

pub async fn valid_url(a: String) -> bool {
    let a = Url::parse(a.as_str());
    a.is_ok()
}
