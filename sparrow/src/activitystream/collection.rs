//! A Collection is a subtype of Object that represents ordered or unordered sets of Object or Link instances.
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection>

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ActivityPub Collection.  
/// <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection>
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub collection_type: String,
    pub total_items: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
}

impl Collection {
    pub fn new() -> Self {
        todo!()
    }
}
