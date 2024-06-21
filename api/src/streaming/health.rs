use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use tracing::debug;

use sparrow::http_response::HttpResponse;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => Ok(Response::builder().status(404).build()),
    }
}

// https://docs.joinmastodon.org/methods/streaming/#health
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap()
    {
        sparrow::auth::TokenAuth::InValid => {
            return HttpResponse::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return HttpResponse::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => {
            Some(userid).unwrap() as i64
        }
    };

    debug!(userid);

    Ok(Response::builder().status(404).build())
}
