use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OnetimeKey {
    pub rowid: Option<i64>,
    pub uuid: String,
    pub device_id: i64,
    pub key: String,
    pub signature: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl OnetimeKey {
    pub async fn all() -> Result<Vec<OneTimeKey>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let otk: Vec<OneTimeKey> =
            sqlx::query_as("SELECT rowid, * FROM one_time_key")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(otk)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<OnetimeKey>>;
}

#[async_trait]
impl Get<(String, String)> for OnetimeKey {
    async fn get((key, val): (String, String)) -> Result<Vec<Setting>> {
        let query_template =
            format!("SELECT rowid, * FROM onetime_key WHERE {} = ?", key);
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
