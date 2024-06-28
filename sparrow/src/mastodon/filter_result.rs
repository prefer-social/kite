// https://docs.joinmastodon.org/entities/FilterResult/

use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::mastodon::filter::Filter;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FilterResult {
    pub filter: Filter,
    pub keyword_matches: Vec<String>,
    pub status_matches: Vec<String>,
}
