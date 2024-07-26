//! Represents an OAuth token used for authenticating with the API and performing actions.  
//! Mastodon doc: <https://docs.joinmastodon.org/methods/oauth/#token>
//!               <https://docs.joinmastodon.org/entities/Token/>

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::table::account::Account as TAccount;
use crate::table::oauth_access_token::Get as _;
use crate::table::oauth_access_token::OauthAccessToken;

/// Represents an OAuth token used for authenticating with the API and performing actions.
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Token {
    /// An OAuth token to be used for authorization.  
    access_token: String,
    /// The OAuth token type. Mastodon uses Bearer tokens.  
    token_type: String,
    /// The OAuth scopes granted by this token, space-separated.   
    scope: String,
    /// When the token was generated. Unix time.   
    created_at: i64,
    /// application id from . Internal use.  
    #[serde(skip_serializing)]
    application_id: String,
    /// Uid from Account table. Internal use.  
    #[serde(skip_serializing)]
    account_id: Option<String>,
    #[serde(skip_serializing)]
    username: Option<String>,
}

impl Token {
    /// Create new token struct.  
    pub async fn new(
        scope: String,
        application_id: String,
        resource_owner_id: String,
        last_used_ip: String,
    ) -> Result<Self> {
        let oat = OauthAccessToken::new(
            scope,
            application_id.clone(),
            resource_owner_id,
            last_used_ip,
        )
        .await?;

        Ok(Token {
            access_token: oat.token.unwrap(),
            token_type: "bearer".to_string(),
            scope: oat.scopes.unwrap(),
            created_at: oat.created_at,
            application_id: application_id,
            account_id: None,
            username: None,
        })
    }

    /// Validate given token.  
    /// This validate method returns CredentialAccount - https://docs.joinmastodon.org/entities/Account/#CredentialAccount
    pub async fn validate(
        token_type: String,
        token: String,
    ) -> Result<Option<MAccount>> {
        if token_type == "Bearer" {
            let accounts: Vec<TAccount> =
                OauthAccessToken::validate(token).await?;

            if accounts.is_empty() {
                return Ok(None);
            };
            let tacct: TAccount = accounts.first().unwrap().to_owned();
            let macct = MAccount::get(tacct).await?;

            return Ok(Some(macct));
        }

        Ok(None)
    }

    /// Returns token owners Account struct.  
    pub async fn owner(
        _token_type: String,
        token: String,
    ) -> Result<MAccount> {
        //if token_type == "Bearer" {
        // let token = OauthAccessToken::get(("token".to_string(), token))
        //     .await?
        //     .last()
        //     .unwrap()
        //     .to_owned();
        // let owner_id = token.resource_owner_id.unwrap();

        let account_tbl: TAccount =
            TAccount::fr_token(token).await?.last().unwrap().to_owned();

        tracing::debug!("######################################");

        let maccount = MAccount::get(account_tbl).await?;

        tracing::debug!("################### 2");
        return Ok(maccount);
        //}
    }
}
