use chrono::format::strftime::StrftimeItems;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{QueryResult, Value as SV},
    variables,
};

use crate::mastodon::account::Account;

// https://docs.joinmastodon.org/methods/instance/#v1
// https://docs.joinmastodon.org/entities/V1_Instance

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    uri: String,
    title: String,
    short_description: String,
    description: String,
    email: String,
    version: String,
    urls: Option<Urls>,
    stats: Option<Stats>,
    thumbnail: Option<String>,
    languages: Value,
    registration: bool,
    approval_required: bool,
    invites_enabled: bool,
    configuration: Value,
    contact_account: Option<Account>,
    rules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Urls {
    streaming_api: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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
            crate::mastodon::account::Account::build_with_username(username)
                .await
                .unwrap();

        Instance {
            uri: variables::get("domain").unwrap(),
            title: settings.get("site_title").unwrap().to_owned(),
            short_description: settings
                .get("site_short_description")
                .unwrap()
                .to_owned(),
            email: settings.get("site_contact_email").unwrap().to_owned(),
            contact_account: Some(account),
            stats: Some(Stats {
                user_count: crate::table::user::User::user_count()
                    .await
                    .unwrap(),
                status_count: 0,
                domain_count: 1,
            }),
            ..Default::default()
        }
    }

    pub async fn to_json_string(self) -> anyhow::Result<String> {
        let mock = r#"{
            "uri": "seungjin.ap.dev.seungjin.net",
            "title": "AP dev server",
            "short_description": "short description for ap dev server",
            "description": "",
            "email": "seungjin@duck.com",
            "version": "0.0.1",
            "urls": {
                "streaming_api": "wss://seungjin.ap.dev.seungjin.net"
            },
            "stats": {
                "user_count": 1,
                "status_count": 1,
                "domain_count": 1
            },
            "thumbnail": "https://media-mstd.seungjin.net/site_uploads/files/000/000/001/@1x/b3e8da26f8f02054.png",
            "languages": [
                "en"
            ],
            "registrations": false,
            "approval_required": false,
            "invites_enabled": true,
            "configuration": {
                "accounts": {
                "max_featured_tags": 10
            },
            "statuses": {
            "max_characters": 500,
            "max_media_attachments": 4,
            "characters_reserved_per_url": 23
            },
            "media_attachments": {
            "supported_mime_types": [
                "image/jpeg",
                "image/png",
                "image/gif"
            ],
            "image_size_limit": 16777216,
            "image_matrix_limit": 33177600,
            "video_size_limit": 103809024,
            "video_frame_rate_limit": 120,
            "video_matrix_limit": 8294400
            },
            "polls": {
            "max_options": 4,
            "max_characters_per_option": 50,
            "min_expiration": 300,
            "max_expiration": 2629746
            }
        },
        "contact_account": {
            "id": "109737937659013254",
            "username": "seungjin",
            "acct": "seungjin",
            "display_name": "seungjin",
            "locked": false,
            "bot": false,
            "discoverable": false,
            "group": false,
            "created_at": "2023-01-23T00:00:00.000Z",
            "note": "<p>The future is already here <br />- it&#39;s just not very evenly distributed.<br />William Gibson</p>",
            "url": "https://mstd.seungjin.net/@seungjin",
            "uri": "https://mstd.seungjin.net/users/seungjin",
            "avatar": "https://media-mstd.seungjin.net/accounts/avatars/109/737/937/659/013/254/original/626c9187e341632b.jpg",
            "avatar_static": "https://media-mstd.seungjin.net/accounts/avatars/109/737/937/659/013/254/original/626c9187e341632b.jpg",
            "header": "https://media-mstd.seungjin.net/accounts/headers/109/737/937/659/013/254/original/9a714d77de20ae26.jpg",
            "header_static": "https://media-mstd.seungjin.net/accounts/headers/109/737/937/659/013/254/original/9a714d77de20ae26.jpg",
            "followers_count": 95,
            "following_count": 327,
            "statuses_count": 1878,
            "last_status_at": "2023-12-25",
            "noindex": true,
            "emojis": [],
            "roles": [
            {
                "id": "3",
                "name": "Owner",
                "color": ""
            }
            ],
            "fields": [
            {
                "name": "Speaks",
                "value": "English, 한국어",
                "verified_at": null
            },
            {
                "name": "Pronouns",
                "value": "He / His",
                "verified_at": null
            },
            {
                "name": "General interests",
                "value": "Environment, Animal right, Coffee, Tea, Poor man&#39;s food",
                "verified_at": null
            },
            {
                "name": "Technical interests",
                "value": "Rust Programming Language, WebAssembly, Cloud Computing",
                "verified_at": null
            }
            ]
        },
        "rules": [
            {
            "id": "3",
            "text": "Single user(and his bots) server. "
            }
        ]
        }"#.to_string();
        let inst = serde_json::to_string(&self).unwrap();
        Ok(mock)
    }
}
