// https://docs.joinmastodon.org/methods/oauth/#token
// https://docs.joinmastodon.org/entities/Token/
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::mastodon::account::Account as MAccount;
use crate::table::account::Account as TAccount;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Token {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: i64,
    #[serde(skip_serializing)]
    application_id: String,
    #[serde(skip_serializing)]
    account_id: Option<String>,
    #[serde(skip_serializing)]
    username: Option<String>,
}

impl Token {
    pub async fn new(
        scope: String,
        application_id: String,
        resource_owner_id: String,
        last_used_ip: String,
    ) -> Result<Self> {
        let oat = crate::table::oauth_access_token::OauthAccessToken::new(
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

    pub async fn validate(
        token_type: String,
        token: String,
    ) -> Result<Option<MAccount>> {
        if token_type == "Bearer" {
            let accounts: Vec<TAccount> =
                 crate::table::oauth_access_token::OauthAccessToken::validate(
                     token,
                 )
                 .await?;

            if accounts.is_empty() {
                return Ok(None);
            };
            let tacct: TAccount = accounts.first().unwrap().to_owned();
            let macct = crate::mastodon::account::Account::from(tacct);

            return Ok(Some(macct));
        }

        Ok(None)
    }
}
