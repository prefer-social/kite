//pub mod statuses;
//pub mod unfollow;
pub mod follow;
pub mod lookup;
pub mod relationships;
pub mod verify_credentials;
// https://docs.joinmastodon.org/methods/accounts/
// https://docs.joinmastodon.org/methods/accounts/#get

use anyhow::Result;
use sparrow::http_response::HttpResponse;
use spin_sdk::http::{Method, Params, Request, Response};

pub async fn request(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("????????????????");
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

pub async fn get(_req: Request, params: Params) -> Result<Response> {
    tracing::debug!("requested -> GET /api/v1/accounts");
    // let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap()
    // {
    //     sparrow::auth::TokenAuth::InValid => {
    //         return sparrow::http_response::HttpResponse::unauthorized().await;
    //     }
    //     sparrow::auth::TokenAuth::TokenNotProvided => {
    //         return sparrow::http_response::HttpResponse::unauthorized().await;
    //     }
    //     sparrow::auth::TokenAuth::Valid(userid) => {
    //         Some(userid).unwrap() as i64
    //     }
    // };

    // let aaa = params.get("id").unwrap();
    // tracing::debug!(aaa);
    //
    // let query_id = match params.get("id") {
    //     Some(x) => x,
    //     None => return HttpResponse::not_found().await,
    // };
    //
    // let account =
    //     sparrow::mastodon::account::Account::get(query_id.to_string()).await;
    //
    // let b = serde_json::to_string(&account).unwrap();
    // tracing::debug!(b);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("arsars".to_owned())
        .build())
}
