use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::{Method, RequestBuilder, Response};

use crate::http_response::HttpResponse;
use sparrow::activitypub::ap_object::ApObject;
use sparrow::mastodon::account::actor_url::ActorUrl;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Remove as _;

pub(crate) async fn received(object: ApObject<Value>) -> Result<Response> {
    let request = RequestBuilder::new(Method::Get, object.actor.to_owned())
        .header("Content-Type", "application/activity+json")
        .build();
    let response: Response = spin_sdk::http::send(request).await.unwrap();
    if *response.status() == 401u16 {
        tracing::debug!("Account '{}' is gone.", object.actor);

        let actor_url = ActorUrl::new(object.actor).unwrap();
        MAccount::remove(actor_url).await?;
        return HttpResponse::accepted();
    }
    HttpResponse::invalid_request()
}
