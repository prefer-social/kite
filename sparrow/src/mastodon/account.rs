// https://docs.joinmastodon.org/entities/Account/

pub mod source;
pub mod field;
pub mod credential_account;
pub mod muted_account;

use anyhow::Result;
use async_trait::async_trait;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::str;
use regex::Regex;

use crate::mastodon::custom_emoji::CustomEmoji;
use crate::mastodon::uid::Uid;
use crate::mastodon::user::User;
use crate::mastodon::user_role::UserRole as Role;
use crate::mastodon::username::Username;
use crate::table::account::Account as TAccount;
use crate::table::account::Get as _;
use crate::mastodon::account::field::Field;
use crate::mastodon::account::source::Source;

/// Can I write any?
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Account {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: Uid,
    pub username: Username,
    pub acct: String,
    pub url: String,
    pub display_name: String,
    pub note: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub locked: bool,
    pub fields: Vec<Field>,
    pub emojis: Vec<CustomEmoji>,
    pub bot: bool,
    pub group: bool,
    pub discoverable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noindex: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub last_status_at: DateTime<Utc>,
    pub statuses_count: u32,
    pub followers_count: u32,
    pub following_count: u32,
    #[serde(skip_serializing, skip_deserializing)]
    pub private_key: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub public_key: Option<String>,
    // MutedAccount entity attributes
}

impl Account {
    pub async fn default_user() -> Result<(Account, User)> {
        let du = crate::table::user::User::default_user()
            .await?
            .first()
            .unwrap()
            .to_owned();
        let user = crate::mastodon::user::User::from(du);
        let account_id =
            crate::mastodon::uid::Uid(user.account_id.clone().unwrap());
        let account = Self::get(account_id).await?;
        Ok((account, user))
    }
}

impl TryFrom<TAccount> for Account {
    type Error = ();
    fn try_from(acct_tbl: TAccount) -> Result<Self, Self::Error> {
        let bot: bool = match acct_tbl.actor_type.unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let account = Account {
            uid: acct_tbl.uid.into(),
            username: acct_tbl.username.clone().into(),
            acct: format!(
                "{}@{}",
                acct_tbl.username.to_string(),
                acct_tbl.domain.unwrap()
            ),
            display_name: acct_tbl.display_name.unwrap(),
            locked: acct_tbl.locked.unwrap_or_default(),
            bot: bot,
            discoverable: acct_tbl.discoverable.unwrap_or_default(),
            created_at: DateTime::from_timestamp(
                acct_tbl.created_at.unwrap(),
                0,
            )
            .unwrap(),
            note: acct_tbl.note.unwrap(),
            url: acct_tbl.url.unwrap_or_default(),
            avatar: acct_tbl.avatar_remote_url.clone().unwrap_or_default(),
            avatar_static: acct_tbl
                .avatar_remote_url
                .clone()
                .unwrap_or_default(),
            header: acct_tbl.header_remote_url.clone().unwrap_or_default(),
            header_static: acct_tbl.header_remote_url.unwrap_or_default(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            ..Default::default()
        };
        Ok(account)
    }
}

impl TryFrom<crate::activitypub::person_actor::PersonActor> for Account {
    type Error = anyhow::Error;
    fn try_from(
        actor: crate::activitypub::person_actor::PersonActor,
    ) -> Result<Account, Self::Error> {
        todo!()
    }
}

impl TryInto<String> for Account {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        Ok(serde_json::to_string(&self).unwrap())
    }
}

impl TryInto<Value> for Account {
    type Error = ();
    fn try_into(self) -> Result<Value, Self::Error> {
        Ok(serde_json::to_value(&self).unwrap())
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(a: T) -> Result<Account>;
}

#[async_trait]
impl Get<Uid> for Account {
    async fn get(uid: Uid) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "uid".to_string(),
            uid.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Ok(self::Account::try_from(acct_tbl).unwrap())
    }
}

#[async_trait]
impl Get<Username> for Account {
    async fn get(username: Username) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "username".to_string(),
            username.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Ok(self::Account::try_from(acct_tbl).unwrap())
    }
}

impl Account {
    // https://docs.joinmastodon.org/entities/Search/#accounts
    pub async fn search(mut st: &String) -> Result<Vec<Account>> {
        let mut search_term: String = st.to_string();
        // Local account: Don't search local acct b/c it is a single user server
        if !search_term.contains("@") && !search_term.starts_with("@") {
            let empty: Vec<Account> = Vec::new();
            return Ok(empty);
        }

        let mut link_type: String = "application/activity+json".to_string();
        let mut actor_url: String = "".to_string();

        // Check acct (email) format
        let account_regex = 
        Regex::new(r"^([a-z0-9_+@]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}+$)").unwrap();
        if account_regex.is_match(search_term.as_str()) {

            if search_term.starts_with("@") {
                search_term = search_term[1..].to_string();
            }

            let webfinger =
               crate::webfinger::WebFinger::query(search_term.as_str()).await?;

            let links = webfinger.unwrap().links;

            for link in links.iter() {
                if link.rel == "self" {
                    link_type = link.link_type.to_owned().unwrap();
                    actor_url = link.href.to_owned().unwrap();
                }
            }
        }

        // Check url (actor url) format
        let url_regex = 
        Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
        if url_regex.is_match(search_term.as_str()) {
            actor_url = search_term.to_string();
        }

        if actor_url == "" { return Ok(Vec::new()) }
        let actor = Self::get_actor(actor_url, link_type).await?;

        // Put actor into Account tbl

        let acct_tbl = crate::table::account::Account::try_from(actor).unwrap();
        tracing::debug!("{:?}", acct_tbl);


        // Convert acct tbt to mastodon acct

        let acc = crate::mastodon::account::Account::try_from(acct_tbl).unwrap();
        
        let a = vec![acc];

        Ok(a)

    }

    pub async fn get_actor(actor_url: String, ct: String) -> Result<crate::activitypub::person_actor::PersonActor> {
        let request = Request::builder()
            .method(Method::Get)
            .header("Accept", ct)
            .uri(actor_url)
            .build();
        let response: Response = spin_sdk::http::send(request).await.unwrap();
        let status = response.status();
        let ct = response.header("content-type").unwrap().as_str().unwrap();

        let actor_str = str::from_utf8(response.body()).unwrap();
        let actor_value = serde_json::to_value(&actor_str).unwrap();

        crate::table::actor_json::ActorJson::put(serde_json::from_str(actor_str).unwrap())
            .await?;

        // Convert this to ActivityPub Actor
        let actor: crate::activitypub::person_actor::PersonActor = crate::activitypub::person_actor::PersonActor::try_from(actor_value).unwrap();

        tracing::debug!("-=-=-=-=--=--=-=-=-=-=-=-");
        tracing::debug!("{:?}", actor);
        

        Ok(actor)
    }
}
