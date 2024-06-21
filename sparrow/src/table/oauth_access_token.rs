use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OauthAccessToken {
    pub rowid: Option<i64>,
    pub uuid: String, // not null, primary key
    pub token: Option<String>,
    pub refresh_token: Option<i64>,
    pub revoked_at: Option<i64>,
    pub created_at: i64,
    pub scopes: Option<i64>,
    pub application_id: Option<i64>,
    pub resource_owner_id: Option<i64>,
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
    pub async fn set(uuid: String, code: String, token: String) -> Result<()> {
        // let sqlx_conn = spin_sqlx::Connection::open_default()?;
        //  let a = sqlx::query_as("INSERT INTO oauth_access_token (uuid, code, token) VALUES ()")

        Ok(())
    }
}

/*
CREATE TABLE public.oauth_access_tokens (
    id bigint NOT NULL,
    token character varying NOT NULL,
    refresh_token character varying,
    expires_in integer,
    revoked_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    scopes character varying,
    application_id bigint,
    resource_owner_id bigint,
    last_used_at timestamp without time zone,
    last_used_ip inet
);
*/
