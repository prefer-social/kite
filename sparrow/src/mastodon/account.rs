// https://docs.joinmastodon.org/entities/Account/
use anyhow::Result;
use async_trait::async_trait;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::mastodon::uid::Uid;
use crate::mastodon::user::User;
use crate::mastodon::username::Username;
use crate::table::account::Get as _;

pub mod search;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Account {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: Uid,
    pub username: Username,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub note: String,
    pub url: Option<String>,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub last_status_at: Option<DateTime<Utc>>,
    pub emojis: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    #[serde(skip_serializing, skip_deserializing)]
    pub private_key: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub public_key: Option<String>,
}

impl Account {
    pub async fn default_user() -> Result<(Account, User)> {
        let du = crate::table::user::User::default_user()
            .await?
            .first()
            .unwrap()
            .to_owned();
        let user = crate::mastodon::user::User::from(du);
        let account_id =
            crate::mastodon::uid::Uid(user.account_id.clone().unwrap());
        let account = Self::get(account_id).await?;
        Ok((account, user))
    }
}

impl From<crate::table::account::Account> for Account {
    fn from(acct_tbl: crate::table::account::Account) -> Self {
        let bot: bool = match acct_tbl.actor_type.unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let account = Account {
            uid: acct_tbl.uid.into(),
            username: acct_tbl.username.clone().into(),
            acct: format!(
                "{}@{}",
                acct_tbl.username.to_string(),
                acct_tbl.domain.unwrap()
            ),
            display_name: acct_tbl.display_name.unwrap(),
            locked: acct_tbl.locked.unwrap(),
            bot: bot,
            discoverable: acct_tbl.discoverable.unwrap_or_default(),
            created_at: DateTime::from_timestamp(
                acct_tbl.created_at.unwrap(),
                0,
            )
            .unwrap(),
            note: acct_tbl.note.unwrap(),
            url: acct_tbl.url,
            avatar: acct_tbl.avatar_remote_url.clone().unwrap(),
            avatar_static: acct_tbl.avatar_remote_url.unwrap(),
            header: acct_tbl.header_remote_url.clone().unwrap(),
            header_static: acct_tbl.header_remote_url.unwrap(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            ..Default::default()
        };
        account
    }
}

impl Into<String> for Account {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for Account {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(a: T) -> Result<Account>;
}

#[async_trait]
impl Get<Uid> for Account {
    async fn get(uid: Uid) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "uid".to_string(),
            uid.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Ok(self::Account::from(acct_tbl))
    }
}

#[async_trait]
impl Get<Username> for Account {
    async fn get(username: Username) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "username".to_string(),
            username.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Ok(self::Account::from(acct_tbl))
    }
}
