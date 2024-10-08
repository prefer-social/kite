//! Represents a status posted by an account.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Status/>

use anyhow::Result;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use struct_iterable::Iterable;
use uuid::Uuid;

use crate::activitystream::object::note::Note as NoteObject;
use crate::mastodon::{
    account::actor_url::ActorUrl, account::uri::Uri as AccountUri,
    account::Account as MAccount, account::Get, custom_emoji::CustomEmoji,
    filter_result::FilterResult, media_attachment::MediaAttachment,
    poll::Poll, preview_card::PreviewCard, ACTOR_ACCOUNT, ME_ACCOUNT,
};
use crate::table::account::Account as TAccount;
use crate::table::status::Status as TStatus;
use crate::table::New;

/// Represents a status posted by an account.  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Status/>
#[derive(
    Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone, Iterable,
)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    // rowid from sqlite
    pub rowid: Option<i64>,
    /// ID(uuid v7) of the status in the database.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    /// URI of the status used for federation. actor_url
    pub uri: Option<String>,
    ///  The date when this status was created.
    pub created_at: DateTime<Utc>,
    ///  The account that authored this status.
    pub account: MAccount,
    /// HTML-encoded status content.
    pub content: String,
    /// Visibility of this status.
    /// String (Enumerable oneOf)
    /// `public` = Visible to everyone, shown in public timelines.
    /// `unlisted` = Visible to public, but not included in public timelines.
    /// `private` = Visible to followers only, and to any mentioned users.
    /// `direct` = Visible only to mentioned users.
    pub visibility: String,
    /// Is this status marked as sensitive content?
    pub sensitive: bool,
    /// Subject or summary line, below which status content is collapsed until expanded.
    pub spoiler_text: String,
    /// Media that is attached to this status.
    pub media_attachments: Vec<MediaAttachment>,
    /// The application used to post this status.
    pub application: Option<Application>,
    ///  Mentions of users within the status content.
    pub mentions: Vec<Metion>,
    /// Hashtags used within the status content.
    pub tags: Vec<Tag>,
    /// Custom emoji to be used when rendering status content.
    pub emojis: Vec<CustomEmoji>,
    /// How many boosts this status has received.
    pub reblogs_count: i64,
    /// How many favourites this status has received.
    pub favourites_count: i64,
    /// How many replies this status has received.
    pub replies_count: i64,
    /// A link to the status’s HTML representation.
    /// Nullable String (URL) or null
    pub url: Option<String>,
    /// ID of the status being replied to.
    /// Nullable String (cast from an integer but not guaranteed to be a number) or null
    pub in_reply_to_id: Option<String>,
    /// ID of the account that authored the status being replied to.
    /// Nullable String (cast from an integer but not guaranteed to be a number) or null
    pub in_reply_to_account_id: Option<String>,
    /// The status being reblogged.
    /// Nullable Status or null
    pub reblog: Option<Box<Status>>,
    /// The poll attached to the status.
    /// Nullable Poll or null
    pub poll: Option<Poll>,
    /// Preview card for links included within status content.
    /// Nullable PreviewCard or null
    pub card: Option<PreviewCard>,
    /// Primary language of this status.
    /// Nullable String (ISO 639 Part 1 two-letter language code) or null
    pub language: String,
    /// Plain-text source of a status.
    /// Returned instead of content when status is deleted, so the user may redraft from the source text without the client having to reverse-engineer the original text from the HTML content.
    /// Nullable String or null
    pub text: String,
    /// Timestamp of when the status was last edited.
    /// Nullable String (ISO 8601 Datetime)
    pub edited_at: String,
    /// If the current token has an authorized user: Have you favourited this status?
    pub favourited: Option<bool>,
    /// If the current token has an authorized user: Have you boosted this status?
    pub reblogged: Option<bool>,
    /// If the current token has an authorized user: Have you muted notifications for this status’s conversation?
    pub muted: Option<bool>,
    /// If the current token has an authorized user: Have you bookmarked this status?
    pub bookmarked: Option<bool>,
    /// If the current token has an authorized user: Have you pinned this status? Only appears if the status is pinnable.
    pub pinned: Option<bool>,
    /// If the current token has an authorized user: The filter and keywords that matched this status.
    pub filtered: Vec<FilterResult>,
}

impl From<crate::table::status::Status> for Status {
    fn from(_tbl: crate::table::status::Status) -> Self {
        let status = Status {
            ..Default::default()
        };
        status
    }
}

impl Into<String> for Status {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for Status {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

impl Status {
    //whose, how many/when
    pub async fn get(_a: MAccount) {}

    pub async fn count(taccount: TAccount) -> Result<u64> {
        Ok(TStatus::count(taccount).await.unwrap() as u64)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Metion {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

impl Status {
    pub async fn search(_search_term: &String) -> Result<Vec<Status>> {
        Ok(Vec::new())
    }
}

/// The application used to post status.
#[derive(
    Hash, Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone,
)]
pub struct Application {
    /// The name of the application that posted this status.
    pub name: String,
    /// The website associated with the application that posted this status.
    /// Nullable String (URL) or null
    pub website: String,
}

impl Status {
    pub async fn new(note: NoteObject, actor_account: MAccount) -> Result<()> {
        let note_published_at = note.published.unwrap();
        let created_at = NaiveDateTime::parse_from_str(
            &note_published_at.as_str(),
            "%Y-%m-%dT%H:%M:%SZ",
        )
        .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc))
        .unwrap();

        let status = Status {
            uid: Uuid::now_v7().to_string(),
            uri: Some(note.id),
            created_at: created_at,
            account: actor_account,
            //content,
            //visibility: note,
            sensitive: note.sensitivity.unwrap_or_default(),
            // spoiler_text,
            media_attachments: vec![],
            // application,
            // mentions,
            // tags,
            // emojis,
            // reblogs_count,
            // favourites_count,
            // replies_count,
            url: Some(note.url.unwrap()),
            in_reply_to_id: None,
            in_reply_to_account_id: None,
            // reblog,
            // poll,
            // card,
            language: "en".to_string(),
            text: note.content.unwrap(),
            // edited_at,
            // favourited,
            // reblogged,
            // muted,
            // bookmarked,
            // pinned,
            // filtered,
            ..Default::default()
        };

        status.save().await
    }

    pub async fn save(&self) -> Result<()> {
        let a = TStatus::try_from(self.to_owned()).unwrap();
        a.new().await
    }
}
