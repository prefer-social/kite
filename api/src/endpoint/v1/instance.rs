use anyhow::Result;
use sparrow::http_response::HttpResponse;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => return HttpResponse::not_found().await,
    }
}

// TODO: GET /api/v1/instance
// https://docs.joinmastodon.org/methods/instance/#v1
pub async fn get(_req: Request, _params: Params) -> Result<Response> {

    let a = sparrow::mastodon::instance::Instance::build().await;
    //.await.to_json_string().await.unwrap();
    let b = a.to_json_string().await.unwrap();




    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(b)
        .build())
}
