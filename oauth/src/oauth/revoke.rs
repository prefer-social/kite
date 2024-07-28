use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder().status(404).build())
}
