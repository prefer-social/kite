//! Mastodon Account struct  
//! 
//! Mastodon reference: <https://docs.joinmastodon.org/entities/Account/>  

use anyhow::{Result, Error};
use async_trait::async_trait;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::str;
use regex::Regex;
use struct_iterable::Iterable;

use crate::activitystream::activity::follow::Follow as FollowActivity;
use crate::activitystream::ordered_collection::OrderedCollection;
use crate::mastodon::custom_emoji::CustomEmoji;
use crate::mastodon::user::User;
use crate::mastodon::user_role::UserRole as Role;
use crate::mastodon::account::username::Username;
use crate::mastodon::account::field::Field;
use crate::mastodon::account::source::Source;
use crate::mastodon::account::uri::Uri as AccountUri;
use crate::mastodon::account::uid::Uid as AccountUid;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::follow::Follow;
use crate::mastodon::status::Status;
use crate::table::account::Account as TAccount;
use crate::table::account::Get as _;
use crate::table::account::Remove as _;
use crate::table::user::Get as _;
use crate::table::account::New as _;


pub mod source;
pub mod field;
//pub mod credential_account;
//pub mod muted_account;
pub mod username; 
pub mod uri;
pub mod uid;
pub mod actor_url;

/// Account struct  
/// 
/// Mastodon reference: <https://docs.joinmastodon.org/entities/Account/>  
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Iterable)]
pub struct Account {
    /// The account id(UUID v7).
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: AccountUid,
    /// The username of the account, not including domain.
    pub username: Username,
    /// The Webfinger account URI. Equal to username for local users, or username@domain for remote users.
    #[serde(rename(serialize = "acct", deserialize = "acct"))]
    pub account_uri: AccountUri,
    /// Actor uri (url form). Not for serializing
    #[serde(skip_serializing, skip_deserializing)]
    pub actor_url: ActorUrl, 
    /// The location of the user’s profile page.
    pub url: String,
    /// The profile’s display name.
    pub display_name: String,
    /// The profile’s bio or description.
    pub note: String,
    /// An image icon URL that is shown next to statuses and in the profile.  
    #[serde(default = "default_avatar")]
    pub avatar: String,
    /// A static version of the avatar URL. Equal to avatar if its value is a static image; different if avatar is an animated GIF.
    #[serde(default = "default_avatar")]
    pub avatar_static: String,
    /// An image banner URL that is shown above the profile and in profile cards.
    #[serde(default = "default_header")]
    pub header: String,
    /// A static version of the header URL. Equal to header if its value is a static image; different if header is an animated GIF.
    #[serde(default = "default_header")]
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
    pub statuses_count: u64,
    /// The reported followers of this profile.
    pub followers_count: u64,
    /// The reported follows of this profile.
    pub following_count: u64,
    /// Private_key of Account. Not serializable. Available only for local account 
    #[serde(skip_serializing, skip_deserializing)]
    pub private_key: Option<String>,
    /// Public_key of Account. Not serializable.
    #[serde(skip_serializing, skip_deserializing)]
    pub public_key: String,
    /// inbox_url of Account. Not serializable
    #[serde(skip_serializing, skip_deserializing)]
    pub inbox_url: Option<String>,
    /// outbox_url of Account. Not serializable
    #[serde(skip_serializing, skip_deserializing)]
    pub outbox_url: Option<String>,
    /// shared_inbox_url of Account. Not serializable
    #[serde(skip_serializing, skip_deserializing)]
    pub shared_inbox_url: Option<String>,
    /// following_url of Account. Not serializable
    #[serde(skip_serializing, skip_deserializing)]
    pub following_url: Option<String>,
    /// followers_url of Account. Not serializable
    #[serde(skip_serializing, skip_deserializing)]
    pub followers_url: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub indexable: Option<bool>,
    /// Only for credential account
    /// CredentialAccount entity attributes
    /// An extra attribute that contains source values to be used with API methods that verify credentials and update credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// The role assigned to the currently authorized user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Only for muted account
    /// MutedAccount entity attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_expires_at: Option<String>,

}

type MAccount = Account;

