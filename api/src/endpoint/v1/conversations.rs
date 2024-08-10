// https://docs.joinmastodon.org/methods/conversations/#get
// Array of Conversation
// https://docs.joinmastodon.org/entities/Conversation/

use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};
use tracing::debug;

use crate::http_response::HttpResponse;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("[]")
        .build())
}
