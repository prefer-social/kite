// https://docs.joinmastodon.org/methods/accounts/#relationships
// Returns: Array of Relationship
// https://docs.joinmastodon.org/entities/Relationship/

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("Requested -> {} {}", req.method().to_string(), req.path_and_query().unwrap());

    let url = req.uri();
    let parsed_url = Url::parse(url).unwrap();
    let query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();

    debug!("{query:?}");

    let id = query.get("id[]");

    match id {
        Some(id) => {
            let a = req.body();
            let b = serde_json::to_string(a).unwrap();
            debug!(b);

            let foo = sparrow::mastodon::relationship::Relationship {
                uid: id.into(),
                following: Some(false),
                showing_reblogs: Some(true),
                notifying: Some(false),
                followed_by: Some(false),
                blocking: Some(false),
                blocked_by: Some(false),
                muting: Some(false),
                muting_notifications: Some(false),
                requested: Some(false),
                domain_blocking: Some(false),
                endorsed: Some(false),
                ..Default::default()
            };

            let json_val = serde_json::to_string(&foo).unwrap();

            tracing::debug!(json_val);

            Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .body(json_val)
                .build())
        }
        None => {
            Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .body("[]")
                .build())
        }
    }
}
