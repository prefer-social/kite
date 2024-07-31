use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::http_response::HttpResponse;
use sparrow::activitystream::ordered_collection::OrderedCollection;
use sparrow::mastodon::account::Account as MAccount;

pub async fn req(
    req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    let (default_account, _) = MAccount::default().await?;

    let following = OrderedCollection::new(
        default_account.following_url.unwrap(),
        default_account.following_count as i64,
    );
    tracing::debug!("{:?}", following);
    let json_string = serde_json::to_string(&following).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(json_string)
        .build())
}
