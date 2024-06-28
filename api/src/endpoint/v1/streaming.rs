use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use std::collections::HashMap;
use url::Url;
pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

// `501 Not Implemented`` for now
pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    sparrow::http_response::HttpResponse::not_implemented().await
}
