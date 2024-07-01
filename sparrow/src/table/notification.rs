use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct Notification {
    pub rowid: i64,
    pub uid: String,
    pub activity_id: String,
    pub activity_type: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub account_id: String,
    pub from_account_id: String,
    pub notification_type: String,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Notification>>;
}

#[async_trait]
impl Get<(String, String)> for Notification {
    async fn get((key, val): (String, String)) -> Result<Vec<Notification>> {
        let query_template =
            format!("SELECT rowid, * FROM notification WHERE {} = ?", key);
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
