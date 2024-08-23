// https://docs.joinmastodon.org/methods/favourites/#get
use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

// Returns: Array of Status (https://docs.joinmastodon.org/entities/Status/)
//
pub async fn get(req: Request, params: Params) -> Result<Response> {
    debug!("{params:?}");

    let foo = r#"[]"#;

    let json_val: serde_json::Value = serde_json::from_str(foo).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())
}
