// https://docs.joinmastodon.org/methods/followed_tags/#get

use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    key_value::Store,
    sqlite::{Connection, QueryResult, Value},
};
use std::collections::HashMap;

use crate::http_response::HttpResponse;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

// Returns: Array of Tag
// https://docs.joinmastodon.org/entities/Tag/
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    // TODO: implement this.
    // Return empty array for now.

    let tags: Vec<sparrow::mastodon::tag::Tag> = Vec::new();
    let return_body = serde_json::to_string(&tags).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(return_body)
        .build())
}
