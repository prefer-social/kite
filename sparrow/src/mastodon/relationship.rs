// https://docs.joinmastodon.org/entities/Relationship/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mastodon::custom_emoji::CustomEmoji;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Relationship {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showing_reblogs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifying: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followed_by: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_by: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muting_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_blocking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endorsed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
