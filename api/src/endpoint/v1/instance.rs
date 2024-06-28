use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

pub mod peer;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return sparrow::http_response::HttpResponse::not_found().await,
    }
}

// TODO: GET /api/v1/instance
// https://docs.joinmastodon.org/methods/instance/#v1
pub async fn get(req: Request, _params: Params) -> Result<Response> {

    tracing::debug!("<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    let a = sparrow::mastodon::instance::Instance::build().await;
    let b: String = a.into();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(b)
        .build())
}
