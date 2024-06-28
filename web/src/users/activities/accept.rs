// when it receives accept activity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
//use sparrow::activitypub::apo::{Accept, Follow};
use spin_sdk::sqlite::Value as SV;

/*

{
    "@context":"https://www.w3.org/ns/activitystreams",
"id":"https://mas.to/users/seungjin#accepts/follows/",
"type":"Accept",
"actor":"https://mas.to/users/seungjin",
"object":{
    "id":"https://ap.dev.seungjin.net/6a565478-afc6-46b9-a1bf-2ac6b8c998e7",
    "type":"Follow",
    "actor":"https://ap.dev.seungjin.net/users/seungjin",
    "object":"https://mas.to/users/seungjin"
}
}


*/

pub async fn accept_action(activity: serde_json::Value) {
    tracing::debug!("accept_action ##############");

    tracing::debug!("{activity:?}");

    let object: Accept = serde_json::from_value(activity).unwrap();
    let id = object.id;
    let actor = object.actor;
    let follow_object = object.object;
    let object_id = &follow_object.id;

    tracing::debug!("accept_action ##############");

    let foo = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT object FROM following WHERE json_extract(object, '$.id') = ?",
            &[SV::Text(object_id.clone())],
        )
        .await;

    if foo.rows().count() == 0 {
        tracing::debug!("Accept requested but not Followed");
        return;
    }

    let foo2 = foo.rows().next().unwrap().get::<&str>("object").unwrap();
    let foo3: Follow = serde_json::from_str(foo2).unwrap();

    //debug!(follow_object);
    //debug!(foo3);

    if follow_object == foo3 {
        let now = Utc::now();
        let formated_now = now.format("%F %T").to_string();

        // sparrow::db::Connection::builder()
        //     .await
        //     .execute(
        //         "UPDATE following SET accepted_at = ? WHERE json_extract(object, '$.id') = ?",
        //         &[SV::Text(formated_now), SV::Text(object_id.clone())],
        //     )
        //     .await;
    }

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

    /*
    let d1 = format!("INSERT INTO follower(userId, federationId) VALUES((SELECT id FROM user WHERE federationId = ?), ?)");
    let d2 = object.to_string();
    let d3 = actor.to_string();
    debug!(d1);
    debug!(d2);
    debug!(d3);

    let _ = sparrow::db::Connection::builder().await.execute(
        "INSERT INTO follower(userId, federationId) VALUES((SELECT id FROM user WHERE federationId = ?), ?)",
        &[SV::Text(object.to_string()), SV::Text(actor.to_string())],
    ).await;

    let body = format!(
        r##"{{
            "@context": "https://www.w3.org/ns/activitystreams",
            "id": "{object}#accepts/follows/",
            "type": "Accept",
            "actor": "{object}"
        }}"##
    );
    */
}
