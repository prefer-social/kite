//! User struct.
//!
//! Mastodon doc: N/A

use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::variables;

use crate::mastodon::account::Account as MAccount;
use crate::table::user::Get as _;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
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
    pub role_id: Option<String>,
    pub settings: Option<String>,
    pub time_zone: Option<String>,
}

impl From<crate::table::user::User> for User {
    fn from(tbl: crate::table::user::User) -> Self {
        let user = User {
            uid: tbl.uid.into(),
            email: tbl.email,
            created_at: tbl.created_at,
            updated_at: tbl.updated_at,
            encrypted_password: tbl.encrypted_password,
            reset_password_token: tbl.reset_password_token,
            reset_password_sent_at: tbl.reset_password_sent_at,
            sign_in_count: tbl.sign_in_count,
            current_sign_in_at: tbl.current_sign_in_at,
            last_sign_in_at: tbl.last_sign_in_at,
            confirmation_token: tbl.confirmation_token,
            confirmed_at: tbl.confirmed_at,
            confirmation_sent_at: tbl.confirmation_sent_at,
            unconfirmed_email: tbl.unconfirmed_email,
            locale: tbl.locale,
            encrypted_otp_secret: tbl.encrypted_otp_secret,
            encrypted_otp_secret_iv: tbl.encrypted_otp_secret_iv,
            encrypted_otp_secret_salt: tbl.encrypted_otp_secret_salt,
            consumed_timestep: tbl.consumed_timestep,
            otp_required_for_login: tbl.otp_required_for_login,
            last_emailed_at: tbl.last_emailed_at,
            otp_backup_codes: tbl.otp_backup_codes, // is an Array
            account_id: tbl.account_id,             // account.uuid
            disabled: tbl.disabled,
            invite_id: tbl.invite_id,
            chosen_languages: tbl.chosen_languages,
            created_by_application_id: tbl.created_by_application_id,
            approved: tbl.approved, // default(TRUE), not null
            sign_in_token: tbl.sign_in_token,
            sign_in_token_sent_at: tbl.sign_in_token_sent_at,
            webauthn_id: tbl.webauthn_id,
            sign_up_ip: tbl.sign_up_ip,
            role_id: tbl.role_id,
            settings: tbl.settings,
            time_zone: tbl.time_zone,
            ..Default::default()
        };
        user
    }
}

impl Into<String> for User {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for User {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

impl User {
    pub async fn validate(username: String, password: String) -> Result<bool> {
        let domain = variables::get("domain")
            .expect("domain is not propery set in SPIN_VARIABLE");

        let encrypted_password =
            crate::table::user::User::get_encrypted_password(username, domain)
                .await?;

        if encrypted_password.is_empty() {
            return Ok(false);
        }

        let parsed_hash = PasswordHash::new(&encrypted_password[0].0).unwrap();

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(a: T) -> Result<Vec<crate::mastodon::user::User>>;
}

#[async_trait]
impl Get<MAccount> for User {
    async fn get(
        account: MAccount,
    ) -> Result<Vec<crate::mastodon::user::User>> {
        let account_id = account.uid.to_string();

        let user = crate::table::user::User::get((
            "account_id".to_string(),
            account_id,
        ))
        .await
        .unwrap_or_default();

        if user.is_empty() {
            let empty_vector = Vec::new();
            return Ok(empty_vector);
        }
        let user_row = user.last().unwrap().to_owned();
        let mastodon_user = vec![Self::try_from(user_row).unwrap()];

        Ok(mastodon_user)
    }
}
