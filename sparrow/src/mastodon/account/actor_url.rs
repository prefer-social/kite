use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::default::Default;
use std::string::ToString;
use std::{fmt, str};
use url::Url;

use crate::activitystream::actor::person::Person;
use crate::table::actor_json::ActorJson;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ActorUrl(pub Option<Url>);

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
        let request = Request::builder()
            .method(Method::Get)
            .header("Accept", ct)
            .uri(actor_url)
            .build();

        let response: Response = spin_sdk::http::send(request).await.unwrap();

        let actor_str = str::from_utf8(response.body()).unwrap();

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
            r => {
                tracing::debug!(
                    "Actor {:?} response is {}",
                    self.0.as_ref().unwrap(),
                    r
                );
            }
        }

        let _ct = response.header("content-type").unwrap().as_str().unwrap();
        let actor_value: Value = serde_json::from_str(actor_str).unwrap();

        // This saves acor to actor_json table
        ActorJson::put(serde_json::from_str(actor_str).unwrap()).await?;

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
