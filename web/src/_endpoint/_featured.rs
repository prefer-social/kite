// https://kite.seungjin.net/users/seungjin/collections/features

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::utils::not_found;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
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
