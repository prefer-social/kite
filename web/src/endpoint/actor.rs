use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;
use sparrow::activitystream::actor::person::Person;

pub async fn req(req: Request, params: Params) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
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
    let actor = Person::new(account).await.unwrap();
    let s = serde_json::to_string(&actor).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(s)
        .build())
}
