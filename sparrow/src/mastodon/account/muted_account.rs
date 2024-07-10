//! MutedAccount: Return account that is muted    
//!
//! Based on Account but extra fields are added.  
//! Extra fileds: mute_expires_at
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Account/#MutedAccount>  

use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::str;

use crate::mastodon::account::field::Field;
use crate::mastodon::custom_emoji::CustomEmoji;
use crate::mastodon::uid::Uid;
use crate::mastodon::username::Username;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MutedAccount {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: Uid,
    pub username: Username,
    pub acct: String,
    pub url: String,
    pub display_name: String,
    pub note: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub locked: bool,
    pub fields: Vec<Field>,
    pub emojis: Vec<CustomEmoji>,
    pub bot: bool,
    pub group: bool,
    pub discoverable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noindex: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub last_status_at: DateTime<Utc>,
    pub statuses_count: u32,
    pub followers_count: u32,
    pub following_count: u32,
    #[serde(skip_serializing, skip_deserializing)]
    pub private_key: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub public_key: Option<String>,
    pub mute_expires_at: String,
}
