//! oauth_access_grant table  
//!

use anyhow::Result;
use async_trait::async_trait;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OauthAccessGrant {
    pub rowid: Option<i64>,
    pub uuid: String, // not null, primary key
    pub token: String,
    pub expires_in: i64,
    pub redirect_uri: String,
    pub created_at: i64,
    pub revoked_at: Option<i64>,
    pub scopes: Option<i64>,
    pub application_id: i64,
    pub resource_owner_id: i64,
}

impl OauthAccessGrant {
    pub async fn all() -> Result<Vec<OauthAccessGrant>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let oag: Vec<OauthAccessGrant> =
            sqlx::query_as("SELECT rowid, * FROM oauth_access_grant")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oag)
    }

    pub async fn get(_w_claus: String) -> Result<Vec<OauthAccessGrant>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let oag: Vec<OauthAccessGrant> =
            sqlx::query_as("SELECT rowid, * FROM oauth_access_grant WHERE ")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oag)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<OauthAccessGrant>>;
}

#[async_trait]
impl Get<(String, String)> for OauthAccessGrant {
    async fn get(
        (key, val): (String, String),
    ) -> Result<Vec<OauthAccessGrant>> {
        let query_template = format!(
            "SELECT rowid, * FROM oauth_access_token WHERE {} = ?",
            key
        );
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
