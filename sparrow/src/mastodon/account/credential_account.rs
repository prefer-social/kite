//! CredentialAccount: Return account when verify_credentials is passed.  
//!
//! Based on Account but extra fields are added.  
//! Extra fileds: role, source  
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Account/#CredentialAccount>  

use anyhow::Result;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::str;

use crate::mastodon::account::field::Field;
use crate::mastodon::account::uid::Uid;
use crate::mastodon::account::uri::Uri as AccountUri;
use crate::mastodon::account::username::Username;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Role;
use crate::mastodon::account::Source;
use crate::mastodon::custom_emoji::CustomEmoji;

/// CredentialAccount: Return account when verify_credentials is passed.  
/// Based on Account but extra fields are added.  
/// Extra fileds: role, source  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Account/#CredentialAccount>  
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CredentialAccount {
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
    /// CredentialAccount entity attributes
    /// An extra attribute that contains source values to be used with API methods that verify credentials and update credentials.
    pub source: Source,
    /// The role assigned to the currently authorized user.
    pub role: Role,
}

impl CredentialAccount {
    pub async fn get(a: MAccount) -> Result<CredentialAccount> {
        //let role;
        //let source;

        use crate::mastodon::user::Get as _;
        let user = crate::mastodon::user::User::get(a.clone())
            .await?
            .unwrap()
            .to_owned();

        let role = crate::mastodon::user_role::UserRole::get(user)
            .await?
            .unwrap();

        let source = crate::mastodon::account::Source::get().await?;

        Ok(CredentialAccount {
            uid: a.uid,
            username: a.username,
            acct: AccountUri::try_from(a.acct).unwrap().to_string(),
            url: a.url,
            display_name: a.display_name,
            note: a.note,
            avatar: a.avatar,
            avatar_static: a.avatar_static,
            header: a.header,
            header_static: a.header_static,
            locked: a.locked,
            fields: a.fields,
            emojis: a.emojis,
            bot: a.bot,
            group: a.group,
            discoverable: a.discoverable,
            noindex: a.noindex,
            moved: a.moved,
            suspended: a.suspended,
            limited: a.limited,
            created_at: a.created_at,
            last_status_at: a.last_status_at,
            statuses_count: a.statuses_count,
            followers_count: a.followers_count,
            following_count: a.following_count,
            private_key: a.private_key,
            public_key: a.public_key,
            // CredentialAccount entity attributes
            source: source,
            role: role,
        })
    }
}
