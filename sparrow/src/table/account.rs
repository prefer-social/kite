use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

use crate::mastodon::uid::Uid;
use crate::mastodon::username::Username;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct Account {
    pub rowid: i64,
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String, // not null, primary key
    pub username: String, // default(""), not null
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    pub public_key: Option<String>, // default(""), not null
    pub created_at: Option<i64>,    // not null
    pub updated_at: Option<i64>,    // not null
    pub note: Option<String>,       // default(""), not null
    pub display_name: Option<String>, // default(""), not null
    pub uri: String,                // default(""), not null
    pub url: Option<String>,
    pub avatar_file_name: Option<String>,
    pub avatar_content_type: Option<String>,
    pub avatar_file_size: Option<i64>,
    pub avatar_updated_at: Option<i64>,
    pub header_file_name: Option<String>,
    pub header_content_type: Option<String>,
    pub header_file_size: Option<i64>,
    pub header_updated_at: Option<i64>,
    pub avatar_remote_url: Option<String>,
    pub locked: Option<bool>, // default(FALSE), not null
    pub header_remote_url: Option<String>, // default(""), not null
    pub last_webfingered_at: Option<i64>,
    pub inbox_url: Option<String>, // default(""), not null
    pub outbox_url: Option<String>, // default(""), not null
    pub shared_inbox_url: Option<String>, // default(""), not null
    pub following_url: Option<String>, // default(""), not null
    pub followers_url: Option<String>, // default(""), not null
    pub protocol: Option<i64>,     // default("ostatus"), not null
    pub memorial: Option<bool>,    // default(FALSE), not null
    pub moved_to_account_id: Option<i64>,
    pub featured_collection_url: Option<String>,
    pub fields: Option<String>,
    pub actor_type: Option<String>,
    pub discoverable: Option<bool>,
    pub also_known_as: Option<String>, // is an Array
    pub silenced_at: Option<i64>,
    pub suspended_at: Option<i64>,
    pub hide_collections: Option<bool>,
    pub avatar_storage_schema_version: Option<i64>,
    pub header_storage_schema_version: Option<i64>,
    pub devices_url: Option<String>,
    pub suspension_origin: Option<i64>,
    pub sensitized_at: Option<i64>,
    pub trendable: Option<bool>,
    pub reviewed_at: Option<i64>,
    pub requested_review_at: Option<i64>,
    pub indexable: Option<bool>, // default(FALSE), not null
}

impl Account {
    pub async fn all() -> Result<Vec<Account>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts: Vec<Account> =
            sqlx::query_as("SELECT rowid, * FROM account")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(accounts)
    }

    pub async fn federation_id(self: &Self) -> Result<String> {
        let username = self.username.clone();
        let domain = self.domain.clone();
        Ok(format!("{}@{}", username, domain.unwrap()))
    }

    pub async fn get_with_account(
        username: String,
        domain: String,
    ) -> Result<Option<Account>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts: Vec<Account> = sqlx::query_as(
            "SELECT rowid, * FROM account WHERE username = ? AND domain = ?",
        )
        .bind(username)
        .bind(domain)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(Some(accounts.first().unwrap().to_owned()))
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Account>>;
}

#[async_trait]
impl Get<(String, String)> for Account {
    async fn get((key, val): (String, String)) -> Result<Vec<Account>> {
        let query_template =
            format!("SELECT rowid, * FROM account WHERE {} = ?", key);
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}


impl From<crate::activitypub::person_actor::PersonActor> for Account {
    fn from(actor: crate::activitypub::person_actor::PersonActor) -> Self {

        let account = Account {
            uid: actor.id,
            username: actor.
            domain: Option<String>,
            public_key: actor.public_key,
            created_at: Option<i64>,    // not null
            updated_at: Option<i64>,    // not null
            note: Option<String>,       // default(""), not null
            display_name: Option<String>, // default(""), not null
            uri: String,                // default(""), not null
            url: Some(actor.url),
            avatar_file_name: Option<String>,
            avatar_content_type: Some(actor.icon.unwrap_or_default().media_type),
            avatar_updated_at: Option<i64>,
            header_file_name: Option<String>,
            header_content_type: Some(actor.image.unwrap_or_default().media_type),,
            header_updated_at: Option<i64>,
            avatar_remote_url: Some(actor.icon.unwrap_or_default().url),
            header_remote_url: Some(actor.image.unwrap_or_default().url), // default(""), not null
            last_webfingered_at: Option<i64>,
            inbox_url: Some(actor.inbox),
            outbox_url: Some(actor.outbox),
            shared_inbox_url: Some(actor.endpoints.shared_inbox), // default(""), not null
            following_url: Some(actor.following),
            followers_url: Some(actor.followers), // default(""), not null
            memorial: actor.memorial,
            featured_collection_url: Some(actor.featured),
            actor_type: Some(actor.actor_type),
            discoverable: Some(actor.discoverable),
            devices_url: actor.devices,
            indexable: Some(actor.indexable),
            ..Default::default() // default(FALSE), not null
        };
        account
    }
}
