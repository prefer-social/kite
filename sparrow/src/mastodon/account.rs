// https://docs.joinmastodon.org/entities/Account/
use anyhow::Result;
use async_trait::async_trait;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::mastodon::uid::Uid;
use crate::mastodon::username::Username;
use crate::table::account::Get as _;

pub mod search;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Account {
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

impl Account {
    // pub async fn get(uid: Uid) -> anyhow::Result<Account> {
    //
    //     let account = crate::table::account::Account::get(uid)
    //          .await
    //          .unwrap_or_default();
    //     let account = account.into_iter().next().unwrap();
    //
    //     let bot: bool = match account.actor_type.unwrap().as_str() {
    //         "service" => true,
    //         _ => false,
    //     };
    //
    //     let account = Account {
    //         uid: account.uid,
    //         username: account.username.clone(),
    //         acct: format!(
    //             "{}@{}",
    //             account.username,
    //             account.domain.unwrap()
    //         ),
    //         display_name: account.display_name.unwrap(),
    //         locked: account.locked.unwrap(),
    //         bot: bot,
    //         discoverable: account.discoverable.unwrap(),
    //         created_at: DateTime::from_timestamp(
    //             account.created_at.unwrap(),
    //             0,
    //         )
    //         .unwrap(),
    //         note: account.note.unwrap(),
    //         url: account.url,
    //         avatar: account.avatar_remote_url.clone().unwrap(),
    //         avatar_static: account.avatar_remote_url.unwrap(),
    //         header: account.header_remote_url.clone().unwrap(),
    //         header_static: account.header_remote_url.unwrap(),
    //         followers_count: 0,
    //         following_count: 0,
    //         statuses_count: 0,
    //         ..Default::default()
    //     };
    //     Ok(account)
    // }

    pub async fn get_user(
        username: Username,
    ) -> anyhow::Result<crate::table::user::User> {
        let account = crate::table::account::Account::get(username)
            .await
            .unwrap_or_default();

        let account_id = account.into_iter().next().unwrap().uid;

        let users =
            crate::table::user::User::get_by_account_id(account_id).await?;
        let user = users.first().unwrap().to_owned();
        Ok(user)
    }

    pub async fn get_user_by_userid(
        userid: String,
    ) -> Result<Option<crate::table::user::User>> {
        let user = crate::table::user::User::get(userid)
            .await
            .unwrap_or_default();
        Ok(user)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(a: T) -> Result<Account>;
}

#[async_trait]
impl Get<Uid> for Account {
    async fn get(uid: Uid) -> Result<Account> {
        let accounts = crate::table::account::Account::get(uid)
            .await
            .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Ok(self::Account::from(acct_tbl))
    }
}

#[async_trait]
impl Get<Username> for Account {
    async fn get(username: Username) -> Result<Account> {
        let accounts = crate::table::account::Account::get(username)
            .await
            .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Ok(self::Account::from(acct_tbl))
    }
}
