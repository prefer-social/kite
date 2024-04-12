// when it receives follow activity

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sparrow::activitypub::apo::{Accept, Follow};
use spin_sdk::sqlite::Value as SV;

pub async fn follow_action(activity: serde_json::Value) {
    tracing::debug!("follow_action ##############");

    println!("{activity:?}");

    let follow_object: Follow = serde_json::from_value(activity).unwrap();
    let them = &follow_object.actor;
    let me = &follow_object.object;
    let follow_object_string = serde_json::to_string(&follow_object).unwrap();

    tracing::debug!("follow_action ##############");

    /*

    Send following to inbox

    {
        "@context":"https://www.w3.org/ns/activitystreams",
        "id":"https://ap.dev.seungjin.net/users/seungjin#accepts/follows/",
        "type":"Accept",
        "actor":"https://ap.dev.seungjin.net/users/seungjin",
        "object":
            { "id":"https://mstd.seungjin.net/7e303bf3-79ef-4e91-b240-a9e167f7877e",
              "type":"Follow",
              "actor":"https://mstd.seungjin.net/users/seungjin",
              "object":"https://ap.dev.seungjin.net/users/seungjin"
            }
    }
    */

    // Add this to
    tracing::debug!("---------------------- INSERTING into follower");

    let _ = sparrow::db::Connection::builder().await.execute(
        "INSERT INTO follower(userId, federationId, object) VALUES((SELECT id FROM user WHERE federationId = ?), ?, ?)",
        &[SV::Text(me.to_owned()), SV::Text(them.to_owned()), SV::Text(follow_object_string.clone()) ],
    ).await;

    let body = Accept {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{me}#accepts/follows/"),
        kind: "Accept".to_string(),
        actor: me.clone(),
        object: follow_object.clone(),
    };

    let body_string = serde_json::to_string(&body).unwrap();

    tracing::debug!("---<>>>>>");
    tracing::debug!(them);
    tracing::debug!(body_string);

    let a = sparrow::send::foo(them.to_string(), body_string).await;

    if a.unwrap() == 201 {}
}
