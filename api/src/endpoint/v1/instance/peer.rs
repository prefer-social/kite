// https://docs.joinmastodon.org/methods/instance/#peers
use crate::http_response::HttpResponse;
use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

// https://docs.joinmastodon.org/methods/instance/#peers
// Return: Array of String
// OAuth: Public
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    // TODO: implement this.
    // Return empty array for now.

    let content: Vec<String> = Vec::new();
    let return_body = serde_json::to_string(&content).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(return_body)
        .build())
}
