use serde::{Serialize, Deserialize};
/// ActivityPub Object struct
/// Resource: <https://www.w3.org/TR/activitypub/#obj>

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object<T> {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    //#[serde(skip_serializing_if = "Option::is_none")]
    pub object: T,
}
