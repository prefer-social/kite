//! follow table

use anyhow::{Error, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sqlx::sqlite::Connection as dbcon;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::mastodon::account::uid::Uid as AccountUid;

/// follow table in Database
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct Follow {
    pub rowid: Option<i64>,
    pub uid: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub account_uid: Option<String>,
    pub target_account_uid: Option<String>,
    pub show_reblogs: Option<bool>,
    pub uri: Option<String>,
    pub notify: Option<bool>,
    pub languages: Option<String>,
}

impl Follow {
    /// do follow action.  
    /// Record follow in database table.  
    /// Send follo action ok signal to sender.  
    pub async fn new(uri: String, sub: AccountUid, obj: AccountUid) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;

        // if follow is already exist, do update or instert

        let a = sqlx::query(
            "INSERT INTO follow(uid, account_uid, target_account_uid, uri) VALUES (?, ?, ?, ?)",
        )
        .bind(Uuid::now_v7().to_string())
        // .bind(
        //     SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs() as i64
        // )
        .bind(sub.to_string())
        .bind(obj.to_string())
        .bind(uri)
        .execute(&sqlx_conn)
        .await;

        match a {
            Ok(_) => {}
            Err(e) => {
                tracing::debug!("{:?}", e);
            }
        }

        Ok(())
    }

    pub async fn update(uri: String, sub: AccountUid, obj: AccountUid) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;

        // if follow is already exist, do update or instert

        let updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let a =
            sqlx::query("UPDATE follow SET updated_at = ?, uri = ? WHERE account_uid = ? AND target_account_uid = ?")
                .bind(updated_at)
                .bind(uri)
                .bind(sub.to_string())
                .bind(obj.to_string())
                .execute(&sqlx_conn)
                .await;

        match a {
            Ok(_) => {}
            Err(e) => {
                tracing::debug!("{:?}", e);
            }
        }

        Ok(())
    }

    /// Do NOT use this
    pub async fn all() -> Result<Vec<Follow>> {
        let sqlx_conn = dbcon::open_default()?;
        let follows: Vec<Follow> = sqlx::query_as("SELECT rowid, * FROM follow")
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(follows)
    }

    pub async fn follower_count(account_uid: String) -> Result<u64> {
        let sqlx_conn = dbcon::open_default()?;
        let follows: (i64,) =
            sqlx::query_as("SELECT count(*) FROM follow WHERE target_account_uid = ?")
                .bind(account_uid)
                .fetch_one(&sqlx_conn)
                .await?;
        Ok(follows.0 as u64)
    }

    pub async fn following_count(account_uid: String) -> Result<u64> {
        let sqlx_conn = dbcon::open_default()?;
        let followings: (i64,) =
            sqlx::query_as("SELECT COUNT(*) AS COUNT FROM follow WHERE account_uid = ?")
                .bind(account_uid)
                .fetch_one(&sqlx_conn)
                .await?;
        Ok(followings.0 as u64)
    }

    pub async fn followers(account_uuid: String) -> Result<Vec<Self>> {
        let sqlx_conn = dbcon::open_default()?;
        let followings: Vec<Follow> =
            sqlx::query_as("SELECT rowid, * FROM follow WHERE target_account_uid = ?")
                .bind(account_uuid)
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(followings)
    }

    pub async fn followings(account_uuid: String) -> Result<Vec<Self>> {
        let sqlx_conn = dbcon::open_default()?;
        let followings: Vec<Follow> =
            sqlx::query_as("SELECT rowid, * FROM follow WHERE account_id = ?")
                .bind(account_uuid)
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(followings)
    }

    pub async fn relations(a: String, b: String) -> Result<Vec<Self>> {
        let mut result: Vec<Follow> = Vec::new();
        let sqlx_conn = dbcon::open_default()?;
        let sql_template =
            "SELECT rowid, * FROM follow WHERE account_uid = ? AND target_account_uid = ?";
        let case_a: Vec<Follow> = sqlx::query_as(sql_template)
            .bind(a.clone())
            .bind(b.clone())
            .fetch_all(&sqlx_conn)
            .await?;
        let case_b: Vec<Follow> = sqlx::query_as(sql_template)
            .bind(b)
            .bind(a)
            .fetch_all(&sqlx_conn)
            .await?;
        result.push(case_a.last().unwrap().clone());
        result.push(case_b.last().unwrap().clone());

        Ok(result)
    }

    /// follow relatioship
    /// 0 -> No relationship
    /// 1 -> a follows b
    /// 2 -> b follows a
    /// 3 -> a,b follow each other
    pub async fn relation(a: String, b: String) -> Result<usize> {
        let sqlx_conn = dbcon::open_default()?;
        let sql_template =
            "SELECT rowid FROM follow WHERE account_uid = ? AND target_account_uid = ?";
        let case_a = sqlx::query(sql_template)
            .bind(a.clone())
            .bind(b.clone())
            .fetch_all(&sqlx_conn)
            .await?;
        let case_b = sqlx::query(sql_template)
            .bind(b)
            .bind(a)
            .fetch_all(&sqlx_conn)
            .await?;
        if case_a.len() == 0 && case_b.len() == 0 {
            return Ok(0);
        }
        if case_a.len() > 0 && case_b.len() == 0 {
            return Ok(1);
        }
        if case_a.len() == 0 && case_b.len() > 0 {
            return Ok(2);
        }
        if case_a.len() > 0 && case_b.len() > 0 {
            return Ok(3);
        }
        Ok(0)
    }

    pub async fn unfollow(uri: String) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;
        let a = sqlx::query("DELETE FROM follow WHERE uri = ?")
            .bind(uri)
            .execute(&sqlx_conn)
            .await;
        match a {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::msg("Unfollow error from table processing")),
        }
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Follow>>;
}

#[async_trait]
impl Get<(String, String)> for Follow {
    async fn get((key, val): (String, String)) -> Result<Vec<Follow>> {
        let query_template = format!("SELECT rowid, * FROM follow WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let follows = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(follows)
    }
}
