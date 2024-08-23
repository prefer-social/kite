// https://docs.joinmastodon.org/methods/oauth/#token

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::{Method, Params, Request, Response};
use std::str;

use crate::http_response::HttpResponse;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );
    HttpResponse::not_found()
}

// Obtain a token
// `POST /oauth/token HTTP/1.1`

pub async fn post(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    let body = str::from_utf8(req.body()).unwrap();
    // let grant_type
    let a: Value = serde_json::from_str(body)?;
    let code = a["code"].as_str().unwrap();
    let client_id = a["client_id"].as_str().unwrap();
    let client_secret = a["client_secret"].as_str().unwrap();
    let redirect_uri = a["redirect_uri"].as_str().unwrap();
    let scope = a["scope"].as_str().unwrap();
    let grant_type = a["grant_type"].as_str().unwrap();

    let application =
        String::from_utf8(sparrow::cache::get(client_id).await?.unwrap())?;
    //let application = sparrow::mastodon::application::Application::get
    let b: Value = serde_json::from_str(application.as_str())?;

    if a["client_secret"] != b["client_secret"] {
        tracing::debug!("client_secret not matched");
        let a = r#"{
        "error": "invalid_client",
        "error_description": "Client authentication failed due to unknown client, no client authentication included, or unsupported authentication method."
        }"#;
        let json_val: serde_json::Value = serde_json::from_str(&a).unwrap();
        return Ok(Response::builder()
            .status(401)
            .header("Content-Type", "application/json")
            .body(json_val.to_string())
            .build());
    }

    // PASSED

    let ip = req
        .header("x-real-ip")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    let application_id = b["id"].as_str().unwrap().to_string();

    let application_from_db =
        sparrow::mastodon::application::Application::get_by_app_id(
            application_id.clone(),
        )
        .await?;
    let resource_owner_id = application_from_db.owner_id.unwrap();

    let token = sparrow::mastodon::token::Token::new(
        scope.to_string(),
        application_id,
        resource_owner_id,
        ip,
    )
    .await?;

    let json_val = serde_json::to_string(&token).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(json_val.to_string())
        .build())

    // Check code, client_id and client_secret are valid.

    // let app_temp_check = sparrow::db::Connection::builder()
    //     .await
    //     .execute(
    //         "SELECT * FROM app_temp WHERE client_id = ? AND client_secret = ?",
    //         &[
    //             SV::Text(client_id.to_string()),
    //             SV::Text(client_secret.to_string()),
    //         ],
    //     )
    //     .await;
    // let code_check = sparrow::db::Connection::builder()
    //     .await
    //     .execute(
    //         "SELECT * FROM user_authorization_code WHERE code = ?",
    //         &[SV::Text(code.to_string())],
    //     )
    //     .await;
    //
    // if app_temp_check.rows.len() > 0 && code_check.rows.len() > 0 {
    //     // ALL FINE.
    //     // CREATE TOKEN. INSERT INTO DB
    //
    //     let user_id = code_check
    //         .rows()
    //         .next()
    //         .unwrap()
    //         .get::<u32>("userId")
    //         .unwrap();
    //
    //     let token_type = "Bearer".to_string();
    //
    //     let token = sparrow::utils::create_token().await;
    //     let created_at = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();
    //
    //     let _ = sparrow::db::Connection::builder().await.execute("INSERT INTO auth_token (userId, token, token_type, scope, valid, issuedAt) VALUES (?, ?, ?, ?, ?, ?)", &[SV::Integer(user_id as i64), SV::Text(token.clone()), SV::Text(token_type.clone()), SV::Text(scope.to_string()), SV::Integer(1), SV::Integer(created_at as i64) ]).await;
    //
    //     let a = format!(
    //         r#"{{
    //         "access_token": "{token}",
    //         "token_type": "{token_type}",
    //         "scope": "{scope}",
    //         "created_at": {created_at}
    //         }}"#
    //     );
    //     let json_val: serde_json::Value = serde_json::from_str(&a).unwrap();
    //     return Ok(Response::builder()
    //         .status(200)
    //         .header("Content-Type", "application/json")
    //         .body(json_val.to_string())
    //         .build());
    // }
}
