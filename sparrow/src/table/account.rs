use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Account {
    pub rowid: Option<i64>,
    pub uuid: Option<String>, // not null, primary key
    pub username: Option<String>, // default(""), not null
    pub domain: Option<String>,
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

    pub async fn get_with_id(uuid: String) -> Result<Account> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let account: Account =
            sqlx::query_as("SELECT rowid, * FROM account WHERE uuid = ?")
                .bind(uuid)
                .fetch_one(&sqlx_conn)
                .await?;
        Ok(account)
    }

    pub async fn select() -> Result<()> {
        let table_hashmaps =
            crate::table::utils::hashmap_from_table("account".to_string())
                .await
                .unwrap()
                .unwrap();

        for table_hashmap in table_hashmaps {
            let foo = serde_json::to_string(&table_hashmap).unwrap();
            let account: Value = serde_json::from_str(foo.as_str()).unwrap();

            tracing::debug!("{:?}", account);
        }

        Ok(())
    }

    pub async fn get_with_username(username: String) -> Result<Self> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let account: Account =
            sqlx::query_as("SELECT rowid, * FROM account WHERE username = ?")
                .bind(username)
                .fetch_one(&sqlx_conn)
                .await?;
        Ok(account)
    }

    pub async fn get_with_account_id(
        account_id: String,
    ) -> anyhow::Result<Option<Self>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as::<_, Account>(
            "SELECT rowid, * FROM account WHERE uuid = ?",
        )
        .bind(account_id)
        .fetch_all(&sqlx_conn)
        .await?;
        let account = accounts.into_iter().next();
        Ok(account)
    }

    pub async fn get_with_account(
        account: String,
    ) -> Result<Option<Vec<Self>>> {
        let u = account.split("@").collect::<Vec<&str>>();

        println!("{:?}", u);

        let username = u[0];
        let domain = u[1];

        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = match sqlx::query_as(
            "SELECT rowid, * FROM account WHERE username = ? AND domain = ?",
        )
        .bind(username)
        .bind(domain)
        .fetch_all(&sqlx_conn)
        .await
        {
            Ok(s) => s,
            _ => return Ok(None),
        };

        Ok(Some(accounts))
    }

    pub async fn get_userid(username: String) -> Option<u64> {
        let qr = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT id FROM account WHERE username = ?",
                &[SV::Text(username)],
            )
            .await
            .to_owned();
        let row = qr.rows().next().unwrap();
        row.get::<u64>("id")
    }

    pub async fn federation_id(self: &Self) -> Result<String> {
        let username = self.username.clone();
        let domain = self.domain.clone();
        Ok(format!("{}@{}", username.unwrap(), domain.unwrap()))
    }
}
