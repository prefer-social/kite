/// Return Returns: Relationship
// (POST) /api/v1/accounts/seungjin@mas.to/follow?notify=false&reblogs=true
// https://docs.joinmastodon.org/methods/accounts/#follow
use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;

use sparrow::activitystream::activity::follow::Follow;
use sparrow::activitystream::activity::Activity;
use sparrow::activitystream::activity::ActivityType;

use sparrow::activitystream::actor::person::Person;
use sparrow::mastodon::account::uid::Uid;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;
use sparrow::mastodon::relationship::Relationship;
use sparrow::mastodon::token::Token;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

/// Returns: Relationship
pub async fn post(req: Request, params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let mut token = req.header("Authorization").unwrap().as_str().unwrap();
    //if token.starts_with("Bearer ") {
    //    token = token.replace("Bearer ", "");
    //}

    let mut c = token.chars();
    for _ in "Bearer ".chars().into_iter() {
        c.next();
    }
    token = c.as_str();

    let who_to_follow = Uid(params.get("id").unwrap().to_string());

    // Get MAccount info with who_to_follow
    let to_account = MAccount::get(who_to_follow).await?;
    //let recipient = to_account.account_uri;
    let recipient_actor = Person::new(to_account.to_owned()).await.unwrap();

    // Get MAccount info about me
<<<<<<< HEAD
    let from_account =
        Token::owner("Bearer".to_string(), token.to_string()).await?;
=======
    let from_account = Token::owner("Bearer".to_string(), token.to_string()).await?;
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
    let from_actor = Person::new(from_account.to_owned()).await.unwrap();

    // ActivityPub request to follow

<<<<<<< HEAD
    let follow_object =
        Follow::new::<Follow>(from_actor.id, recipient_actor.id).await;
    let send_result =
        sparrow::mastodon::publish_activity(follow_object).await?;
=======
    let follow_object = Follow::new::<Follow>(from_actor.id, recipient_actor.id).await;
    let send_result = sparrow::mastodon::post_activity(follow_object).await?;
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac

    if send_result == 202u16 {
        // Let's return relationship.
        //let me
        //let follower =

        let mut r = Relationship::new(&from_account, &to_account).await?;
        r.requested = true;
        let a = serde_json::to_string(&r).unwrap();

        tracing::debug!(a);

        return Ok(Response::builder()
            .status(202)
            .header("Content-Type", "Application/json")
            .body(a)
            .build());
    }

    HttpResponse::not_acceptable()
}
