// https://docs.joinmastodon.org/methods/accounts/#relationships
// Returns: Array of Relationship

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => crate::http_responses::notfound().await,
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    let url = req.uri();
    let u = Url::parse(url).unwrap();
    let query: HashMap<_, _> = u.query_pairs().into_owned().collect();

    debug!("{query:?}");

    let id = query.get("id[]");

    match id {
        Some(id) => {
            let a = req.body();
            let b = serde_json::to_string(a).unwrap();
            debug!(b);

            let foo = format!(
                r#"[
          {{
            "id": "{id}",
            "following": false,
            "showing_reblogs": true,
            "notifying": false,
            "followed_by": false,
            "blocking": false,
            "blocked_by": false,
            "muting": false,
            "muting_notifications": false,
            "requested": false,
            "domain_blocking": false,
            "endorsed": false
          }}]"#
            );

            let json_val: serde_json::Value = serde_json::from_str(foo.as_str()).unwrap();
            Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .body(json_val.to_string())
                .build())
        }
        None => {
            let foo = r#"[]"#;
            let json_val: serde_json::Value = serde_json::from_str(foo).unwrap();
            Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .body(json_val.to_string())
                .build())
        }
    }
}
