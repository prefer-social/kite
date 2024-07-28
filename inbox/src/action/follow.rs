//! Follow
//!
//! Return https://docs.joinmastodon.org/entities/Relationship/

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::Response;

use crate::http_response::HttpResponse;
use sparrow::activitypub::object::Object as APObject;

pub(crate) async fn follow(object: APObject<Value>) -> Result<Response> {
    let a = sparrow::activitypub::follow::Follow::receive(object).await;
    match a {
        Ok(_) => {
            tracing::debug!("Let's send their inbox ... folloing is ok.");
            // Should return http status 202, Accepted
            return HttpResponse::accepted();
        }
        _ => {
            tracing::debug!("follow table insert error");
        }
    }
    // Once it follows well,

    return HttpResponse::not_implemented();
}
