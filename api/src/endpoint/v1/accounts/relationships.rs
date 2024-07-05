// https://docs.joinmastodon.org/methods/accounts/#relationships
// Returns: Array of Relationship
//

use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};
use std::collections::HashMap;
use url::Url;
use sparrow::mastodon::relationship::Relationship;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("Requested -> {} {}", req.method().to_string(), req.path_and_query().unwrap());

    let url = req.uri();
    let parsed_url = Url::parse(url).unwrap();
    let query: Vec<(String, String)> = parsed_url.query_pairs().into_owned().collect();

    let mut id_array: Vec<String> = Vec::new();
    for (k,v) in query {
        if k == "id[]" {
            id_array.push(v);
        }
    }

    if id_array.len() == 0 { // Return empty array
        return Ok(Response::builder()
            .status(200)
            .header("Context-Type", "application/activity+json")
            .body("[]")
            .build());;
    }

    let mut relationships: Vec<Relationship> = Vec::new();
    for id in id_array {
        let foo = sparrow::mastodon::relationship::Relationship {
            uid: id,
            following: false,
            showing_reblogs: true,
            notifying: false,
            followed_by: true,
            blocking: false,
            blocked_by: false,
            muting: false,
            muting_notifications: false,
            requested: false,
            domain_blocking: false,
            endorsed: false,
            ..Default::default()
        };
        relationships.push(foo);
    }

    let json_val = serde_json::to_string(&relationships).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val)
        .build())

}
