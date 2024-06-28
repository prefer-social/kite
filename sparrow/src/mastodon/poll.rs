// https://docs.joinmastodon.org/entities/Poll/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mastodon::custom_emoji::CustomEmoji;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Poll {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: i64,
    pub voters_count: Option<i64>,
    pub options: Vec<PollOption>,
    pub emojis: Vec<CustomEmoji>,
    pub voted: Option<bool>,
    pub own_votes: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct PollOption {
    pub title: String,
    pub votes_count: Option<i64>,
}
