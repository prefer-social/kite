// https://docs.joinmastodon.org/methods/lists/#get
use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value},
};
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get() {
    todo!();
}

// https://docs.joinmastodon.org/methods/lists/#create
pub async fn post(_req: Request, _params: Params) -> Result<Response> {
    todo!();
}

pub async fn not_found(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder().status(404).build())
}
