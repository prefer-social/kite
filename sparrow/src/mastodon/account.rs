//! Mastodon Account struct  
//! 
//! Mastodon reference: <https://docs.joinmastodon.org/entities/Account/>  

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
use crate::table::user::Get as _;
use crate::mastodon::account::field::Field;
use crate::mastodon::account::source::Source;
use crate::activitypub::person_actor::PersonActor;


/// Account struct  
/// 
/// Mastodon reference: <https://docs.joinmastodon.org/entities/Account/>  
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Account {
    /// The account id(UUID v7).
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: Uid,
    /// The username of the account, not including domain.
    pub username: Username,
    /// The Webfinger account URI. Equal to username for local users, or username@domain for remote users.
    pub acct: String,
    /// The location of the user’s profile page.
    pub url: String,
    /// The profile’s display name.
    pub display_name: String,
    /// The profile’s bio or description.
    pub note: String,
    /// An image icon URL that is shown next to statuses and in the profile. 
    pub avatar: String,
    /// A static version of the avatar URL. Equal to avatar if its value is a static image; different if avatar is an animated GIF.
    pub avatar_static: String,
    /// An image banner URL that is shown above the profile and in profile cards.
    pub header: String,
    /// A static version of the header URL. Equal to header if its value is a static image; different if header is an animated GIF.
    pub header_static: String,
    /// Whether the account manually approves follow requests.
    pub locked: bool,
    /// Additional metadata attached to a profile as name-value pairs.
    pub fields: Vec<Field>,
    /// Custom emoji entities to be used when rendering the profile.
    pub emojis: Vec<CustomEmoji>,
    /// Indicates that the account may perform automated actions, may not be monitored, or identifies as a robot.
    pub bot: bool,
    /// Indicates that the account represents a Group actor.
    pub group: bool,
    /// Whether the account has opted into discovery features such as the profile directory.
    /// Nullable
    pub discoverable: bool,
    /// Whether the local user has opted out of being indexed by search engines.
    /// Nullable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noindex: Option<bool>,
    /// Indicates that the profile is currently inactive and that its user has moved to a new account.
    /// Nullable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moved: Option<bool>,
    /// An extra attribute returned only when an account is suspended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    /// An extra attribute returned only when an account is silenced. If true, indicates that the account should be hidden behind a warning screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<bool>,
    /// When the account was created.
    pub created_at: DateTime<Utc>,
    /// When the most recent status was posted.
    /// Nullable
    pub last_status_at: DateTime<Utc>,
    /// How many statuses are attached to this account.
    pub statuses_count: u32,
    /// The reported followers of this profile.
    pub followers_count: u32,
    /// The reported follows of this profile.
    pub following_count: u32,
    /// Private_key of Account. Not serializable. Available only for local account 
    #[serde(skip_serializing, skip_deserializing)]
    pub private_key: Option<String>,
    /// Public_key of Account. Not serializable.
    #[serde(skip_serializing, skip_deserializing)]
    pub public_key: Option<String>,
}

type MAccount = Account;

impl Account {
    /// Getting Mastodon Account for default user(owner).  
    pub async fn default() -> Result<(MAccount, User)> {
        let du = crate::table::user::User::default_user()
            .await?
            .first()
            .unwrap()
            .to_owned();
        let user = crate::mastodon::user::User::from(du);
        let account_id =
            crate::mastodon::uid::Uid(user.account_id.clone().unwrap());
        let account = Self::fr_uid(account_id).await?;
        Ok((account, user))
    }

    /// Geting an Mastodon Account with User.  
    pub async fn to_user(&self) -> Result<Option<User>> {
        let account_id: String = self.uid.to_string();
        let users = crate::table::user::User::get(("account_id".to_string(), account_id)).await?;
        let user = users.last().map( |x| x.to_owned() );      

        if user.is_none() { return Ok(None) }
        let mastodon_user = crate::mastodon::user::User::from(user.unwrap());
        Ok(Some(mastodon_user))
    }

    /// Searching account.  
    /// Mastodon doc: <https://docs.joinmastodon.org/entities/Search/#accounts>
    pub async fn search(st: &String) -> Result<Vec<MAccount>> {
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


        tracing::debug!("actor_url: {}", actor_url);
        tracing::debug!("Getting an actor...");

        // Getting an (Person)Actor
        let actor = PersonActor::from_url(actor_url, link_type).await?;

        let acct_tbl: TAccount = TAccount::try_from(actor).unwrap();
        // INSERT tacct into Table
       
        
        //let acct_tbl = crate::table::account::Account::try_from(actor).unwrap();
        //tracing::debug!("acct_tbl -> {:?}", acct_tbl);
        let acc = MAccount::fr_tbl(acct_tbl).await.unwrap();
                
        Ok(vec![acc])


    }

    

    /// Getting statuses_count from Actor url.  
    pub async fn statuses_count(url: String) -> Result<u32> {
        let request = Request::builder()
            .method(Method::Get)
            .uri(url)
            .header("User-Agent", "prefer.social")
            .header("Accept", "application/activity+json")
            .build();
        let response: Response = spin_sdk::http::send(request).await?;
        let body = str::from_utf8(response.body()).unwrap();
        let v: crate::activitypub::outbox::Outbox = serde_json::from_str(body).unwrap();
        Ok(v.total_items as u32)
    }

