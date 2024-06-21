/*
    inbox log is temporary data store
    (moving to redis??)
*/

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;
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
