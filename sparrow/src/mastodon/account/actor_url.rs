use anyhow::{Error, Result};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::default::Default;
use std::ops::Deref;
use std::string::ToString;
use std::{fmt, str};
use url::Url;

use crate::activitystream::actor::person::Person;
use crate::mastodon;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::ME_ACCOUNT;
use crate::table::account::Account as TAccount;
use crate::table::actor_json::ActorJson;

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Encode, Decode,
)]
pub struct ActorUrl(#[bincode(with_serde)] pub Option<Url>);

impl ActorUrl {
    pub fn new(u: String) -> Result<Self> {
        match u.parse::<Url>() {
            Ok(url) => Ok(ActorUrl(Some(url))),
            Err(e) => {
                Err(Error::msg(format!("ActorUrl parse error: {:?}", e)))
            }
        }
    }

    pub async fn actor(&self) -> Result<Person> {
        let ct = "application/activity+json";
        let actor_url = self.0.as_ref().unwrap().to_owned();

        let me_account = ME_ACCOUNT.get().unwrap().to_owned();
        let taccount = TAccount::default(); // Todo: Remove this sql call.
        let response = mastodon::get_fediverse(actor_url, me_account).await?;

        let body = response.body();
        let actor = str::from_utf8(body).unwrap();

        match response.status().to_owned() {
            200 => {}
            410 => {
                // Http response 410 is Gone
                return Err(anyhow::Error::msg(format!(
                    "Actor {:?} is gone",
                    self.0.as_ref().unwrap().to_string()
                )));
                // TODO: need to delete from db table?
            } // Gone.
            404 => {
                // Actor not found
                return Err(anyhow::Error::msg(format!(
                    "Actor {:?} NOT FOUND",
                    self.0.as_ref().unwrap().to_string()
                )));
            }
            r => {
                tracing::debug!(
                    "Actor {:?} response is {}",
                    self.0.as_ref().unwrap(),
                    r
                );
            }
        }

        let headers = response.headers();
        let actor_value: Value = serde_json::from_str(actor).unwrap();

        // This saves acor to actor_json table
        ActorJson::put(serde_json::from_str(actor).unwrap()).await?;

        // Convert this to ActivityPub Actor
        let actor = Person::try_from(actor_value).unwrap();

        Ok(actor)
    }

    pub async fn remove(&self) {
        todo!()
    }
}

impl Default for ActorUrl {
    fn default() -> Self {
        ActorUrl(None)
    }
}

impl fmt::Display for ActorUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_ref().unwrap())
    }
}
