use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

// https://docs.joinmastodon.org/methods/push/#get
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-forwarded-for").unwrap().as_str().unwrap()
    );

    return Ok(Response::builder()
        .status(200)
        .header("Content-Type", "Application/json")
        .body("{}")
        .build());
}

// https://docs.joinmastodon.org/methods/push/#create
pub async fn post(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-forwarded-for").unwrap().as_str().unwrap()
    );

    return Ok(Response::builder()
        .status(200)
        .header("Content-Type", "Application/json")
        .body("{}")
        .build());
}
