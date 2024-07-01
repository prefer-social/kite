// https://docs.joinmastodon.org/methods/accounts/#lookup
// Quickly lookup a username to see if it is available, skipping WebFinger resolution.
// Returns: Account

use anyhow::Result;
use sparrow::http_response::HttpResponse;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use url::Url;
use sparrow::mastodon::account::Get;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        //Method::Get => get(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}
