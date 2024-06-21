use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::mastodon::account::Account;
use crate::mastodon::application::Application;

use super::media::MediaAttachment;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: String,
    pub created_at: String,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub sensitive: bool,
    #[serde(default)]
    pub spoiler_text: Option<String>,
    pub visibility: String,
    pub language: String,
    pub uri: String,
    pub url: String,
    pub replies_count: u32,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub favourited: bool,
    pub reblogged: bool,
    pub muted: bool,
    pub bookmarked: bool,
    pub content: String,
    #[serde(default)]
    pub reblog: Option<String>,
    pub application: Application,
    pub account: Account,
    pub media_attachments: Option<Vec<MediaAttachment>>,
    pub mentions: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub emojis: Option<Vec<String>>,
    pub card: Option<String>,
    pub poll: Option<String>,
}

impl Status {
    //whose, how many/when
    pub async fn get(a: Account) {}
}
