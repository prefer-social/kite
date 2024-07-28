// (POST) /api/v1/accounts/seungjin@mas.to/follow?notify=false&reblogs=true
// https://docs.joinmastodon.org/methods/accounts/#follow

use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use crate::http_response::HttpResponse;
use sparrow::activitypub::actor::Actor;
use sparrow::activitypub::follow::Follow;
use sparrow::mastodon::account::uid::Uid;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;
use sparrow::mastodon::token::Token;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

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
    let recipient_actor = Actor::build(to_account.to_owned()).await.unwrap();

    // Get MAccount info about me
    let from_account =
        Token::owner("Bearer".to_string(), token.to_string()).await?;
    let from_actor = Actor::build(from_account.to_owned()).await.unwrap();

    // ActivityPub request to follow

    let follow_object = Follow::new(from_actor.id, recipient_actor.id).await;
    let send_result = sparrow::mastodon::send(follow_object).await?;

    if send_result == 202u16 {
        // Need to update table.
        // Follow requested successfully.
    }

    // let envelop = Envelop {
    //     address: recipient.clone(),
    //     letter: &follow_object,
    // };

    // let paylod = serde_json::to_vec(&envelop).unwrap();

    // let rc = variables::get("redis_credential").unwrap();

    // let address = format!(
    //     "redis://{}@{}:{}",
    //     variables::get("redis_credential").unwrap(),
    //     variables::get("redis_host").unwrap(),
    //     variables::get("redis_port").unwrap(),
    // );
    // let conn = redis::Connection::open(&address)?;

    // let redis_channel = variables::get("redis_channel").unwrap();
    // tracing::debug!("-------------?------------");
    // tracing::debug!(redis_channel);
    // let put_into_postbox = conn.publish(redis_channel.as_str(), &paylod);

    // tracing::debug!("---> {put_into_postbox:?}");

    // let _rq = sparrow::db::Connection::builder()
    //     .await
    //     .execute(
    //         "INSERT OR IGNORE INTO following(userId, federationId, object) VALUES(?,?,?)",
    //         &[
    //             SV::Text(userid.to_string()),
    //             SV::Text(recipient.clone()),
    //             SV::Text(serde_json::to_string(&follow_object).unwrap()),
    //         ],
    //     )
    //     .await;

    // if put_into_postbox.is_ok() {
    //     let foo = format!(
    //         r#"{{
    //         "id": "{recipient}",
    //         "following": true,
    //         "showing_reblogs": false,
    //         "notifying": false,
    //         "followed_by": false,
    //         "blocking": false,
    //         "blocked_by": false,
    //         "muting": false,
    //         "muting_notifications": false,
    //         "requested": false,
    //         "domain_blocking": false,
    //         "endorsed": false
    //     }}"#
    //     );

    //     let json_val: serde_json::Value =
    //         serde_json::from_str(foo.as_str()).unwrap();
    //     Ok(Response::builder()
    //         .status(200)
    //         .header("Context-Type", "application/activity+json")
    //         .body(json_val.to_string())
    //         .build())
    // } else {
    //     Ok(Response::builder()
    //         .status(500)
    //         .header("Context-Type", "application/activity+json")
    //         .build())
    // }

    HttpResponse::accepted()
}
