// GET /api/v1/timelines/home HTTP/1.1
// https://docs.joinmastodon.org/methods/timelines/#home
// Returns: Array of Status

use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value},
};
use tracing::debug;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

// GET /api/v1/timelines/home HTTP/1.1
// TODO: (GET) /api/v1/timelines/home?min_id=111655997000785377 use min_id
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    debug!("timelines/home was called");

    //let a = req.header("min"

    let foo = r#"[]"#;

    let json_val: serde_json::Value = serde_json::from_str(foo).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())
}
