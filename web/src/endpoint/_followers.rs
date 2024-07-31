use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;
use sparrow::activitystream::ordered_collection::OrderedCollection;
use sparrow::mastodon::account::Account as MAccount;

pub async fn req(req: Request, params: Params) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::method_not_allowed(),
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    // Todo: Process default account only for now.
    // Should get from token in request.
    let (default_account, _) = MAccount::default().await?;

    let follower = OrderedCollection::new(
        default_account.followers_url.unwrap(),
        default_account.followers_count as i64,
    );

    let json_string = serde_json::to_string(&follower).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "applicagtion/activity+json")
        .body(json_string)
        .build())
}

/*
{
    "@context": "https://www.w3.org/ns/activitystreams",
    "id": "https://mas.to/users/seungjin/followers",
    "type": "OrderedCollection",
    "totalItems": 1,
    "first": "https://mas.to/users/seungjin/followers?page=1"
}
*/
