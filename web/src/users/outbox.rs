// Outbox

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => {
            todo!();
            //not_found(req, params).await,
        }
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    let json_val: serde_json::Value = serde_json::from_str("{}").unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())
}
