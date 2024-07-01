// https://docs.joinmastodon.org/methods/notifications/#get
// Returns: Array of Notification
// https://docs.joinmastodon.org/entities/Notification/

use anyhow::Result;
use sparrow::http_response::HttpResponse;
use spin_sdk::http::{Method, Params, Request, Response,};
use tracing::debug;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("requested -> {} {}", req.method().to_string(), req.path_and_query().unwrap());

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("[]")
        .build())

}