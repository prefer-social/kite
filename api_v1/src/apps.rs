use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value},
};
use std::{collections::HashMap, thread::spawn};
use tracing::debug;
use url::Url;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => not_found(req, params).await,
    }
}

// TODO: create application https://docs.joinmastodon.org/methods/apps/#create
//
pub async fn post(req: Request, _params: Params) -> Result<Response> {
    debug!("requested --> /api/v1/apps");

    let client_id = random_string(44).await;
    let client_secret = random_string(44).await;
    let vapid_key = random_string(44).await;

    // Add above value to app_temp table
    // before that delete any row older than 1 hour.

    let _qr = crates::sparrow::db::Connection::builder()
        .await
        .execute(
            "DELETE FROM app_temp WHERE app_temp.createdAt <= date('now', '-1 hour')",
            &[],
        )
        .await;

    let _qr = sparrow::db::Connection::builder()
        .await
        .execute(
            "INSERT INTO app_temp(client_id, client_secret, vapid_key) VALUES( ?, ?, ?)",
            &[
                Value::Text(client_id.clone()),
                Value::Text(client_secret.clone()),
                Value::Text(vapid_key.clone()),
            ],
        )
        .await;

    let a = req.uri();
    let b = Url::parse(a).unwrap();
    let c: HashMap<_, _> = b.query_pairs().into_owned().collect();

    let client_name = c
        .get("client_name")
        .unwrap_or(&"unknown".to_string())
        .to_string();
    let redirect_uris = c
        .get("redirect_uris")
        .unwrap_or(&"urn:ietf:wg:oauth:2.0:oob".to_string())
        .to_string();
    let scopes = c
        .get("scopes")
        .unwrap_or(&"read write push".to_string())
        .to_string();
    let website = c
        .get("website")
        .unwrap_or(&"unknown".to_string())
        .to_string();

    let a1 = format!(
        r#"{{
      "id": "{client_id}",
      "name": "rasarsars",
      "website": "{website}",
      "redirect_uri": "{redirect_uris}",
      "client_id": "{client_id}",
      "client_secret": "{client_secret}",
      "vapid_key": "{vapid_key}"}}
      "#
    );

    let json_val: serde_json::Value = serde_json::from_str(&a1).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())
}

struct Application {
    id: String,
    name: String,
    website: Option<String>,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
    vapid_key: String,
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
        .body("arsars")
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
