// Returns: CredentialAccount (https://docs.joinmastodon.org/entities/Account/#CredentialAccount)

use crate::mastodon::uid::Uid;
use crate::mastodon::username::Username;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CredentialAccount {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: Uid,
    pub username: Username,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub note: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    pub url: Option<String>,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub last_status_at: Option<DateTime<Utc>>,
    pub emojis: Vec<String>,
    pub fields: Vec<Field>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl From<&crate::mastodon::account::Account> for CredentialAccount {
    fn from(mas_account: &crate::mastodon::account::Account) -> Self {
        let fields: Vec<Field> = Vec::new();

        let credential_account = CredentialAccount {
            uid: mas_account.uid.to_owned(),
            username: mas_account.username.clone().into(),
            acct: mas_account.acct.to_owned(),
            display_name: mas_account.display_name.to_owned(),
            locked: mas_account.locked,
            bot: mas_account.bot,
            discoverable: mas_account.discoverable,
            group: mas_account.group,
            created_at: mas_account.created_at,
            note: mas_account.note.to_owned(),
            url: mas_account.url.to_owned(),
            avatar: mas_account.avatar.to_owned(),
            avatar_static: mas_account.avatar_static.to_owned(),
            header: mas_account.header.to_owned(),
            header_static: mas_account.header_static.to_owned(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            last_status_at: mas_account.last_status_at,
            emojis: mas_account.emojis.clone().unwrap_or_default(),
            source: None,
            fields: fields,
            role: None,
        };

        credential_account
    }
}

impl Into<String> for CredentialAccount {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for CredentialAccount {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Source {
    pub privacy: String,
    pub sensitive: bool,
    pub language: String,
    pub note: String,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: String,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: String,
    pub color: String,
    pub highlighted: bool,
}

impl CredentialAccount {}
