// https://docs.joinmastodon.org/methods/apps/

use anyhow::Result;
use sparrow::utils::random_string;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    key_value::Store,
    sqlite::{Connection, QueryResult, Value},
};
use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

// TODO: create application https://docs.joinmastodon.org/methods/apps/#create
//
pub async fn post(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let client_id = uuid::Uuid::now_v7().to_string();
    let client_secret = random_string(44).await;
    let vapid_key = random_string(44).await;

    let url = Url::parse(req.uri()).unwrap();
    let query_hashmap: HashMap<_, _> =
        url.query_pairs().into_owned().collect();

    let client_name = query_hashmap
        .get("client_name")
        .unwrap_or(&"unknown".to_string())
        .to_string();
    let redirect_uris = query_hashmap
        .get("redirect_uris")
        .unwrap_or(&"urn:ietf:wg:oauth:2.0:oob".to_string())
        .to_string();
    let scopes = query_hashmap
        .get("scopes")
        .unwrap_or(&"read write push".to_string())
        .to_string();
    let website = query_hashmap
        .get("website")
        .unwrap_or(&"unknown".to_string())
        .to_string();

    let application = sparrow::mastodon::application::Application {
        uid: Uuid::now_v7().to_string(),
        name: client_name,
        website: Some(website),
        redirect_uri: Some(redirect_uris),
        client_id: Some(client_id),
        client_secret: Some(client_secret),
        vapid_key: Some(vapid_key),
        owner_id: None,
    };

    let app_json_string: String = serde_json::to_string(&application).unwrap();
    let hour_from_now =
        chrono::offset::Utc::now().add(Duration::from_secs(60 * 60));
    sparrow::cache::set_with_exp(
        application.client_id.clone().unwrap().as_str(),
        app_json_string.as_bytes(),
        hour_from_now,
    )
    .await?;

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(app_json_string)
        .build())
}
