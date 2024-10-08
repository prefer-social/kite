//! oauth_application table

use anyhow::Result;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use spin_sqlx::sqlite::Connection as dbcon;

#[derive(
    Default, Clone, Debug, PartialEq, sqlx::FromRow, Serialize, Deserialize,
)]
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
    pub owner_id: Option<String>,
    pub confidential: Option<bool>,
}

impl OauthApplication {
    pub async fn all() -> Result<Vec<OauthApplication>> {
        let sqlx_conn = dbcon::open_default()?;
        let oat: Vec<OauthApplication> =
            sqlx::query_as("SELECT rowid, * FROM oauth_application")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(oat)
    }

    pub async fn add(
        app: crate::mastodon::application::Application,
        user_id: Option<String>,
    ) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;
        sqlx::query(
            "INSERT INTO oauth_application ( 
            uid,
            name,
            secret,
            redirect_uri,
            website,
            owner_id
            ) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(app.uid.clone())
        .bind(app.name)
        .bind(app.client_secret.unwrap())
        .bind(app.redirect_uri.unwrap())
        .bind(app.website.unwrap())
        .bind(user_id.unwrap())
        .execute(&sqlx_conn)
        .await?;

        tracing::debug!("-----> ADDED, application ---> {}", app.uid);

        Ok(())
    }

    pub async fn get_by_app_id(app_id: String) -> Result<OauthApplication> {
        let sqlx_conn = dbcon::open_default()?;
        let oa: OauthApplication = sqlx::query_as(
            "SELECT rowid, * FROM oauth_application WHERE uid = ?",
        )
        .bind(app_id)
        .fetch_one(&sqlx_conn)
        .await?;
        Ok(oa)
    }

    pub async fn remove(uid: String) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;
        let _res = sqlx::query("DELETE FROM oauth_application WHERE uid = ?")
            .bind(uid)
            .execute(&sqlx_conn)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<OauthApplication>>;
}

#[async_trait]
impl Get<(String, String)> for OauthApplication {
    async fn get(
        (key, val): (String, String),
    ) -> Result<Vec<OauthApplication>> {
        let query_template = format!(
            "SELECT rowid, * FROM oauth_application WHERE {} = ?",
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