impl Account {
    /// Getting a (Mastodon) Account from Table Account struct.   
    async fn from_table(acct_tbl: TAccount) -> Result<Self> {
        
        let bot: bool = match acct_tbl.actor_type.clone().unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let followers_count: u64;
        let following_count: u64;
        let statuses_count: u64;

        let account_uri = AccountUri {
            username: acct_tbl.username.clone(),
            domain: acct_tbl.domain.clone(),
        };

        if acct_tbl.private_key.is_some() { // Local user 
            //tracing::debug!("LOCAL USER");
            followers_count = Follow::follower_count(account_uri.clone()).await?;
            following_count = 11; //Follow::following_count(account_uri.clone()).await?;
            statuses_count = 11; //Status::count(account_uri).await?;
        } else { // Remote user
            //tracing::debug!("REMOTE USER");
            followers_count = Self::followers_count(acct_tbl.followers_url.clone().unwrap()).await?.into();
            following_count =  Self::following_count(acct_tbl.following_url.clone().unwrap()).await?.into();
            statuses_count = Self::statuses_count(acct_tbl.outbox_url.clone().unwrap().to_owned()).await.unwrap();
        }

        let account_uri = AccountUri::new(acct_tbl.username.clone(), acct_tbl.domain);

        let account = Account {
            uid: acct_tbl.uid.clone().into(),
            username: acct_tbl.username.clone().into(),
            account_uri,
            display_name: acct_tbl.display_name,
            locked: convert_to_bool(acct_tbl.locked.unwrap_or_default()),
            bot: bot,
            discoverable: convert_to_bool(acct_tbl.discoverable.unwrap_or_default()),
            created_at: DateTime::from_timestamp(
                acct_tbl.created_at,
                0,
            )
            .unwrap(),
            note: acct_tbl.note,
            url: acct_tbl.url.unwrap_or_default(),
            actor_url: ActorUrl::new(acct_tbl.uri)?,
            avatar: acct_tbl.avatar_remote_url.clone().unwrap_or(default_avatar()),
            avatar_static: acct_tbl
                .avatar_remote_url
                .clone()
                .unwrap_or(default_avatar()),
            header: acct_tbl.header_remote_url.clone().unwrap_or(default_header()),
            header_static: acct_tbl.header_remote_url.clone().unwrap_or(default_header()),
            followers_count,
            following_count,
            statuses_count,
            public_key: acct_tbl.public_key,
            private_key: acct_tbl.private_key,
            inbox_url: acct_tbl.inbox_url,
            outbox_url: acct_tbl.outbox_url,
            shared_inbox_url: acct_tbl.shared_inbox_url,
            following_url: acct_tbl.following_url,
            followers_url: acct_tbl.followers_url,
            indexable: Some(convert_to_bool(acct_tbl.indexable.unwrap())),
            ..Default::default()
        };
        Ok(account)
    }

    /// Getting Mastodon Account for default user(owner).  
    pub async fn default() -> Result<(MAccount, User)> {
        let user = crate::mastodon::user::User::default().await?;
        let account_id =
            AccountUid(user.account_id.clone().unwrap());
        let account = Self::get(account_id).await?;
        Ok((account, user))
    }

    /// Geting an Mastodon Account with User.  
    pub async fn user(&self) -> Result<Option<User>> {
        let account_id: String = self.uid.to_string();
        let users = crate::table::user::User::get(("account_id".to_string(), account_id)).await?;
        let user = users.last().map( |x| x.to_owned() );      

        if user.is_none() { return Ok(None) }
        let mastodon_user = crate::mastodon::user::User::from(user.unwrap());
        Ok(Some(mastodon_user))
    }

    /// Is this Mastodon account local?  
    pub fn local(&self) -> bool {
        self.account_uri.domain.is_none()
    }

