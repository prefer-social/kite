// https://docs.joinmastodon.org/methods/apps/

mod verify_credentials;

use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value},
    key_value::Store,
};
use std::{collections::HashMap};

use url::Url;

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
    tracing::debug!("requested --> /api/v1/apps");

    let client_id = uuid::Uuid::now_v7().to_string();
    let client_secret = random_string(44).await;
    let vapid_key = random_string(44).await;

    let url = Url::parse(req.uri()).unwrap();
    let query_hashmap: HashMap<_, _> = url.query_pairs().into_owned().collect();

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
        id: None,
        name: client_name,
        website: Some(website),
        redirect_uri: Some(redirect_uris),
        client_id: Some(client_id),
        client_secret: Some(client_secret),
        vapid_key: Some(vapid_key),
    };

    let app = sparrow::mastodon::application::new(application).await.unwrap();

    let app_json_string: String = serde_json::to_string(&app).unwrap();
    tracing::debug!("{}", app_json_string);

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(app_json_string)
        .build())
}


pub async fn random_string(length: u8) -> String {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    let mut rng = StdRng::from_entropy();
    let random_string: String = (0..length)
        .map(|_| match rng.gen_range(0..=2) {
            0 => char::from(rng.gen_range(b'0'..=b'9') as char),
            1 => char::from(rng.gen_range(b'A'..=b'Z') as char),
            _ => char::from(rng.gen_range(b'a'..=b'z') as char),
        })
        .collect();
    random_string
}
