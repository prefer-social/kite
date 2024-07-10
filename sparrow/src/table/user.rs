use anyhow::Result;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use spin_sqlx::Connection as dbcon;

#[derive(
    Clone, Debug, Deserialize, Serialize, PartialEq, Default, sqlx::FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub rowid: Option<i64>,
    pub uid: Option<String>,     // not null, primary key
    pub email: Option<String>,   // default(""), not null
    pub created_at: Option<i64>, // not null
    pub updated_at: Option<i64>, // not null
    pub encrypted_password: Option<String>, // default(""), not null
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<i64>,
    pub sign_in_count: Option<i64>, // default(0), not null
    pub current_sign_in_at: Option<i64>,
    pub last_sign_in_at: Option<i64>,
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<i64>,
    pub confirmation_sent_at: Option<i64>,
    pub unconfirmed_email: Option<String>,
    pub locale: Option<String>,
    pub encrypted_otp_secret: Option<String>,
    pub encrypted_otp_secret_iv: Option<String>,
    pub encrypted_otp_secret_salt: Option<String>,
    pub consumed_timestep: Option<i64>,
    pub otp_required_for_login: Option<bool>, // default(FALSE), not null
    pub last_emailed_at: Option<i64>,
    pub otp_backup_codes: Option<String>, // is an Array
    pub account_id: Option<String>,       // account.uid
    pub disabled: Option<bool>,
    pub invite_id: Option<i64>,
    pub chosen_languages: Option<String>, // is an Array
    pub created_by_application_id: Option<i64>,
    pub approved: Option<bool>, // default(TRUE), not null
    pub sign_in_token: Option<String>,
    pub sign_in_token_sent_at: Option<i64>,
    pub webauthn_id: Option<String>,
    pub sign_up_ip: Option<String>,
    pub role_id: Option<String>,
    pub settings: Option<String>,
    pub time_zone: Option<String>,
}

impl User {
    pub async fn all() -> Result<Vec<Self>> {
        let sqlx_conn = dbcon::open_default()?;
        let users: Vec<User> = sqlx::query_as("SELECT rowid, * FROM user")
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(users)
    }

    pub async fn get_encrypted_password(
        username: String,
        domain: String,
    ) -> Result<Vec<(String,)>> {
        let sqlx_conn = dbcon::open_default()?;
        let encryted_password: Vec<(String, )>  = sqlx::query_as(r#"SELECT user.encrypted_password FROM user 
        LEFT JOIN account ON user.account_id = account.uid WHERE account.username = ? AND account.domain = ?"#)
        .bind(username)
        .bind(domain)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(encryted_password)
    }

    pub async fn default_user() -> Result<Vec<Self>> {
        let sqlx_conn = dbcon::open_default()?;
        let main_users: Vec<User> = sqlx::query_as(
            "SELECT rowid, * FROM user WHERE user.admin == true",
        )
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(main_users)
    }

    pub async fn get_with_uid(uid: String) -> Result<Option<Self>> {
        let users = Self::get(("uid".to_owned(), uid)).await?;
        Ok(Some(users.first().unwrap().to_owned()))
    }

    pub async fn user_count() -> Result<i64> {
        let sqlx_conn = dbcon::open_default()?;
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) AS C FROM user")
            .fetch_one(&sqlx_conn)
            .await?;
        Ok(count.0)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<User>>;
}

#[async_trait]
impl Get<(String, String)> for User {
    async fn get((key, val): (String, String)) -> Result<Vec<User>> {
        let aaa = key.to_owned();
        let query_template =
            format!("SELECT rowid, * FROM account WHERE {} = ?", aaa);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
