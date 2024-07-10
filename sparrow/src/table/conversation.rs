//! converstion table

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sqlx::Connection as dbcon;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct Conversation {
    pub rowid: i64,
    pub uid: String,
    pub uri: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Conversation>>;
}

#[async_trait]
impl Get<(String, String)> for Conversation {
    async fn get((key, val): (String, String)) -> Result<Vec<Conversation>> {
        let query_template =
            format!("SELECT rowid, * FROM conversation WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
