// https://docs.joinmastodon.org/entities/Account/

use chrono::offset::Utc;
use chrono::DateTime;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Builder, Debug, Serialize, Deserialize, PartialEq, Eq, Clone,
)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group: bool,
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

impl Account {
    pub async fn build(id: String) -> anyhow::Result<Account> {
        let acct_tbl = crate::table::account::Account::get_with_id(id)
            .await
            .unwrap_or_default();

        let bot: bool = match acct_tbl.actor_type.unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let account = Account {
            id: acct_tbl.uuid.unwrap(),
            username: acct_tbl.username.clone().unwrap(),
            acct: format!(
                "{}@{}",
                acct_tbl.username.unwrap(),
                acct_tbl.domain.unwrap()
            ),
            display_name: acct_tbl.display_name.unwrap(),
            locked: acct_tbl.locked.unwrap(),
            bot: bot,
            discoverable: acct_tbl.discoverable.unwrap(),
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
        Ok(account)
    }

    pub async fn build_with_username(
        username: String,
    ) -> anyhow::Result<Account> {
        let acct_tbl =
            crate::table::account::Account::get_with_username(username)
                .await
                .unwrap_or_default();

        let bot: bool = match acct_tbl.actor_type.unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let account = Account {
            id: acct_tbl.uuid.unwrap(),
            username: acct_tbl.username.clone().unwrap(),
            acct: format!(
                "{}@{}",
                acct_tbl.username.unwrap(),
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
        Ok(account)
    }

    pub async fn to_json_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self).unwrap())
    }
}
