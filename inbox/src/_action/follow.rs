//! Follow
//!
//! Return https://docs.joinmastodon.org/entities/Relationship/

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::Response;

use crate::http_response::HttpResponse;
use sparrow::activitypub::ap_object::ApObject;
use sparrow::mastodon;
use sparrow::mastodon::account::Account as MAccount;

pub(crate) async fn received(object: ApObject<Value>) -> Result<Response> {
    let a =
        sparrow::activitypub::follow::Follow::to_activitypub_obj(object).await;
    match a {
        Ok(b) => {
            // Send accept request
            /*
             {"@context":"https://www.w3.org/ns/activitystreams",
              "id":"https://mas.to/users/seungjin#accepts/follows/",
               "type":"Accept",
                "actor":"https://mas.to/users/seungjin",
                "object":{"id":"https://dev.prefer.social/0190fd6b-8a97-7300-b60a-e9ae4fe6e4c7","type":"Follow","actor":"https://dev.prefer.social/self","object":"https://mas.to/users/seungjin"}}

            */
            let c = serde_json::to_value(b).unwrap();

            let (default_user, _) = MAccount::default().await?;

            let actor = default_user.actor_url.to_string();
            let accept =
                sparrow::activitypub::accept::Accept::new(actor, c).await;

            let a = mastodon::publish_activity(accept).await?;

            // Insert into DB

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
