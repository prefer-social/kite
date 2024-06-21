use crate::table::account::Account;
use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use serde_derive::{Deserialize, Serialize};
use spin_sdk::variables;

#[derive(
    Clone, Debug, Deserialize, Serialize, PartialEq, Default, sqlx::FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub rowid: Option<i64>,
    pub uuid: Option<String>,  // not null, primary key
    pub email: Option<String>, // default(""), not null
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
    pub account_id: Option<String>,       // account.uuid
    pub disabled: Option<bool>,
    pub invite_id: Option<i64>,
    pub chosen_languages: Option<String>, // is an Array
    pub created_by_application_id: Option<i64>,
    pub approved: Option<bool>, // default(TRUE), not null
    pub sign_in_token: Option<String>,
    pub sign_in_token_sent_at: Option<i64>,
    pub webauthn_id: Option<String>,
    pub sign_up_ip: Option<String>,
    pub role_id: Option<i64>,
    pub settings: Option<String>,
    pub time_zone: Option<String>,
}

impl User {
    pub async fn all() -> Result<Vec<Self>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let users: Vec<User> = sqlx::query_as("SELECT rowid, * FROM user")
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(users)
    }

    pub async fn validate(username: String, password: String) -> Result<bool> {
        let domain = variables::get("domain")
            .expect("domain is not propery set in SPIN_VARIABLE");

        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let encryted_password: (String, )  = sqlx::query_as(r#"SELECT user.encrypted_password FROM user 
        LEFT JOIN account ON user.account_id = account.uuid WHERE account.username = ? AND account.domain = ?"#)
        .bind(username)
        .bind(domain)
        .fetch_one(&sqlx_conn)
        .await?;

        let parsed_hash = PasswordHash::new(&encryted_password.0).unwrap();

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn default_user() -> Result<(Self, Account)> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let main_users: Vec<User> = sqlx::query_as(
            "SELECT rowid, * FROM user WHERE user.admin == true",
        )
        .fetch_all(&sqlx_conn)
        .await?;
        let main_user = main_users.get(0).unwrap().clone();

        let account_id = main_user.to_owned().account_id.unwrap();
        let account = Account::get_with_account_id(account_id).await?.unwrap();

        Ok((main_user, account))
    }

    pub async fn get(uuid: &str) -> Result<Vec<Self>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let users: Vec<User> =
            sqlx::query_as("SELECT rowid, * FROM user WHERE uresr.uuid = ?")
                .bind(uuid)
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(users)
    }

    pub async fn user_count() -> Result<i64> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) AS C FROM user")
            .fetch_one(&sqlx_conn)
            .await?;
        Ok(count.0)
    }
}
