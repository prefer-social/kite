use anyhow::Result;
use oauth2::AccessToken;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

use super::account::Account;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OauthAccessToken {
    pub rowid: Option<i64>,
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
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let oat: Vec<OauthAccessToken> =
            sqlx::query_as("SELECT rowid, * FROM oauth_access_token")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oat)
    }

    // INSERT INTO user_authorization_code(userId, code, token_issued) VALUES((SELECT id FROM user WHERE user.name == ?), ?, ?)
    //let token = sparrow::mastodon::token::Token::new(
    //code, client_id, client_secret, redirect_uri, scope, grant_type
    //);

    pub async fn new(
        scope: String,
        application_id: String,
        resource_owner_id: String,
        last_used_ip: String,
    ) -> Result<Self> {
        //
        // TODO: https://www.rfc-editor.org/rfc/rfc6750
        //

        let token_id = uuid::Uuid::now_v7().to_string();
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
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
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let accts: Vec<crate::table::account::Account> = sqlx::query_as(
            r#"SELECT D.rowid, D.* FROM oauth_access_token 
            AS A FULL OUTER JOIN oauth_application AS B ON A.application_id = B.uid 
            FULL OUTER JOIN user AS C ON B.owner_id = c.uid 
            FULL OUTER JOIN account as D ON D.uid = c.account_id 
            WHERE A.token = ?"#,
        )
        .bind(token)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(accts)
    }
}
