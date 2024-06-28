// https://docs.joinmastodon.org/entities/Filter/

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

use crate::mastodon::filter_result::FilterResult;
use crate::mastodon::filter_keyword::FilterKeyword;
use crate::mastodon::filter_status::FilterStatus;


#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Filter {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub title: String,
    pub context: Vec<Description>,
    pub expires_at: String,
    pub filter_action: FilterAction,
    pub keywords: Vec<FilterKeyword>,
    pub statuses: Vec<FilterStatus>,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]

pub enum Description {
    #[default]
    Home,
    Notifications,
    Public,
    Thread,
    Account,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum FilterAction {
    #[default]
    Warn,
    Hide,
}
