//! follow table

use anyhow::Result;
use async_trait::async_trait;
use spin_sqlx::Connection as dbcon;

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
        let sqlx_conn = dbcon::open_default()?;
        let follows: Vec<Follow> =
            sqlx::query_as("SELECT rowid, * FROM follow")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(follows)
    }

    pub async fn get_number_of_followers(account_uuid: String) -> Result<i64> {
        let sqlx_conn = dbcon::open_default()?;
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
        let sqlx_conn = dbcon::open_default()?;
        let followings: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) AS COUNT FROM follow WHERE account_uuid = ?",
        )
        .bind(account_uuid)
        .fetch_one(&sqlx_conn)
        .await?;
        Ok(followings.0)
    }

    pub async fn get_followers(account_uuid: String) -> Result<Vec<Follow>> {
        let sqlx_conn = dbcon::open_default()?;
        let followings: Vec<Follow> = sqlx::query_as("SELECT rowid, * AS COUNT FROM follow WHERE target_account_uuid = ?")
            .bind(account_uuid)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(followings)
    }

    pub async fn get_followings(account_uuid: String) -> Result<Vec<Follow>> {
        let sqlx_conn = dbcon::open_default()?;
        let followings: Vec<Follow> = sqlx::query_as(
            "SELECT rowid, * AS COUNT FROM follow WHERE account_id = ?",
        )
        .bind(account_uuid)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(followings)
    }

    pub async fn new(_f: Follow) -> Result<()> {
        let _sqlx_conn = dbcon::open_default()?;
        todo!();
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Follow>>;
}

#[async_trait]
impl Get<(String, String)> for Follow {
    async fn get((key, val): (String, String)) -> Result<Vec<Follow>> {
        let query_template =
            format!("SELECT rowid, * FROM follow WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
