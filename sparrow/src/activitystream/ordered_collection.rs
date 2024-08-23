//!  A subtype of Collection in which members of the logical collection are assumed to always be strictly ordered.
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollection>

/*
{
  "@context": "https://www.w3.org/ns/activitystreams",
  "summary": "Sally's notes",
  "type": "OrderedCollection",
  "totalItems": 2,
  "orderedItems": [
    {
      "type": "Note",
      "name": "A Simple Note"
    },
    {
      "type": "Note",
      "name": "Another Simple Note"
    }
  ]
}
*/

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::activitystream;
use crate::mastodon::account::Account as MAccount;

/// ActivityPub OroderdCollection.  
/// <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollection>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "@context")]
    pub context: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub object_type: String,
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

impl OrderedCollection {
    pub fn new(id: String, total_items: i64) -> Self {
        OrderedCollection {
            context: Some(activitystream::default_context()),
            id: Some(id.clone()),
            object_type: "OrderedCollection".to_string(),
            total_items: total_items,
            first: Some(format!("{}?page=1", id)),
            ..Default::default()
        }
    }
}
