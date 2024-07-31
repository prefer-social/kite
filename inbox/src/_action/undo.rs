use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::Response;

use crate::http_response::HttpResponse;
use sparrow::activitypub::ap_object::ApObject;

pub(crate) async fn undo(object: ApObject<Value>) -> Result<Response> {
    tracing::debug!("{:?}", object);

    // let follow_object = Follow::new(from_actor.id, recipient_actor.id).await;

    HttpResponse::teapot()
}
