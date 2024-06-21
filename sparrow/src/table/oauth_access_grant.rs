/*
CREATE TABLE public.oauth_access_grants (
    id bigint NOT NULL,
    token character varying NOT NULL,
    expires_in integer NOT NULL,
    redirect_uri text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    revoked_at timestamp without time zone,
    scopes character varying,
    application_id bigint NOT NULL,
    resource_owner_id bigint NOT NULL
);
*/

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

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

    pub async fn get(w_claus: String) -> Result<Vec<OauthAccessGrant>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let oag: Vec<OauthAccessGrant> =
            sqlx::query_as("SELECT rowid, * FROM oauth_access_grant WHERE ")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oag)
    }
}
