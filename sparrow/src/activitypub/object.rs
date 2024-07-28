//! ActivityPub Object struct
//! Resource: <https://www.w3.org/TR/activitypub/#obj>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::Debug;

/// ActivityPub Object Types
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum ObjectType {
    Follow,
    Accept,
    Reject,
    Undo,
    Delete,
    Note,
    Create,
    Replies,
    RsaSignature2017,
    OrderedCollection,
    OrderedCollectionPage,
    #[default]
    NotDefined,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ActicityPub Object base template
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Object<T>
where
    T: Debug + Serialize + ToString,
{
    #[serde(rename = "@context")]
    pub context: Value,
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    pub object: T,
}

impl<T> Object<T>
where
    T: Debug + Serialize + ToString,
{
    pub fn new(
        id: String,
        object_type: ObjectType,
        actor: String,
        published: Option<DateTime<Utc>>,
        to: Option<Vec<String>>,
        cc: Option<Vec<String>>,
        object: T,
    ) -> Self {
        Object {
            context: default_context(),
            id,
            object_type,
            actor,
            published,
            to,
            cc,
            object,
        }
    }
}

fn default_context() -> Value {
    serde_json::from_str("https://www.w3.org/ns/activitystreams").unwrap()
}
