use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::variables;

use crate::mastodon::account::{Account, Get};
use crate::mastodon::username::Username;

// https://docs.joinmastodon.org/methods/instance/#v1
// https://docs.joinmastodon.org/entities/V1_Instance

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Instance {
    uri: String,
    title: String,
    short_description: String,
    description: String,
    email: String,
    version: String,
    urls: Option<Urls>,
    stats: Option<Stats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    languages: Option<Value>,
    registrations: bool,
    approval_required: bool,
    invites_enabled: bool,
    configuration: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_account: Option<Account>,
    rules: Vec<String>,
}

impl Into<String> for Instance {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for Instance {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Urls {
    streaming_api: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Stats {
    user_count: i64,
    status_count: i64,
    domain_count: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct MediaAttachment {
    supported_mime_types: Vec<String>,
    image_size_limit: i64,
    image_matrix_limit: i64,
    video_size_limit: i64,
    video_frame_rate_limit: i32,
    video_matrix_limit: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Polls {
    max_options: usize,
    max_characters_per_option: usize,
    min_expiration: usize,
    max_expiration: usize,
}

impl Instance {
    pub async fn build() -> Instance {
        let settings = crate::table::setting::Setting::all().await.unwrap();
        let username =
            settings.get("site_contact_username").unwrap().to_owned();
        let account =
            crate::mastodon::account::Account::get(Username(username))
                .await
                .unwrap();

        Instance {
            uri: variables::get("domain").unwrap(),
            title: settings.get("site_title").unwrap().to_owned(),
            short_description: settings
                .get("site_short_description")
                .unwrap()
                .to_owned(),
            description: "".to_string(),
            email: settings.get("site_contact_email").unwrap().to_owned(),
            version: settings.get("site_version").unwrap().to_string(),
            urls: None,
            stats: Some(Stats {
                user_count: crate::table::user::User::user_count()
                    .await
                    .unwrap(),
                status_count: 0,
                domain_count: 1,
            }),
            contact_account: Some(account),
            ..Default::default()
        }
    }
}
