use anyhow::Result;
use spin_sdk::http::{Params, Request, Response};

pub async fn request(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text.html")
        .body("html presentation layer is not yet implemented")
        .build())
}
