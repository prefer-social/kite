//! inbox log is temporary data store

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct InboxLog {
    pub rowid: Option<i64>,
    pub uid: Option<String>,
    pub valid_sig: Option<String>,
    pub sig_header: Option<String>,
    pub headers: Option<String>,
    pub body: Option<String>,
    pub created_at: i64,
}

impl InboxLog {
    pub async fn put(
        valid_sig: String,
        sig_header: String,
        hostname: String,
        body: String,
    ) -> Result<()> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        sqlx::query("INSERT INTO inbox_log (uuid, valid_sig, sig_header, hostname, body) VALUES ($1, $2, $3, $4, $5)")
            .bind(Uuid::now_v7().to_string())
            .bind(valid_sig)
            .bind(sig_header)
            .bind(hostname)
            .bind(body)
            .execute(&sqlx_conn)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<InboxLog>>;
}

#[async_trait]
impl Get<(String, String)> for InboxLog {
    async fn get((key, val): (String, String)) -> Result<Vec<InboxLog>> {
        let query_template =
            format!("SELECT rowid, * FROM inbox_log WHERE {} = ?", key);
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
