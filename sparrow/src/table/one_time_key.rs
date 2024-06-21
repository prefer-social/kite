use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OneTimeKey {
    pub rowid: Option<i64>,
    pub uuid: String,
    pub device_id: i64,
    pub key: String,
    pub signature: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl OneTimeKey {
    pub async fn all() -> Result<Vec<OneTimeKey>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let otk: Vec<OneTimeKey> =
            sqlx::query_as("SELECT rowid, * FROM one_time_key")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(otk)
    }
}