    /// Searching account.  
    /// Mastodon doc: <https://docs.joinmastodon.org/entities/Search/#accounts>
    pub async fn search(st: &String) -> Result<Vec<MAccount>> {
        let mut search_term: String = st.to_string().to_lowercase();
        // Local account: Don't search local acct b/c it is a single user server
        if !search_term.contains("@") && !search_term.starts_with("@") {
            let empty: Vec<Account> = Vec::new();
            return Ok(empty);
        }

        let mut link_type: String = "application/activity+json".to_string();
        let mut actor_url: String = "".to_string();

        // Check account uri(email) format
        let account_regex = 
            Regex::new(r"^([a-z0-9_+@]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}+$)").unwrap();
        if account_regex.is_match(search_term.as_str()) {
            tracing::debug!("Search: AccountUri");
            if search_term.starts_with("@") {
                search_term = search_term[1..].to_string();
            }

            let webfinger =
               crate::webfinger::WebFinger::query(search_term.as_str()).await?;

            // Webfinger query returns None.  
            if webfinger.is_none() {
                return Ok(Vec::new());
            }

            let links = webfinger.unwrap().links;

            for link in links.iter() {
                if link.rel == "self" {
                    link_type = link.link_type.clone().unwrap();
                    actor_url = link.href.clone().unwrap();
                }
            }
        }

        // Check url(actor url) format
        let url_regex = 
            Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();
        if url_regex.is_match(search_term.as_str()) {
            tracing::debug!("Seach ActorUrl");
            actor_url = search_term;
        }

        if actor_url == "" { return Ok(Vec::new()) }


        //tracing::debug!("actor_url: {}", actor_url);
        //tracing::debug!("Getting an actor...");

        // Getting an (Person)Actor
        //let actor = PersonActor::from_url(actor_url.as_str(), link_type.as_str()).await?;

        let actor_url = ActorUrl::new(actor_url)?;
        let actor = actor_url.actor().await?;
        let acct_tbl: TAccount = TAccount::try_from(actor).unwrap();

        //tracing::debug!("Generated TAccount from received actor: {:?}", acct_tbl);

        // INSERT this searched account into Account table.
        TAccount::new(acct_tbl.clone()).await?;

        //tracing::debug!("{:?}", acct_tbl);

        //let acc = MAccount::fr_tbl(acct_tbl).await.unwrap();
        let actor_url = ActorUrl::new(acct_tbl.uri).unwrap();
        let acc = MAccount::get(actor_url).await?;

        //tracing::debug!("Generated MAccount from above TAccount: {:?}", acc);
        
        Ok(vec![acc])


    }

    /// Getting statuses_count from Actor url.  
    pub async fn statuses_count(url: String) -> Result<u64> {
        let request = Request::builder()
            .method(Method::Get)
            .uri(url)
            .header("User-Agent", "prefer.social")
            .header("Accept", "application/activity+json")
            .build();
        let response: Response = spin_sdk::http::send(request).await?;
        match response.status() {
            410u16 => { return Err(anyhow::Error::msg("Resource is Gone")); },
            _ => (),
        }
        let body = str::from_utf8(response.body()).unwrap();
        let v: OrderedCollection = serde_json::from_str(body).unwrap();
        Ok( v.total_items as u64)
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
        match response.status() {
            410u16 => { return Err(anyhow::Error::msg("Resource is Gone")); },
            _ => (),
        }
        let body = str::from_utf8(response.body()).unwrap();
        let v: OrderedCollection = serde_json::from_str(body).unwrap();
        Ok(v.total_items as u32)
    }

