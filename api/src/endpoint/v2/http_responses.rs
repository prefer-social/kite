use anyhow::Result;
use spin_sdk::http::Response;

pub async fn unauthorized() -> Result<Response> {
    let a = r#"{"error": "The access token is invalid"}"#;
    Ok(Response::builder()
        .status(401)
        .header("Content-Type", "Application/json")
        .body(a)
        .build())
}

pub async fn notfound() -> Result<Response> {
    Ok(Response::builder()
        .status(404)
        //.header("Content-Type", "Application/json")
        //.body(a)
        .build())
}
