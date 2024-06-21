// (POST) /api/v1/accounts/seungjin@mas.to/unfollow
// https://docs.joinmastodon.org/methods/accounts/#unfollow

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

use sparrow::http_response::HttpResponse;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap()
    {
        sparrow::auth::TokenAuth::InValid => {
            return HttpResponse::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return HttpResponse::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => {
            Some(userid).unwrap() as i64
        }
    };

    debug!(userid);

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
