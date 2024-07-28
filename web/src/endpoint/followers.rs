use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;
use sparrow::activitypub::follow::follower::Follower;
use sparrow::mastodon::account::Account as MAccount;

pub async fn req(req: Request, params: Params) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    // Todo: Process default account only for now.
    let (default_account, _) = MAccount::default().await?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "applicagtion/activity+json")
        .body(
            Follower::build(default_account.uid)
                .await
                .to_json_string()
                .await
                .unwrap(),
        )
        .build())
}
