use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value},
};
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub mod health;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

// `501 Not Implemented`` for now
pub async fn get(req: Request, params: Params) -> Result<Response> {
    Ok(Response::builder()
        .status(501)
        .body("Not Implemented (yet)".to_string())
        .build())
}
