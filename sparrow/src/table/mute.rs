use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sqlx::sqlite::Connection as dbcon;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct Mute {
    pub uid: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub hide_notifications: bool,
    pub account_id: String,
    pub target_account_id: String,
    pub expires_at: i64,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Mute>>;
}

#[async_trait]
impl Get<(String, String)> for Mute {
    async fn get((key, val): (String, String)) -> Result<Vec<Mute>> {
        let query_template =
            format!("SELECT rowid, * FROM mute WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