    /// Get following_count from Actor url.
    pub async fn following_count(url: String) -> Result<u32> {
        let request = Request::builder()
            .method(Method::Get)
            .uri(url)
            .header("User-Agent", "prefer.social")
            .header("Accept", "application/activity+json")
            .build();
        let response: Response = spin_sdk::http::send(request).await?;
        let body = str::from_utf8(response.body()).unwrap();
        let v: crate::activitypub::following::Following = serde_json::from_str(body).unwrap();
        Ok(v.total_items as u32)
    }

    /// Get followers_count from Actor url.  
    pub async fn followers_count(url: String) -> Result<u32> {
        let request = Request::builder()
            .method(Method::Get)
            .uri(url)
            .header("User-Agent", "prefer.social")
            .header("Accept", "application/activity+json")
            .build();
        let response: Response = spin_sdk::http::send(request).await?;
        let body = str::from_utf8(response.body()).unwrap();
        let v: crate::activitypub::follower::Follower = serde_json::from_str(body).unwrap();
        Ok(v.total_items as u32)
    }

    /// Getting a (Mastodon) Account from uid.  
    pub async fn fr_uid(uid: Uid) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
             "uid".to_string(),
             uid.to_string(),
         ))
         .await
         .unwrap_or_default();
         let acct_tbl = accounts.into_iter().next().unwrap();
         self::Account::fr_tbl(acct_tbl).await
    }

    /// Getting a (Mastodon) Account from username.  
    pub async fn fr_username(username: Username) -> Result<Account> {
         let accounts: Vec<TAccount> = TAccount::get((
             "username".to_string(),
             username.to_string(),
         ))
         .await
         .unwrap_or_default();
         let acct_tbl = accounts.into_iter().next().unwrap();
         tracing::debug!("{:?}", acct_tbl);
         self::Account::fr_tbl(acct_tbl).await
     }

    /// Getting a (Mastodon) Account from Table Account struct.   
    pub async fn fr_tbl(tbl: TAccount) -> Result<Self> {
        
        let bot: bool = match tbl.actor_type.unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let account = Account {
            uid: tbl.uid.into(),
            username: tbl.username.clone().into(),
            acct: format!(
                "{}@{}",
                tbl.username.to_string(),
                tbl.domain.unwrap()
            ),
            display_name: tbl.display_name,
            locked: tbl.locked.unwrap_or_default(),
            bot: bot,
            discoverable: tbl.discoverable.unwrap_or_default(),
            created_at: DateTime::from_timestamp(
                tbl.created_at,
                0,
            )
            .unwrap(),
            note: tbl.note,
            url: tbl.url.unwrap_or_default(),
            avatar: tbl.avatar_remote_url.clone().unwrap_or("https://mstd.seungjin.net/avatars/original/missing.png".to_string()),
            avatar_static: tbl
                .avatar_remote_url
                .clone()
                .unwrap_or("https://mstd.seungjin.net/avatars/original/missing.png".to_string()),
            header: tbl.header_remote_url.clone().unwrap_or("https://mstd.seungjin.net/avatars/original/missing.png".to_string()),
            header_static: tbl.header_remote_url.unwrap_or("https://mstd.seungjin.net/avatars/original/missing.png".to_string()),
            followers_count: Self::followers_count(tbl.followers_url.to_owned().unwrap()).await?,
            following_count:  Self::following_count(tbl.following_url.to_owned().unwrap()).await?,
            statuses_count: Self::statuses_count(tbl.outbox_url.to_owned().unwrap()).await?,
            ..Default::default()
        };
        Ok(account)
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


#[async_trait(?Send)]
pub trait Get<T> {
    async fn get(a: T) -> Result<Account>;
}

#[async_trait(?Send)]
impl Get<Uid> for Account {
    async fn get(uid: Uid) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "uid".to_string(),
            uid.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        self::Account::fr_tbl(acct_tbl).await
    }   
}

#[async_trait(?Send)]
impl Get<Username> for Account {
    async fn get(username: Username) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "username".to_string(),
            username.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        self::Account::get(acct_tbl).await
    }
}

#[async_trait(?Send)]
impl Get<TAccount> for Account {
    async fn get(acct_tbl: TAccount) -> Result<Self> {

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
            display_name: acct_tbl.display_name,
            locked: acct_tbl.locked.unwrap_or_default(),
            bot: bot,
            discoverable: acct_tbl.discoverable.unwrap_or_default(),
            created_at: DateTime::from_timestamp(
                acct_tbl.created_at,
                0,
            )
            .unwrap(),
            note: acct_tbl.note,
            url: acct_tbl.url.unwrap_or_default(),
            avatar: acct_tbl.avatar_remote_url.clone().unwrap_or_default(),
            avatar_static: acct_tbl
                .avatar_remote_url
                .clone()
                .unwrap_or_default(),
            header: acct_tbl.header_remote_url.clone().unwrap_or_default(),
            header_static: acct_tbl.header_remote_url.unwrap_or_default(),
            followers_count: Self::followers_count(acct_tbl.followers_url.to_owned().unwrap()).await?,
            following_count:  Self::following_count(acct_tbl.followers_url.to_owned().unwrap()).await?,
            statuses_count: Self::statuses_count(acct_tbl.followers_url.unwrap().to_owned()).await?,
            ..Default::default()
        };
        Ok(account)
    }
}