    /// Get followers_count from Actor url.  
    pub async fn followers_count(url: String) -> Result<u32> {
        // If url is not local.
        let request = Request::builder()
            .method(Method::Get)
            .uri(url)
            .header("User-Agent", "prefer.social")
            .header("Accept", "application/activity+json")
            .build();
        let response: Response = spin_sdk::http::send(request).await?;
        match response.status() {
            410u16 => { return Err(anyhow::Error::msg("Resource is Gone")); },
            _ => (),
        }
        let body = str::from_utf8(response.body()).unwrap();
        
        let v: OrderedCollection = serde_json::from_str(body).unwrap();
        Ok(v.total_items as u32)
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

/// Get trait for account.  
#[async_trait(?Send)]
pub trait Get<T> {
    /// Getter for account. 
    async fn get(a: T) -> Result<Account>;
}

#[async_trait(?Send)]
impl Get<TAccount> for Account {
    async fn get(acct_tbl: TAccount) -> Result<Self> {
        let bot: bool = match acct_tbl.actor_type.clone().unwrap().as_str() {
            "service" => true,
            _ => false,
        };

        let followers_count: u64;
        let following_count: u64;
        let statuses_count: u64;

        let account_uri = AccountUri {
            username: acct_tbl.username.clone(),
            domain: acct_tbl.domain.clone(),
        };

        if acct_tbl.domain.is_none() { // Local user 
            //tracing::debug!("LOCAL USER");
            followers_count = Follow::follower_count(account_uri.clone()).await?;
            following_count = Follow::following_count(account_uri.clone()).await?;
            statuses_count = Status::count(account_uri).await?;
        } else { // Remote user
            //tracing::debug!("REMOTE USER");
            followers_count = Self::followers_count(acct_tbl.followers_url.clone().unwrap()).await?.into();
            following_count =  Self::following_count(acct_tbl.following_url.clone().unwrap()).await?.into();
            statuses_count = Self::statuses_count(acct_tbl.outbox_url.clone().unwrap().to_owned()).await.unwrap();
        }

        let account_uri = AccountUri::new(acct_tbl.username.clone(), acct_tbl.domain);

        let account = Account {
            uid: acct_tbl.uid.clone().into(),
            username: acct_tbl.username.clone().into(),
            account_uri,
            display_name: acct_tbl.display_name,
            locked: convert_to_bool(acct_tbl.locked.unwrap_or_default()),
            bot: bot,
            discoverable: convert_to_bool(acct_tbl.discoverable.unwrap_or_default()),
            created_at: DateTime::from_timestamp(
                acct_tbl.created_at,
                0,
            )
            .unwrap(),
            note: acct_tbl.note,
            url: acct_tbl.url.unwrap_or_default(),
            actor_url: ActorUrl::new(acct_tbl.uri)?,
            avatar: acct_tbl.avatar_remote_url.clone().unwrap_or(default_avatar()),
            avatar_static: acct_tbl
                .avatar_remote_url
                .clone()
                .unwrap_or(default_avatar()),
            header: acct_tbl.header_remote_url.clone().unwrap_or(default_header()),
            header_static: acct_tbl.header_remote_url.clone().unwrap_or(default_header()),
            followers_count,
            following_count,
            statuses_count,
            public_key: acct_tbl.public_key,
            private_key: acct_tbl.private_key,
            inbox_url: acct_tbl.inbox_url,
            outbox_url: acct_tbl.outbox_url,
            shared_inbox_url: acct_tbl.shared_inbox_url,
            following_url: acct_tbl.following_url,
            followers_url: acct_tbl.followers_url,
            indexable: Some(convert_to_bool(acct_tbl.indexable.unwrap())),
            ..Default::default()
        };
        Ok(account)
    }
}

#[async_trait(?Send)]
impl Get<AccountUid> for Account {
    async fn get(uid: AccountUid) -> Result<Account> {
        let accounts = crate::table::account::Account::get((
            "uid".to_string(),
            uid.to_string(),
        ))
        .await
        .unwrap_or_default();
        let acct_tbl = accounts.into_iter().next().unwrap();
        Self::get(acct_tbl).await
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
impl Get<AccountUri> for Account {
    async fn get(uri: AccountUri) -> Result<Account> {
        let accounts = crate::table::account::Account::get(uri).await?;
        let acct_tbl: TAccount = accounts.last().unwrap().to_owned();
        let a = Self::from_table(acct_tbl).await?;
        Ok(a)
    }
}

#[async_trait(?Send)]
impl Get<ActorUrl> for Account {
    async fn get(actor_url: ActorUrl) -> Result<Self> {
        let url_string = actor_url.to_string();
        let taccounts = TAccount::fr_actor_url(url_string).await?;
        let taccount = taccounts.last().unwrap();
        let maccount = Self::from_table(taccount.to_owned()).await?;
        Ok(maccount)

    }
}

/// Get trait for account.  
#[async_trait(?Send)]
pub trait New<T> {
    /// Getter for account. 
    async fn new(a: T) -> Result<()>;
}

#[async_trait(?Send)]
pub trait Remove<T> {
    /// Getter for account. 
    async fn remove(a: T) -> Result<()>;
}

#[async_trait(?Send)]
impl Remove<AccountUri> for Account {
    async fn remove(uri: AccountUri) -> Result<()> {
        TAccount::remove(uri).await
    }
}

#[async_trait(?Send)]
impl Remove<ActorUrl> for Account {
    async fn remove(url: ActorUrl) -> Result<()> {
        TAccount::remove(url).await
    }
}

fn convert_to_bool(value: i64) -> bool {
    match value {
        0 => false, 
        _ => true,
    }
}

fn default_avatar() -> String {
    "https://mstd.seungjin.net/avatars/original/missing.png".to_string()
}

fn default_header() -> String {
    "https://mstd.seungjin.net/headers/original/missing.png".to_string()
}
