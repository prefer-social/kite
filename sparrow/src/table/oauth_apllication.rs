use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use spin_sdk::key_value::Store;
use uuid::Uuid;

use crate::mastodon::application::Application;

#[derive(Default, Clone, Debug, PartialEq, sqlx::FromRow)]
pub struct OauthApplication {
    pub rowid: i64,
    pub uid: String,
    pub name: String,
    pub secret: String,
    pub redirect_uri: String,
    pub scopes: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub superapp: bool,
    pub website: String,
    pub owner_type: Option<String>,
    pub owner_id: Option<i64>,
    pub confidential: Option<bool>,
}

impl OauthApplication {
    pub async fn all() -> Result<Vec<OauthApplication>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let oat: Vec<OauthApplication> =
            sqlx::query_as("SELECT rowid, * FROM oauth_application")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oat)
    }

    pub async fn create(
        a: crate::mastodon::application::Application,
    ) -> Result<OauthApplication> {
        let store = Store::open("mem").unwrap();

        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        sqlx::query(
            "INSERT INTO oauth_application ( 
            uid,
            name,
            secret,
            redirect_uri,
            website) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(a.client_id.clone())
        .bind(a.name)
        .bind(a.client_secret)
        .bind(a.redirect_uri)
        .bind(a.website)
        .execute(&sqlx_conn)
        .await?;

        let oauth_application: OauthApplication = sqlx::query_as(
            "SELECT rowid, * FROM oauth_application WHERE uid = ?",
        )
        .bind(a.client_id)
        .fetch_one(&sqlx_conn)
        .await?;

        Ok(oauth_application)
    }
}
