//! conversion_mute table  

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sqlx::Connection as dbcon;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct ConversationMute {
    pub rowid: i64,
    pub uid: String,
    pub conversation_id: String,
    pub account_id: String,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<ConversationMute>>;
}

#[async_trait]
impl Get<(String, String)> for ConversationMute {
    async fn get(
        (key, val): (String, String),
    ) -> Result<Vec<ConversationMute>> {
        let query_template = format!(
            "SELECT rowid, * FROM conversation_mute WHERE {} = ?",
            key
        );
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
