// https://docs.joinmastodon.org/entities/Status/

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::mastodon::{
    account::Account, custom_emoji::CustomEmoji, filter_result::FilterResult,
    media_attachement::MediaAttachement, poll::Poll,
    preview_card::PreviewCard,
};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub rowid: i64,
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub uri: String,
    pub created_at: DateTime<Utc>,
    pub account: Account,
    pub content: String,
    pub visibility: Visibility,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub media_attachments: Vec<MediaAttachement>,
    pub application: Option<HashMap<String, String>>,
    pub mentions: Vec<Metion>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<CustomEmoji>,
    pub reblogs_count: i64,
    pub favourites_count: i64,
    pub replies_count: i64,
    pub url: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Box<Status>,
    pub poll: Option<Poll>,
    pub card: Option<PreviewCard>,
    pub language: String,
    pub text: String,
    pub edited_at: String,
    pub favourited: Option<bool>,
    pub reblogged: Option<bool>,
    pub muted: Option<bool>,
    pub bookmarked: Option<bool>,
    pub pinned: Option<bool>,
    pub filtered: Vec<FilterResult>,
}

impl From<crate::table::status::Status> for Status {
    fn from(tbl: crate::table::status::Status) -> Self {
        let status = Status {
            ..Default::default()
        };
        status
    }
}

impl Into<String> for Status {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for Status {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

impl Status {
    //whose, how many/when
    pub async fn get(a: Account) {}
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum Visibility {
    #[default]
    Public,
    Unlisted,
    Private,
    Direct,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Metion {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

impl Status {
    pub async fn search(search_term: &String) -> Result<Vec<Status>> {
        Ok(Vec::new())
    }
}
