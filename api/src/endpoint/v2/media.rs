//! Media API methods
//!
//! Mastodon doc: <https://docs.joinmastodon.org/methods/media/>

use anyhow::{Error, Result};
use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;

// https://docs.joinmastodon.org/methods/media/#get
pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return HttpResponse::not_found(),
    }
}

// https://docs.joinmastodon.org/methods/media/#get
pub async fn get(_req: Request, params: Params) -> Result<Response> {
    tracing::debug!("requested -> GET /api/v1/media");

    let media_id = params.get("id").unwrap();
    tracing::debug!(media_id);

    let medias = sparrow::mastodon::media_attachment::MediaAttachment::get(
        media_id.to_string(),
    )
    .await?;

    let body = serde_json::to_string(&medias)?;

    tracing::debug!(body);

    return Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(body)
        .build());
}
