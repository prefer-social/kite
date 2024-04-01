// https://docs.joinmastodon.org/methods/oauth/#token

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::{Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder().status(404).build())
}

// Obtain a token
// `POST /oauth/token HTTP/1.1`

pub async fn post(req: Request, _params: Params) -> Result<Response> {
    debug!("<--------- POST /oauth/token --------->");

    let body = str::from_utf8(req.body()).unwrap();
    let v: Value = serde_json::from_str(body)?;
    // Object {"client_id": String("L6H4mgUdXa84735u619byJ54RgbtHDGka8EHwEb8iSwl"), "client_secret": String("JpHJHhn4R2Q29yOxp4IwCRh7vAb36X0pzRSw8m56CzD7"), "code": String("a82RXtyIjafT9iKUeg4FhUzSMxnRvLZBkEvLE8ecdjs15HnTEU9GHmKAXQJvgsio"), "grant_type": String("authorization_code"), "redirect_uri": String("icecubesapp://"), "scope": String("read write follow push")}

    // let grant_type
    let code = v["code"].as_str().unwrap();
    let client_id = v["client_id"].as_str().unwrap();
    let client_secret = v["client_secret"].as_str().unwrap();
    let redirect_uri = v["redirect_uri"].as_str().unwrap();
    let scope = v["scope"].as_str().unwrap();
    let grant_type = v["grant_type"].as_str().unwrap();

    // Check code, client_id and client_secret are valid.

    let app_temp_check = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT * FROM app_temp WHERE client_id = ? AND client_secret = ?",
            &[
                SV::Text(client_id.to_string()),
                SV::Text(client_secret.to_string()),
            ],
        )
        .await;
    let code_check = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT * FROM user_authorization_code WHERE code = ?",
            &[SV::Text(code.to_string())],
        )
        .await;

    if app_temp_check.rows.len() > 0 && code_check.rows.len() > 0 {
        // ALL FINE.
        // CREATE TOKEN. INSERT INTO DB

        let user_id = code_check
            .rows()
            .next()
            .unwrap()
            .get::<u32>("userId")
            .unwrap();

        let token_type = "Bearer".to_string();

        let token = sparrow::utils::create_token().await;
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let _ = sparrow::db::Connection::builder().await.execute("INSERT INTO auth_token (userId, token, token_type, scope, valid, issuedAt) VALUES (?, ?, ?, ?, ?, ?)", &[SV::Integer(user_id as i64), SV::Text(token.clone()), SV::Text(token_type.clone()), SV::Text(scope.to_string()), SV::Integer(1), SV::Integer(created_at as i64) ]).await;

        let a = format!(
            r#"{{
            "access_token": "{token}",
            "token_type": "{token_type}",
            "scope": "{scope}",
            "created_at": {created_at}
            }}"#
        );
        let json_val: serde_json::Value = serde_json::from_str(&a).unwrap();
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json_val.to_string())
            .build());
    }

    let a = r#"{
        "error": "invalid_client",
        "error_description": "Client authentication failed due to unknown client, no client authentication included, or unsupported authentication method."
      }"#;
    let json_val: serde_json::Value = serde_json::from_str(&a).unwrap();

    Ok(Response::builder()
        .status(401)
        .header("Content-Type", "application/json")
        .body(json_val.to_string())
        .build())
}
