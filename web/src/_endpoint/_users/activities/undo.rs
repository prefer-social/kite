use serde::de::IntoDeserializer;
use serde_json::{Map, Value};
use spin_sdk::sqlite::Value as SV;

pub async fn undo_action(activity: serde_json::Value) {
    tracing::debug!("====undo====");
    tracing::debug!("{activity:?}");

    let whats_in_activity = serde_json::to_string(&activity).unwrap();

    tracing::debug!(whats_in_activity);
}

/*


Object {"@context": String("https://www.w3.org/ns/activitystreams"),
"id": String("https://mas.to/users/seungjin#follows/6790942/undo"),
"type": String("Undo"),
"actor": String("https://mas.to/users/seungjin"),

"object": Object {"id": String("https://mas.to/92ffd47c-60c6-4787-a1ff-7f925650a12b"),
"type": String("Follow"), "actor": String("https://mas.to/users/seungjin"),
"object": String("https://ap.dev.seungjin.net/users/seungjin")}}


2024-03-15T07:36:18.192957Z DEBUG web::users::activities::undo: whats_in_activity="{\"@context\":\"https://www.w3.org/ns/activitystreams\",\"id\":\"https://mas.to/users/seungjin#follows/6790942/undo\",\"type\":\"Undo\",\"actor\":\"https://mas.to/users/seungjin\",\"object\":{\"id\":\"https://mas.to/92ffd47c-60c6-4787-a1ff-7f925650a12b\",\"type\":\"Follow\",\"actor\":\"https://mas.to/users/seungjin\",\"object\":\"https://ap.dev.seungjin.net/users/seungjin\"}

*/
