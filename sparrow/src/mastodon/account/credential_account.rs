// https://docs.joinmastodon.org/entities/Account/#CredentialAccount

use anyhow::Result;
use async_trait::async_trait;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::str;

use crate::mastodon::account::field::Field;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Role;
use crate::mastodon::account::Source;
use crate::mastodon::custom_emoji::CustomEmoji;
use crate::mastodon::uid::Uid;
use crate::mastodon::username::Username;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CredentialAccount {
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
    // CredentialAccount entity attributes
    //pub role: Role,
    //pub source: Source,
}

impl TryFrom<MAccount> for CredentialAccount {
    type Error = anyhow::Error;
    fn try_from(a: MAccount) -> Result<Self, Self::Error> {
        //let role;
        //let source;

        Ok(CredentialAccount {
            uid: a.uid,
            username: a.username,
            acct: a.acct,
            url: a.url,
            display_name: a.display_name,
            note: a.note,
            avatar: a.avatar,
            avatar_static: a.avatar_static,
            header: a.header,
            header_static: a.header_static,
            locked: a.locked,
            fields: a.fields,
            emojis: a.emojis,
            bot: a.bot,
            group: a.group,
            discoverable: a.discoverable,
            noindex: a.noindex,
            moved: a.moved,
            suspended: a.suspended,
            limited: a.limited,
            created_at: a.created_at,
            last_status_at: a.last_status_at,
            statuses_count: a.statuses_count,
            followers_count: a.followers_count,
            following_count: a.following_count,
            private_key: a.private_key,
            public_key: a.public_key,
            // CredentialAccount entity attributes
            //role: role,
            //source: source,
        })
    }
}
