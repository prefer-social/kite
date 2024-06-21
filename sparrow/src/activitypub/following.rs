use anyhow::Result;
use chrono::format::strftime::StrftimeItems;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::table::account::Account;
use crate::table::user::User;

/*

     let a = r#"{
    "@context": "https://www.w3.org/ns/activitystreams",
    "id": "https://dev.prefer.social/followers",
    "type": "OrderedCollection",
    "totalItems": 110,
    "first": "https://dev.prefer.social/followers?page=1"
     }"#;
*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Following {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub total_items: i64,
    pub first: String,
}

impl Following {
    pub async fn build() -> Following {
        Following {
            context: vec!["https://www.w3.org/ns/activitystreams".to_string()],
            id: "https://dev.prefer.social/followers".to_string(),
            kind: "OrderedCollection".to_string(),
            total_items: 112,
            first: "https://dev.prefer.social/followers?page=1".to_string(),
        }
    }

    pub async fn to_json_string(self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }
}
