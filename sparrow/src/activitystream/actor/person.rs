//! ActivityStream Actor Person.  
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person>
//! Mastodon doc: <https://www.w3.org/TR/activitypub/#actor-objects>  

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::{fmt, str};

use crate::activitystream::actor::ActorType;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::setting::Setting;
use crate::mastodon::user::Get;
use crate::mastodon::user::User;
use crate::table::account::Account as TAccount;
use crate::table::account::New;

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Actor
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "@context")]
    pub context: Option<Value>,
    /// Actor's id is actorl url.  
    pub id: String,
    #[serde(rename = "type")]
    pub actor_type: ActorType,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    pub featured: String,
    pub featured_tags: String,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memorial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<String>,
    pub public_key: PublicKey,
    pub tag: Vec<Value>,
    pub attachment: Vec<Value>,
    pub endpoints: Endpoints,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    pub kind: String,
    pub media_type: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    pub shared_inbox: String,
}

impl Person {
    /// Get PersonActor struct from MAccount
    pub async fn new(a: MAccount) -> Result<Self> {
        // Git User struct from Account (User.account_id = Account.uid)
        //let u = User::get(a.to_owned()).await?;
        // if u is None, means it is not local user.

        let domain = match a.local() {
            true => Setting::domain().await,
            _ => a.account_uri.domain.unwrap(),
        };

        let pk = PublicKey {
            id: format!("{}#main-key", a.url.clone()),
            owner: a.url.to_owned(),
            public_key_pem: a.public_key,
        };

        let icon = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: a.avatar,
        };

        let image = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: a.header,
        };

        let username = a.username;

        let ct = vec![
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string(),
        ];
        let ct_val = serde_json::to_value(&ct).unwrap();

        let endpoints = Endpoints {
            shared_inbox: format!("https://{}/inbox", domain),
        };

        let pa = Person {
            context: Some(ct_val),
            id: a.actor_url.to_string(),
            actor_type: ActorType::Person,
            following: a.following_url.to_owned().unwrap_or_default(),
            followers: a.followers_url.to_owned().unwrap_or_default(),
            inbox: a.inbox_url.to_owned().unwrap_or_default(),
            outbox: a.outbox_url.to_owned().unwrap_or_default(),
            featured: format!("https://{}/collections/featured", domain), // Todo:
            featured_tags: format!("https://{}/collections/tags", domain), // Todo:
            preferred_username: username.to_string().to_owned(),
            name: a.display_name.to_owned(),
            summary: a.note.to_owned(),
            url: a.url.to_owned(),
            manually_approves_followers: false, // Todo:
            discoverable: a.discoverable.to_owned(),
            indexable: a.indexable.to_owned().unwrap_or_default(),
            published: crate::utils::convert_epoch_to_iso_8601(
                a.created_at.timestamp(),
            ),
            memorial: Some(false),
            devices: None,
            public_key: pk,
            tag: Vec::new(),
            attachment: Vec::new(),
            endpoints: endpoints,
            icon: Some(icon),
            image: Some(image),
            ..Default::default()
        };
        Ok(pa)
    }

    /// Stores this actor to Account table.  
    pub async fn store(&self) -> Result<()> {
        Ok(TAccount::new(self.to_owned()).await?)
    }
}

impl TryFrom<serde_json::Value> for Person {
    type Error = ();
    fn try_from(actor_value: serde_json::Value) -> Result<Self, Self::Error> {
        let person = serde_json::from_value::<Person>(actor_value).unwrap();
        Ok(person)
    }
}
