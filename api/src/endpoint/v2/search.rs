// https://docs.joinmastodon.org/methods/search/
//

use anyhow::{anyhow, Result};
use regex::Regex;
use serde_json::Value;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use std::str;
use std::{collections::HashMap, sync::Arc};
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return sparrow::http_response::HttpResponse::not_found().await,
    }
}

// GET /api/v2/search HTTP/1.1
// FOR NOW, only account search is avaiable
// Returns Search https://docs.joinmastodon.org/entities/Search/
pub async fn get(req: Request, params: Params) -> Result<Response> {
    debug!("Requeted -> GET /api/v2/search");

    // let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap()
    // {
    //     sparrow::auth::TokenAuth::InValid => {
    //         return sparrow::http_response::HttpResponse::unauthorized().await;
    //     }
    //     sparrow::auth::TokenAuth::TokenNotProvided => {
    //         return sparrow::http_response::HttpResponse::unauthorized().await;
    //     }
    //     sparrow::auth::TokenAuth::Valid(userid) => {
    //         Some(userid).unwrap() as i64
    //     }
    // };

    // https://docs.joinmastodon.org/methods/search/#query-parameters
    let path_and_query = req.path_and_query().unwrap();
    let quary: HashMap<_, _> = Url::parse(format!("data://text{path_and_query}").as_str())
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect();
    // query="/api/v2/search?q=apple&resolve=true"
    let search_term = quary.get("q").unwrap();

    // TODO: !!

    //let rr = get_account_info(search_term.clone()).await;
    //debug!("{rr:?}");

    let ai = get_account_info(search_term.clone()).await;

    if ai.is_err() {
        let no_search_results = r#"{
        "accounts": [],
        "statuses": [],
        "hashtags": []
      }"#;
        let json_val: serde_json::Value =
            serde_json::from_str(no_search_results).unwrap();

        let aq = json_val.to_string();
        debug!(aq);

        return Ok(Response::builder()
            .status(200)
            .header("Context-Type", "application/activity+json")
            .body(json_val.to_string())
            .build());
    }

    match ai.unwrap() {
        Some(a) => {
            return Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .body(a)
                .build());
        }
        None => {
            let no_search_results = r#"{
          "accounts": [],
          "statuses": [],
          "hashtags": []
        }"#;
            let json_val: serde_json::Value =
                serde_json::from_str(no_search_results).unwrap();

            let aq = json_val.to_string();
            debug!(aq);

            return Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .body(json_val.to_string())
                .build());
        }
    };
}

pub async fn get_account_info(mut term: String) -> Result<Option<String>> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    // TODO: This is very ugly. Implement a proper validation.
    if term.chars().next().unwrap() == '@' {
        term = sparrow::utils::get_actor_url_from_id(term[1..].to_string())
            .await
            .unwrap();
    } else if email_regex.is_match(term.as_str()) {
        term = sparrow::utils::get_actor_url_from_id(term).await.unwrap();
    } else if Url::parse(term.as_str()).is_err() {
        term = format!("https://{term}");
    }

    let request = Request::builder()
        .method(Method::Get)
        .header("Accept", "application/activity+json")
        .uri(term)
        .build();

    let response: Response = spin_sdk::http::send(request).await?;

    if *response.status() != 200u16 {
        return Ok(None);
    };

    let body = str::from_utf8(response.body()).unwrap();

    let acct: Value = serde_json::from_str(body)?;

    let preferred_username = acct
        .get::<&str>("preferredUsername")
        .unwrap()
        .as_str()
        .unwrap();
    let name = acct.get::<&str>("name").unwrap().as_str().unwrap();
    let summary = acct.get::<&str>("summary").unwrap().as_str().unwrap();
    let url = acct.get::<&str>("url").unwrap().as_str().unwrap();
    let published = acct.get::<&str>("published").unwrap().as_str().unwrap();
    let icon_url = match acct.get("icon") {
        Some(a) => a.get::<&str>("url").unwrap().as_str().unwrap().to_string(),
        None => "https://mstd.seungjin.net/avatars/original/missing.png"
            .to_string(),
    };
    let header_url = match acct.get("image") {
        Some(a) => a.get::<&str>("url").unwrap().as_str().unwrap().to_string(),
        None => "https://mstd.seungjin.net/avatars/original/missing.png"
            .to_string(),
    };

    let host = Url::parse(url).unwrap();
    let host_str = host.host_str().unwrap();
    let acct_str = format!("{preferred_username}@{host_str}");

    let r = format!(
        r#"{{
    "accounts": [
      {{
        "id": "{acct_str}",
        "username": "{name}",
        "acct": "{acct_str}",
        "display_name": "{preferred_username}",
        "locked": false,
        "bot": false,
        "created_at": "{published}",
        "note": "{summary}",
        "url": "{url}",
        "avatar": "{icon_url}",
        "avatar_static": "{icon_url}",
        "header": "{header_url}",
        "header_static": "{header_url}",
        "followers_count": 547,
        "following_count": 404,
        "statuses_count": 28468,
        "last_status_at": "2019-11-17",
        "emojis": [
        ],
        "fields": [
        ]
      }}        
    ],
    "statuses": [],
    "hashtags": []
    }}"#
    );

    debug!(r);

    //Ok(serde_json::from_str(r.as_str())?)
    Ok(Some(r))
}

pub async fn valid_url(a: String) -> bool {
    let a = Url::parse(a.as_str());
    a.is_ok()
}
