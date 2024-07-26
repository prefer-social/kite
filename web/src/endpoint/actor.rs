use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::util;
use sparrow::activitypub::person_actor::PersonActor;

pub async fn req(req: Request, params: Params) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let (account, _user) =
        sparrow::mastodon::account::Account::default().await?;
    let actor = PersonActor::build(account).await.unwrap();
    let s = serde_json::to_string(&actor).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(s)
        .build())
}
