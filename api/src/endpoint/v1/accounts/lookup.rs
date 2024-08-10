// https://docs.joinmastodon.org/methods/accounts/#lookup
// Quickly lookup a username to see if it is available, skipping WebFinger resolution.
// Returns: Account

use anyhow::Result;
use spin_sdk::http::{Params, Request, Response};

use crate::http_response::HttpResponse;

pub async fn request(req: Request, _params: Params) -> Result<Response> {
    match req.method() {
        //Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}
