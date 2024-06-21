use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use spin_sdk::sqlite::{QueryResult, Value as SV};

#[derive(Debug, sqlx::FromRow)]
pub struct Follow {
    pub rowid: Option<i64>,
    pub uuid: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub account_uuid: Option<String>,
    pub target_account_uuid: Option<i64>,
    pub show_rebloges: Option<bool>,
    pub uri: Option<String>,
    pub notify: Option<bool>,
    pub languages: Option<String>,
}

impl Follow {
    pub async fn all() -> Result<Vec<Follow>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let follows: Vec<Follow> =
            sqlx::query_as("SELECT rowid, * FROM follow")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(follows)
    }

    pub async fn get_number_of_followers(account_uuid: String) -> Result<i64> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let follows: (i64,) = sqlx::query_as(
            "SELECT count(*) FROM follow WHERE target_account_uuid = ?",
        )
        .bind(account_uuid)
        .fetch_one(&sqlx_conn)
        .await?;
        Ok(follows.0)
    }

    pub async fn get_number_of_followings(
        account_uuid: String,
    ) -> Result<i64> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let followings: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) AS COUNT FROM follow WHERE account_uuid = ?",
        )
        .bind(account_uuid)
        .fetch_one(&sqlx_conn)
        .await?;
        Ok(followings.0)
    }

    pub async fn get_followers(account_uuid: String) -> Result<Vec<Follow>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let followings: Vec<Follow> = sqlx::query_as("SELECT rowid, * AS COUNT FROM follow WHERE target_account_uuid = ?")
            .bind(account_uuid)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(followings)
    }

    pub async fn get_followings(account_uuid: String) -> Result<Vec<Follow>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let followings: Vec<Follow> = sqlx::query_as(
            "SELECT rowid, * AS COUNT FROM follow WHERE account_id = ?",
        )
        .bind(account_uuid)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(followings)
    }

    pub async fn new(f: Follow) -> Result<()> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;

        todo!();

        Ok(())
    }
}
