use anyhow::Result;
use async_trait::async_trait;
use spin_sqlx::sqlite::Connection as dbcon;

use super::account::Account;

// oauth_acess_token table.
#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OauthAccessToken {
    pub rowid: i64,
    pub uid: String, // not null, primary key
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub revoked_at: Option<i64>,
    pub created_at: i64,
    pub scopes: Option<String>,
    pub application_id: Option<String>,
    pub resource_owner_id: Option<String>,
    pub last_used_at: Option<i64>,
    pub last_used_ip: Option<String>,
}

impl OauthAccessToken {
    pub async fn all() -> Result<Vec<OauthAccessToken>> {
        let sqlx_conn = dbcon::open_default()?;
        let oat: Vec<OauthAccessToken> =
            sqlx::query_as("SELECT rowid, * FROM oauth_access_token")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oat)
    }

    pub async fn new(
        scope: String,
        application_id: String,
        resource_owner_id: String,
        last_used_ip: String,
    ) -> Result<Self> {
        //! TODO: Implement <https://www.rfc-editor.org/rfc/rfc6750>
        let token_id = uuid::Uuid::now_v7().to_string();
        let sqlx_conn = dbcon::open_default()?;
        sqlx::query(r#"INSERT INTO oauth_access_token 
        (uid, token, refresh_token, scopes, application_id, resource_owner_id, last_used_ip) VALUES 
        (?, ?, ?, ?, ?, ?, ?)"#)
        .bind(token_id.clone())
        .bind(crate::utils::random_string(64).await)
        .bind(crate::utils::random_string(64).await)
        .bind(scope)
        .bind(application_id)
        .bind(resource_owner_id)
        .bind(last_used_ip)
        .execute(&sqlx_conn).await?;
        let token: OauthAccessToken = sqlx::query_as::<_, OauthAccessToken>(
            r#"SELECT rowid, * FROM oauth_access_token WHERE uid = ?"#,
        )
        .bind(token_id)
        .fetch_one(&sqlx_conn)
        .await?;
        Ok(token)
    }

    pub async fn validate(token: String) -> Result<Vec<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accts: Vec<crate::table::account::Account> = sqlx::query_as(
            r#"SELECT account.rowid, account.* FROM oauth_access_token 
            INNER JOIN oauth_application ON oauth_access_token.application_id = oauth_application.uid 
            INNER JOIN account ON account.uid = oauth_application.owner_id 
            WHERE token = ?"#)
        .bind(token)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(accts)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<OauthAccessToken>>;
}

#[async_trait]
impl Get<(String, String)> for OauthAccessToken {
    async fn get(
        (key, val): (String, String),
    ) -> Result<Vec<OauthAccessToken>> {
        let query_template = format!(
            "SELECT rowid, * FROM oauth_access_token WHERE {} = ?",
            key
        );
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
