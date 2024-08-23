//! Unfollow is basically doing undo follow
//! (POST) /api/v1/accounts/seungjin@mas.to/unfollow
//! https://docs.joinmastodon.org/methods/accounts/#unfollow

use anyhow::Result;
use serde_json::to_string;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use std::path::MAIN_SEPARATOR;
use tracing::debug;
use url::Url;

use sparrow::activitystream::activity::follow::{self, Follow};
use sparrow::activitystream::activity::Activity;
use sparrow::activitystream::activity::ActivityType;
use sparrow::activitystream::remove_context;
use sparrow::http_response::HttpResponse;
use sparrow::mastodon::account::uid::Uid;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;
use sparrow::mastodon::follow::Follow as MFollow;
use sparrow::mastodon::relationship::Relationship;
use sparrow::mastodon::ME_ACCOUNT;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => HttpResponse::method_not_allowed(),
    }
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    tracing::trace!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap(),
    );

    let me_account = ME_ACCOUNT.get().unwrap().to_owned();
    let who_to_unfollow = Uid(params.get("id").unwrap().to_string());
    let who_to_unfollow_account = MAccount::get(who_to_unfollow).await?;

    // Find Follow objetc I sent.

    // SELECT FROM FOLLOW WHERE account_id = AND target_account_id =
    let follow_record =
        MFollow::follow_record(&me_account, &who_to_unfollow_account).await?;
    let fw = follow_record.unwrap();

    // Send Undo request.

    let follow_activity = Activity::new(
        false,
        fw.uri.unwrap(),
        ActivityType::Follow,
        me_account.actor_url.to_string(),
        None,
        None,
        None,
        Follow(who_to_unfollow_account.actor_url.to_string()),
    );

    let a = serde_json::to_value(&follow_activity).unwrap();
    tracing::trace!("{:?}", a);

    // Database update to unfollow

    let relationship =
        Relationship::new(&me_account, &who_to_unfollow_account)
            .await
            .unwrap(); //Todo: Error process
    let json_str = serde_json::to_string(&relationship).unwrap(); // Tood: Error process

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_str)
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
