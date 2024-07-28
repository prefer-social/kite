//! Accept
//!
//!

use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::Response;

use crate::http_response::HttpResponse;
use sparrow::activitypub::object::Object as APObject;

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

pub(crate) async fn accept(object: APObject<Value>) -> Result<Response> {
    tracing::debug!("{:?}", object);

    // let follow_object = Follow::new(from_actor.id, recipient_actor.id).await;

    HttpResponse::teapot()
}
