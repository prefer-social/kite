//! Account::Uri object. The object contains username and domain.    
//! If domain is None, the account is local user's account.
//!

use anyhow::Result;
use bincode::{Decode, Encode};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::{fmt, str};
use url::Url;

use crate::activitystream::actor::person::Person as PersonActor;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::uid::Uid as AccountUid;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::setting::Setting;
use crate::table::account::Account as TAccount;
use crate::table::account::Get as _;
use crate::table::actor_json::ActorJson;
use crate::webfinger::WebFinger;

/// presents Account's Uri information - username and domain.  
#[derive(
    Debug, Deserialize, Default, Clone, Eq, PartialEq, Encode, Decode,
)]
pub struct Uri {
    pub username: String,
    pub domain: Option<String>, // If domain is None, it is a local user.
}

impl Uri {
    /// returns Account::Uid object.
    pub async fn account_uid(&self) -> Option<AccountUid> {
        match TAccount::get(self.clone()).await {
            Ok(a) => {
                let taccount = a.last().unwrap().to_owned();
                let account_uid = AccountUid(taccount.uid);
                return Some(account_uid);
            }
            Err(_e) => return None,
        };

        //match MAccount::get(self.clone()).await {
        //    Ok(a) => return Some(a.uid),
        //    Err(_e) => return None,
        //};
    }

    /// returns actor's Url from webfinger request.
    pub async fn actor_url(&self) -> Result<ActorUrl> {
        match self.local() {
            true => {
                // Get a actor url
                // Because it is a single user system, simply return this static url
                let domain = Setting::get("site_domain").await.unwrap();
                let my_actor_url = format!("https://{}/self", domain);
                ActorUrl::new(my_actor_url)
            }
            _ => {
                let acct = format!(
                    "{}@{}",
                    self.username,
                    self.domain.as_ref().unwrap()
                );
                let wf = WebFinger::query(acct.as_str()).await?;
                if wf.is_none() {
                    return Err(anyhow::Error::msg(
                        "WebFinger query returns None.",
                    ));
                }
                let links = wf.unwrap().links;
                let mut actor_url = "".to_string();

                for link in links.iter() {
                    if link.rel == "self" {
                        actor_url = link.href.clone().unwrap();
                    }
                }

                ActorUrl::new(actor_url)
            }
        }
    }

    pub fn new(username: String, mut domain: Option<String>) -> Self {
        domain = match domain {
            None => None,
            Some(x) => Some(x.to_lowercase()),
        };

        Uri {
            username: username.to_lowercase(),
            domain,
        }
    }

    /// Is local user?  
    pub fn local(&self) -> bool {
        self.domain == None
    }
}

/// Convert Email format string to Account Uri struct.  
impl TryFrom<String> for Uri {
    type Error = &'static str;
    fn try_from(a: String) -> Result<Self, Self::Error> {
        let mut b = a.split("@");
        if b.clone().count() != 2 {
            return Err("Parsing error from given Uri. Uri form error.");
        }
        return Ok(Uri {
            username: b.next().unwrap().to_string(),
            domain: b.next().map(|x| x.to_string()),
        });
    }
}

/// Convert Account Uri struct to String Result.  
impl TryInto<String> for Uri {
    type Error = &'static str;
    fn try_into(self) -> Result<String, Self::Error> {
        if self.local() {
            return Ok(self.username);
        }
        Ok(format!("{}@{}", self.username, self.domain.unwrap()))
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.local() {
            true => write!(f, "{}", self.username),
            _ => write!(
                f,
                "{}@{}",
                self.username,
                self.domain.as_ref().unwrap()
            ),
        }
    }
}

/// Serialize Account Uri to email format string.  
impl Serialize for Uri {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.local() {
            true => {
                let acct_url = format!("{}", self.username);
                s.serialize_str(acct_url.as_str())
            }
            _ => {
                let acct_url = format!(
                    "{}@{}",
                    self.username,
                    self.domain.as_ref().unwrap()
                );
                s.serialize_str(acct_url.as_str())
            }
        }
    }
}

// impl<'de> Deserialize<'de> for Uri {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct UriVisitor;

//         impl<'de> Visitor<'de> for UriVisitor {
//             type Value = Field;

//             fn expecting(
//                 &self,
//                 formatter: &mut fmt::Formatter,
//             ) -> fmt::Result {
//                 formatter.write_str("`secs` or `nanos`")
//             }

//             fn visit_str<E>(self, value: &str) -> Result<Field, E>
//             where
//                 E: de::Error,
//             {
//                 let a = value.split("@");
//                 match value {
//                     "username" => Ok(Field::Username),
//                     "domain" => Ok(Field::Domain),
//                     _ => Err(de::Error::unknown_field(
//                         value,
//                         &["username", "domain"],
//                     )),
//                 }
//             }
//         }
//         // deserializer.deserialize_struct(
//         //     "Uri",
//         //     &["username", "domain"],
//         //     UriVisitor,
//         // )

//         deserializer.deserialize_string(visitor)

//     }
// }
