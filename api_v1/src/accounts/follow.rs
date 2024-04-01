// (POST) /api/v1/accounts/seungjin@mas.to/follow?notify=false&reblogs=true
// https://docs.joinmastodon.org/methods/accounts/#follow

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::redis;
use spin_sdk::sqlite::Value as SV;
use spin_sdk::variables;
use tracing::debug;
use url::Url;
use uuid::Uuid;

use sparrow::apo::Follow;
use sparrow::postbox::Envelop;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => crate::http_responses::notfound().await,
    }
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap() {
        sparrow::auth::TokenAuth::InValid => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => Some(userid).unwrap() as i64,
    };

    let user_db_id = params.get("id").unwrap().to_string();
    let recipient = sparrow::utils::get_actor_url_from_id(user_db_id)
        .await
        .unwrap();
    let recipient_actor = Url::parse(&recipient.as_str()).unwrap();
    let (federation_id, private_key) = sparrow::utils::get_local_user(userid).await.unwrap();
    let my_actor = Url::parse(&federation_id).unwrap();
    let private_key_pem = sparrow::utils::get_privatekey_with_user_name("seungjin")
        .await
        .unwrap();

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
        actor: my_actor.to_string(),
        object: recipient_actor.to_string(),
    };

    let envelop = Envelop {
        address: recipient.clone(),
        letter: &follow_object,
    };

    let paylod = serde_json::to_vec(&envelop).unwrap();

    let rc = variables::get("redis_credential").unwrap();
    tracing::debug!("rc --> {}", rc);

    let address = format!(
        "redis://{}@{}:{}",
        variables::get("redis_credential").unwrap(),
        variables::get("redis_host").unwrap(),
        variables::get("redis_port").unwrap(),
    );
    let conn = redis::Connection::open(&address)?;

    let put_into_postbox = conn.publish("postbox", &paylod);

    debug!("{put_into_postbox:?}");

    let _rq = sparrow::db::Connection::builder()
        .await
        .execute(
            "INSERT INTO following(userId, federationId, object) VALUES(?,?,?)",
            &[
                SV::Text(userid.to_string()),
                SV::Text(recipient_actor.to_string()),
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

        let json_val: serde_json::Value = serde_json::from_str(foo.as_str()).unwrap();
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
