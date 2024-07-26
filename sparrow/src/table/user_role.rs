// https://docs.joinmastodon.org/entities/Role/

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sqlx::sqlite::Connection as dbcon;

#[derive(
    Clone, Debug, Deserialize, Serialize, PartialEq, Default, sqlx::FromRow,
)]
pub struct UserRole {
    pub uid: String,
    pub name: String,
    pub color: String,
    pub position: i64,
    pub permissions: i64,
    pub highlighted: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Option<UserRole>>;
}

#[async_trait]
impl Get<(String, String)> for UserRole {
    async fn get((key, val): (String, String)) -> Result<Option<UserRole>> {
        let query_template =
            format!("SELECT * FROM user_role WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<UserRole> = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts.last().map(|x| x.to_owned()))
    }
}
