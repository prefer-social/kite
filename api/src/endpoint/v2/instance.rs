use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;

// https://docs.joinmastodon.org/methods/instance/
// Returns: MediaAttachment, but without a URL
pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return HttpResponse::not_found(),
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    //let a = sparrow::mastodon::instance::Instance::get().await;

    //tracing::debug!("{:?}", a);

    Ok(Response::builder()
        .status(404)
        .header("Context-Type", "application/activity+json")
        .build())
}
