use anyhow::{bail, Result};

use spin_sdk::http::{Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use std::str;

use sparrow::mastodon::strt::media::{MediaAttachment, MediaType};

// https://docs.joinmastodon.org/methods/media/#get
pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return crate::http_responses::notfound().await,
    }
}

// https://docs.joinmastodon.org/methods/media/#get
pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("Requeted -> GET /api/v1/media");

    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap() {
        sparrow::auth::TokenAuth::InValid => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => Some(userid).unwrap() as i64,
    };

    let media_id = params.get("id").unwrap();
    tracing::debug!(media_id);

    let media = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT * FROM media_attachement WHERE id = ?",
            &[SV::Text(media_id.to_string())],
        )
        .await;

    let row = media.rows().next().unwrap();

    let media_attachement = MediaAttachment {
        id: row.get::<&str>("id").unwrap().to_string(),
        kind: MediaType::Image,
        url: Some(row.get::<&str>("previewUrl").unwrap().to_string()),
        preview_url: Some(row.get::<&str>("previewUrl").unwrap().to_string()),
        remote_url: None,
        text_url: None,
        meta: None,
        description: None,
        blurhash: None,
        created_at: None,
        updated_at: None,
    };

    let body = serde_json::to_string(&media_attachement)?;

    tracing::debug!(body);

    return Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(body)
        .build());
}
