use anyhow::Result;
use spin_sdk::{
    http::{HeaderValue, IntoResponse, Method, Request, Response},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

use crate::http_response::HttpResponse;
use sparrow::activitystream::ordered_collection::OrderedCollection;
use sparrow::mastodon::account::Account as MAccount;

pub mod http_response;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_outbox(req: Request) -> anyhow::Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!(
        "<---------- ({}) {} ({}) {}--------->",
        req.method().to_string(),
        req.path_and_query().unwrap_or_default(),
        req.header("x-forwarded-ip")
            .unwrap_or(&HeaderValue::string("EMPTY".to_string()))
            .as_str()
            .unwrap(),
        req.header("Accept")
            .unwrap_or(&HeaderValue::string("EMPTY Accept header".to_string()))
            .as_str()
            .unwrap(),
    );

    match req.method() {
        Method::Get => get(req).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(_req: Request) -> Result<Response> {
    let body = r#"
    {
      "id": "https://mastodon.jgarr.net/featured",
      "type": "OrderedCollection",
      "totalItems": 1,
      "orderedItems": [
        {
          "id": "https://mastodon.jgarr.net/1",
          "type": "Note",
          "summary": null,
          "inReplyTo": null,
          "published": "2022-04-26T00:11:17Z",
          "url": "https://mastodon.jgarr.net/1",
          "attributedTo": "https://mastodon.jgarr.net/justin",
          "to": [
            "https://www.w3.org/ns/activitystreams#Public"
          ],
          "cc": [
            "https://mastodon.jgarr.net/followers"
          ],
          "sensitive": false,
          "atomUri": "https://mastodon.jgarr.net/1",
          "inReplyToAtomUri": null,
          "conversation.rs": "tag:mastodon.jgarr.net,2022-04-26:objectId=288755344:objectType=Conversation",
          "content": "<p>THIS IS PINNED</p>",
          "contentMap": {
            "en": "<p>THIS IS PINNED</p>"
          },
          "attachment": [],
          "tag": [],
          "replies": {
            "id": "https://mastodon.jgarr.net/1_replies",
            "type": "Collection",
            "first": {
              "type": "CollectionPage",
              "next": "https://mastodon.jgarr.net/1_replies_more",
              "partOf": "https://mastodon.jgarr.net/1_replies",
              "items": []
            }
          }
        }
      ]
    }"#;

    //let b: Value = serde_json::from_str(body).unwrap();
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(body.to_string())
        .build())
}

/*

{
  "@context": "https://www.w3.org/ns/activitystreams",
  "id": "https://mstd.seungjin.net/users/seungjin/outbox",
  "type": "OrderedCollection",
  "totalItems": 2072,
  "first": "https://mstd.seungjin.net/users/seungjin/outbox?page=true",
  "last": "https://mstd.seungjin.net/users/seungjin/outbox?min_id=0&page=true"
}

  OR




*/
