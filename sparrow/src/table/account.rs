//! account table
//!

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sqlx::Connection as dbcon;
use std::time::{SystemTime, UNIX_EPOCH};
use struct_iterable::Iterable;
use url::Url;

/// DB Account table struct
#[derive(
    Serialize,
    Deserialize,
    Default,
    Clone,
    Debug,
    PartialEq,
    sqlx::FromRow,
    Iterable,
)]
pub struct Account {
    /// rowid from sqlite
    pub rowid: i64,
    /// uid, uuid v7 format. Also when transformed to json, this filed becomes `id`
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String, // not null, primary key
    /// username, default value is "" and not null
    pub username: String,
    /// domain. Local user when None
    pub domain: Option<String>,
    /// private pem key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// public pem key, not null, default is ""
    pub public_key: String,
    /// when field was crated not null, Unix time
    pub created_at: i64,
    /// when field was updated, not null, Unix time
    pub updated_at: i64,
    /// Note filed, default "", not null
    pub note: String,
    /// display name, default "", not null
    pub display_name: String,
    /// uri, default "", not null
    pub uri: String,
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
    /// default(FALSE), not null
    pub locked: Option<bool>,
    /// default(""), not null
    pub header_remote_url: Option<String>,
    pub last_webfingered_at: Option<i64>,
    /// default(""), not null
    pub inbox_url: Option<String>,
    /// default(""), not null
    pub outbox_url: Option<String>,
    /// default(""), not null
    pub shared_inbox_url: Option<String>,
    /// default(""), not null
    pub following_url: Option<String>,
    /// default(""), not null
    pub followers_url: Option<String>,
    /// default("ostatus"), not null
    pub protocol: Option<i64>,
    /// default(FALSE), not null
    pub memorial: Option<bool>,
    pub moved_to_account_id: Option<i64>,
    pub featured_collection_url: Option<String>,
    pub fields: Option<String>,
    pub actor_type: Option<String>,
    pub discoverable: Option<bool>,
    /// is an Array
    pub also_known_as: Option<String>,
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
    /// default(FALSE), not null
    pub indexable: Option<bool>,
}

impl Account {
    /// returns all Account rows
    pub async fn all() -> Result<Vec<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<Account> =
            sqlx::query_as("SELECT rowid, * FROM account")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(accounts)
    }

    /// To get federation_id
    pub async fn federation_id(self: &Self) -> Result<String> {
        let username = self.username.clone();
        let domain = self.domain.clone();
        Ok(format!("{}@{}", username, domain.unwrap()))
    }

    /// Get Account struct by account(username and domain)
    pub async fn get_with_account(
        username: String,
        domain: String,
    ) -> Result<Option<Account>> {
        let sqlx_conn = dbcon::open_default()?;
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

/// I am a trait Get<T>
#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Account>>;
}

/// Very generic table Get function
/// Geting (key: String, val: String).
/// This goes `SELECT * FROM some_table WHERER key = val`
#[async_trait]
impl Get<(String, String)> for Account {
    async fn get((key, val): (String, String)) -> Result<Vec<Account>> {
        let query_template =
            format!("SELECT rowid, * FROM account WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}

#[async_trait(?Send)]
pub trait Put<T> {
    async fn put(arg: T) -> Result<()>;
}

#[async_trait(?Send)]
impl Put<crate::activitypub::person_actor::PersonActor> for Account {
    async fn put(
        actor: crate::activitypub::person_actor::PersonActor,
    ) -> Result<()> {
        let account = Self::try_from(actor).unwrap();

        for (name, value) in account.iter() {
            println!("{:?}", name);
        }

        let a = r#"INSERT INTO account (
         uid ,
        username ,
    domain ,
    public_key,
    note, 
    display_name,
    uri,
    url,
    avatar_file_name,
    avatar_content_type,
    avatar_file_size,
    avatar_updated_at,
    header_file_name,
    header_content_type,
    header_file_size,
    header_updated_at,
    avatar_remote_url,
    locked BOOLEAN,
    header_remote_url,
    last_webfingered_at,
    inbox_url,
    outbox_url,
    shared_inbox_url,
    followers_url,
    following_url,
    protocol,
    memorial,
    moved_to_account_id bigint,
    featured_collection_url,
    fields,
    actor_type,
    discoverable,
    also_known_as,
    silenced_at,
    suspended_at,
    hide_collections,
    avatar_storage_schema_version,
    header_storage_schema_version,
    devices_url,
    suspension_origin,
    sensitized_at,
    trendable,
    reviewed_at,
    requested_review_at,
    indexable,        
        ) VALUES"#;

        let sqlx_conn = dbcon::open_default()?;

        //sqlx::query(
        //     r#"INSERT INTO account
        //() VALUES
        // (?, ?, ?, ?, ?, ?, ?)"#,
        // )
        // .bind(token_id.clone())
        // .bind(crate::utils::random_string(64).await)
        // .bind(crate::utils::random_string(64).await)
        // .bind(scope)
        // .bind(application_id)
        // .bind(resource_owner_id)
        // .bind(last_used_ip)
        // .execute(&sqlx_conn)
        // .await?;
        Ok(())
    }
}

impl TryFrom<crate::activitypub::person_actor::PersonActor> for Account {
    type Error = &'static str;

    /// (Person)Actor to Account
    fn try_from(
        actor: crate::activitypub::person_actor::PersonActor,
    ) -> Result<Self, Self::Error> {
        let current_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let avatar_remote_url = match &actor.icon {
            Some(i) => Some(i.to_owned().url),
            None => None,
        };

        let avatar_content_type = match &actor.icon {
            Some(i) => Some(i.to_owned().media_type),
            None => None,
        };
        let header_remote_url = match &actor.image {
            Some(i) => Some(i.to_owned().url),
            None => None,
        };
        let header_content_type = match &actor.image {
            Some(i) => Some(i.to_owned().media_type),
            None => None,
        };

        let account = Account {
            uid: uuid::Uuid::now_v7().to_string(),
            username: actor.preferred_username,
            domain: Some(
                Url::parse(actor.id.as_str())
                    .unwrap()
                    .domain()
                    .unwrap()
                    .to_string(),
            ),
            public_key: actor.public_key.public_key_pem,
            created_at: current_epoch, // not null
            updated_at: current_epoch, // not null
            note: actor.summary,       // default(""), not null
            display_name: actor.name,  // default(""), not null
            uri: actor.id,             // default(""), not null
            url: Some(actor.url),
            avatar_content_type: avatar_content_type,
            header_content_type: header_content_type,
            avatar_remote_url: avatar_remote_url,
            header_remote_url: header_remote_url,
            last_webfingered_at: Some(current_epoch),
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
        Ok(account)
    }
}
