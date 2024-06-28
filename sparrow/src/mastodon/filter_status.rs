// https://docs.joinmastodon.org/entities/FilterStatus/
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;


#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FilterStatus {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub status_id: String,
}
