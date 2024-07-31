//! inbox_log table  
//!
//! Log inbox received messages

use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use spin_sqlx::sqlite::Connection as dbcon;
use std::fmt::Debug;
use uuid::Uuid;

use crate::activitystream::activity::follow::Follow as FollowActivity;
use crate::activitystream::actor::person::Person as PersonActor;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct ActivityLog {
    pub rowid: Option<i64>,
    pub uid: Option<String>,
    pub sig_header: Option<String>,
    pub headers: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub created_at: i64,
}

impl ActivityLog {
    /// Add ActivityPub object into Database table.  
    pub async fn put(
        sig_header: String,
        hostname: String,
        body: String,
        status: Option<String>,
    ) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;
        sqlx::query("INSERT INTO activity_log (uid, sig_header, hostname, body, status) VALUES ($1, $2, $3, $4, $5)")
            .bind(Uuid::now_v7().to_string())
            .bind(sig_header)
            .bind(hostname)
            .bind(body)
            .bind(status)
            .execute(&sqlx_conn)
            .await?;
        Ok(())
    }

    /// returns Activitypub object with its id.  
    pub async fn get_with_id(id: &str) -> Result<Option<Value>> {
        let sqlx_conn = dbcon::open_default()?;
        // SELECT rowid, * FROM activity_log WHERE JSON_EXTRACT(body, '$.id') = ?
        let a: Vec<Self> = sqlx::query_as(
            "SELECT rowid, * FROM activity_log WHERE JSON_EXTRACT(body, '$.id') = ?",
        )
        .bind(id)
        .fetch_all(&sqlx_conn)
        .await?;

        match a.last() {
            Some(x) => {
                let a = x.body.clone().unwrap();
                let b = serde_json::from_str::<Value>(a.as_str()).unwrap();
                Ok(Some(b))
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<ActivityLog>>;
}

#[async_trait]
impl Get<(String, String)> for ActivityLog {
    async fn get((key, val): (String, String)) -> Result<Vec<ActivityLog>> {
        let query_template =
            format!("SELECT rowid, * FROM activity_log WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
