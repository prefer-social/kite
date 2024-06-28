// GET /api/v1/timelines/home HTTP/1.1
// https://docs.joinmastodon.org/methods/timelines/#home
// Returns: Array of Status

use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
};

use sparrow::mastodon::status::Status;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

// https://docs.joinmastodon.org/methods/timelines/#home
// Returns: Array of Status (https://docs.joinmastodon.org/entities/Status/)
// OAuth: User + read:statuses
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    // TODO: implement this.
    // View statuses from followed users and hashtags.
    // Return empty array for now.

    //let home_timeline: Vec<sparrow::mastodon::status::Status> = Vec::new();
    //let return_body = home_timeline.into();

    let return_body = "[]".to_string();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(return_body)
        .build())
}
