//! Accounts API  
//! Mastodon doc: <https://docs.joinmastodon.org/methods/accounts/>

//pub mod statuses;
//pub mod unfollow;
pub mod follow;
pub mod lookup;
pub mod relationships;
pub mod unfollow;
pub mod verify_credentials;
// https://docs.joinmastodon.org/methods/accounts/
// https://docs.joinmastodon.org/methods/accounts/#get

use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use crate::auth::Authentication;

use crate::http_response::HttpResponse;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

/// Get multiple accounts  
///
/// `GET /api/v1/accounts HTTP/1.1`
/// Returns: Account
/// Mastodon doc: <https://docs.joinmastodon.org/methods/accounts/#index>
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let account = match Authentication::verify(&req).await {
        None => return HttpResponse::unauthorized(),
        Some(x) => x,
    };

    let r = serde_json::to_string(&account).unwrap();
    tracing::debug!(r);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "Application/json")
        .body(r)
        .build())
}
