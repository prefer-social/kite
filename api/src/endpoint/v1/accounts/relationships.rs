//! Find out whether a given account is followed, blocked, muted, etc.  
//! GET /api/v1/accounts/relationships HTTP/1.1
//!
//! Returns: Array of Relationship
//! OAuth: User token + read:follows
//! Mastodon doc: <https://docs.joinmastodon.org/methods/accounts/#relationships>
//! Returns: Array of Relationship
//!

use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};
use url::Url;

use crate::auth::Authentication;
use crate::http_response::HttpResponse;
use sparrow::mastodon::account::uid::Uid as AccountUid;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;
use sparrow::mastodon::relationship::Relationship;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "Requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let a_account = match Authentication::verify(&req).await {
        Some(a) => a,
        None => return HttpResponse::unauthorized(),
    };

    let url = req.uri();
    let parsed_url = Url::parse(url).unwrap();
    let query: Vec<(String, String)> =
        parsed_url.query_pairs().into_owned().collect();

    let mut id_array: Vec<String> = Vec::new();
    for (k, v) in query {
        if k == "id[]" {
            id_array.push(v);
        }
    }

    if id_array.len() == 0 {
        // Return empty array
        return Ok(Response::builder()
            .status(200)
            .header("Context-Type", "application/activity+json")
            .body("[]")
            .build());
    }

    let mut relationships: Vec<Relationship> = Vec::new();
    for id in id_array {
        let b_account_uid = AccountUid(id);
        let b_account = MAccount::get(b_account_uid).await?;
        let r = Relationship::new(&a_account, &b_account).await?;
        relationships.push(r);
    }

    let json_string = serde_json::to_string(&relationships).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_string)
        .build())
}
