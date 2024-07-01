// (POST) /api/v1/accounts/seungjin@mas.to/follow?notify=false&reblogs=true
// https://docs.joinmastodon.org/methods/accounts/#follow

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::redis;
use spin_sdk::sqlite::Value as SV;
use spin_sdk::variables;
use url::Url;
use uuid::Uuid;
use Option;

use sparrow::activitypub::apo::Follow;
use sparrow::http_response::HttpResponse;
use sparrow::postbox::Envelop;
use sparrow::utils::{get_current_time_in_iso_8601, get_inbox_from_actor};

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("requested -> {} {}", req.method().to_string(), req.path_and_query().unwrap());

    let follow_user = params.get("id").unwrap().to_string();
    let recipient = sparrow::utils::get_actor_url_from_id(follow_user)
        .await
        .unwrap();

    let (federation_id, _private_key) =
        sparrow::utils::get_local_user(userid).await.unwrap();

    let my_actor = Url::parse(&federation_id).unwrap();

    let uuid = Uuid::now_v7().to_string();
    let id = format!(
        "{}://{}/{}",
        String::from(my_actor.scheme()),
        String::from(my_actor.host_str().unwrap()),
        uuid
    );

    let follow_object = Follow {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: id,
        kind: "Follow".to_string(),
        actor: federation_id,
        object: recipient.clone(),
    };

    let envelop = Envelop {
        address: recipient.clone(),
        letter: &follow_object,
    };

    let paylod = serde_json::to_vec(&envelop).unwrap();

    let rc = variables::get("redis_credential").unwrap();

    let address = format!(
        "redis://{}@{}:{}",
        variables::get("redis_credential").unwrap(),
        variables::get("redis_host").unwrap(),
        variables::get("redis_port").unwrap(),
    );
    let conn = redis::Connection::open(&address)?;

    let redis_channel = variables::get("redis_channel").unwrap();
    tracing::debug!("-------------?------------");
    tracing::debug!(redis_channel);
    let put_into_postbox = conn.publish(redis_channel.as_str(), &paylod);

    tracing::debug!("---> {put_into_postbox:?}");

    let _rq = sparrow::db::Connection::builder()
        .await
        .execute(
            "INSERT OR IGNORE INTO following(userId, federationId, object) VALUES(?,?,?)",
            &[
                SV::Text(userid.to_string()),
                SV::Text(recipient.clone()),
                SV::Text(serde_json::to_string(&follow_object).unwrap()),
            ],
        )
        .await;

    if put_into_postbox.is_ok() {
        let foo = format!(
            r#"{{
            "id": "{recipient}",
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
        }}"#
        );

        let json_val: serde_json::Value =
            serde_json::from_str(foo.as_str()).unwrap();
        Ok(Response::builder()
            .status(200)
            .header("Context-Type", "application/activity+json")
            .body(json_val.to_string())
            .build())
    } else {
        Ok(Response::builder()
            .status(500)
            .header("Context-Type", "application/activity+json")
            .build())
    }
}
