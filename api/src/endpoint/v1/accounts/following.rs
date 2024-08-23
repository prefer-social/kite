// Return Array of Account
// (GET) /api/v1/accounts/018f56d7-bac8-7f76-a2eb-b089696db637/following
// https://docs.joinmastodon.org/methods/accounts/#following
use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use crate::auth::Authentication;
use crate::http_response::HttpResponse;
use sparrow::activitystream::activity::follow::Follow;
use sparrow::activitystream::activity::Activity;
use sparrow::activitystream::activity::ActivityType;
use sparrow::activitystream::actor::person::Person;
use sparrow::mastodon::account::uid::Uid;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;
use sparrow::mastodon::follow::Follow as MFollow;
use sparrow::mastodon::relationship::Relationship;
use sparrow::mastodon::token::Token;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

/// Accounts which the given account is following, if network is not hidden by the account owner.  
///
/// <https://docs.joinmastodon.org/methods/accounts/#following>
/// Returns: Array of Account
pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    // let account = match Authentication::verify(&req).await {
    //     Some(account) => account,
    //     None => {
    //         tracing::debug!("Authentication failed");
    //     }
    // };

    let id = params.get("id").unwrap();
    tracing::debug!("{:?}", id);

    let a = MFollow::get_following(id.to_string()).await?;

    let b = serde_json::to_string(&a)?;

    return Ok(Response::builder()
        .status(200)
        .header("Content-Type", "Application/json")
        .body(b)
        .build());

    // // Get MAccount info with who_to_follow
    // let to_account = MAccount::get(who_to_follow).await?;
    // //let recipient = to_account.account_uri;
    // let recipient_actor = Person::new(to_account.to_owned()).await.unwrap();

    // // Get MAccount info about me
    // let from_account = Token::owner("Bearer".to_string(), token.to_string()).await?;
    // let from_actor = Person::new(from_account.to_owned()).await.unwrap();

    // // ActivityPub request to follow

    // let follow_object = Follow::new::<Follow>(from_actor.id, recipient_actor.id).await;
    // let send_result = sparrow::mastodon::post_activity(follow_object).await?;

    // if send_result == 202u16 {
    //     // Let's return relationship.
    //     //let me
    //     //let follower =

    //     let mut r = Relationship::new(&from_account, &to_account).await?;
    //     r.requested = true;
    //     let a = serde_json::to_string(&r).unwrap();

    //     tracing::debug!(a);

    //     return Ok(Response::builder()
    //         .status(202)
    //         .header("Content-Type", "Application/json")
    //         .body(a)
    //         .build());
    // }
}
