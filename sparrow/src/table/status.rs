use anyhow::Result;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Deserialize, Serialize, PartialEq, Default, sqlx::FromRow,
)]
pub struct Status {
    pub rowid: String,
    pub uid: String,
    pub uri: Option<String>,
    pub text: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub in_reply_to_id: String,
    pub reblog_of_id: String,
    pub url: String,
    pub sensitive: bool,
    pub visibility: i64,
    pub spoiler_text: String,
    pub reply: bool,
    pub language: String,
    pub conversation_id: String,
    pub local: bool,
    pub account_id: String,
    pub application_id: String,
    pub in_reply_to_account_id: String,
    pub poll_id: String,
    pub deleted_at: i64,
    pub edited_at: i64,
    pub trendable: bool,
    pub ordered_media_attachment_ids: String,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Status>>;
}

#[async_trait]
impl Get<(String, String)> for Status {
    async fn get((key, val): (String, String)) -> Result<Vec<Status>> {
        let query_template =
            format!("SELECT rowid, * FROM status WHERE {} = ?", key);
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
