//! Unfollow is basically doing undo follow
//! (POST) /api/v1/accounts/seungjin@mas.to/unfollow
//! https://docs.joinmastodon.org/methods/accounts/#unfollow

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

use sparrow::activitystream::activity::follow::Follow;
use sparrow::http_response::HttpResponse;
use sparrow::mastodon::account::uid::Uid;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => HttpResponse::method_not_allowed(),
    }
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let mut token = req.header("Authorization").unwrap().as_str().unwrap();

    let mut c = token.chars();
    for _ in "Bearer ".chars().into_iter() {
        c.next();
    }
    token = c.as_str();

    let who_to_unfollow = Uid(params.get("id").unwrap().to_string());

    // Unfollow is basically doing undo follow

    /*
    {
      "@context":"https://www.w3.org/ns/activitystreams",
      "id":"https://mas.to/users/seungjin#follows/6620256/undo",
      "type":"Undo",
      "actor":"https://mas.to/users/seungjin",
      "object":{
        "id":"https://mas.to/0614bc2a-9db6-463d-b23b-772fca54b47b",
        "type":"Follow",
        "actor":"https://mas.to/users/seungjin",
        "object":"https://dev.prefer.social/self"
      }
    }
    */

    // Find the original follow request message from follow table (follow.uri)
    // and generate Follow object
    /*
        let follow = Follow();

        let follow_activity = Activity::new(
            id,
            ActivityType::Follow,
            actor,
            None,
            None,
            None,
            follow,
        );

        let undo_activity = Activity



    */

    //let my_actor = Url::parse("https://ap.dev.seungjin.net/users/seungjin").unwrap();
    //let recipient_actor = Url::parse("https://mas.to/users/seungjin").unwrap();

    let id = params.get("id").unwrap().to_string();
    debug!(id);

    let foo = format!(
        r#"{{
      "id": "{id}",
      "following": false,
      "showing_reblogs": false,
      "notifying": false,
      "followed_by": false,
      "blocking": false,
      "blocked_by": false,
      "muting": false,
      "muting_notifications": false,
      "requested": false,
      "domain_blocking": false,
      "endorsed": false
    }}"#
    );

    let json_val: serde_json::Value =
        serde_json::from_str(foo.as_str()).unwrap();
    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())

    /*
        return example:

        {
      "id": "3",
      "following": true,
      "showing_reblogs": false,
      "notifying": false,
      "followed_by": false,
      "blocking": false,
      "blocked_by": false,
      "muting": false,
      "muting_notifications": false,
      "requested": false,
      "domain_blocking": false,
      "endorsed": false
    }

         */
}
