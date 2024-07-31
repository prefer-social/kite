//! Accept
//!
//!

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::Response;

use crate::http_response::HttpResponse;
use sparrow::activitystream::activity::follow::Follow;
use sparrow::activitystream::activity::Activity;
use sparrow::mastodon::account::actor_url::ActorUrl;
use sparrow::mastodon::account::uid::Uid as AccountUid;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;
use sparrow::mastodon::activity_log::ActivityLog;
use sparrow::mastodon::follow::Follow as MFollow;

/*
Object {
  context: "https://www.w3.org/ns/activitystreams",
  id: "https://mas.to/users/seungjin#accepts/follows/",
  object_type: Accept,
  actor: "https://mas.to/users/seungjin",
  published: None,
  to: None,
  cc: None,
  object: Object {
    "id": String("https://dev.prefer.social/0190f9e0-6ed8-78e0-bf59-7f74bfc0feb0"),
    "type": String("Follow"),
    "actor": String("https://dev.prefer.social/self"),
    "object": String("https://mas.to/users/seungjin")
  }
}
*/

pub(crate) async fn received(object: Activity<Value>) -> Result<Response> {
    // Check Value type
    let obj_type = object.object.get::<String>("type".to_string()).unwrap();
    let obj_type_str = obj_type.as_str().unwrap();

    let object_clone = object.clone();

    match obj_type_str {
        "Follow" => {
            let activity =
                Follow::new(object.actor, object.object.to_string()).await;

            // Check activiy.object is what I really sent.
            // https://dev.prefer.social/0190fcb0-5272-77c3-acb1-3e9be71ff930
            // SELECT * FROM activity_log WHERE JSON_EXTRACT(body, '$.id') = ?
            match ActivityLog::get_with_id(
                object.object.get("id").unwrap().as_str().unwrap(),
            )
            .await
            .unwrap()
            {
                None => {
                    tracing::error!(
                        "Havn't published this acticity {}",
                        object.object.get("id").unwrap().to_string()
                    )
                }
                Some(x) => {
                    let log_obj = sparrow::activitypub::remove_context(x);
                    let given_obj = sparrow::activitypub::remove_context(
                        object_clone.object,
                    );
                    if given_obj != log_obj {
                        tracing::error!(
                        "Integration error! No matching follow was published! {}", object.object.get("id").unwrap().to_string()
                    )
                    }
                }
            };

            let fa = serde_json::from_str::<Value>(
                activity.object.to_string().as_str(),
            )
            .unwrap();

            let uri = fa.get("id").unwrap().to_string();

            let sub_actor_url = ActorUrl::new(
                fa.get("actor").unwrap().as_str().unwrap().to_string(),
            )
            .unwrap();

            let sub = MAccount::get(sub_actor_url).await?.uid;

            let obj_actor_url = ActorUrl::new(
                fa.get("object").unwrap().as_str().unwrap().to_string(),
            )
            .unwrap();

            let obj = MAccount::get(obj_actor_url).await?.uid;

            MFollow::add(uri, sub, obj).await?;
        }
        _ => {}
    }

    HttpResponse::accepted()
}
