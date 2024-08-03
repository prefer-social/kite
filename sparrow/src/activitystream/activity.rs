//! An Activity is a subtype of Object that describes some form of action that may happen, is currently happening, or has already happened. The Activity type itself serves as an abstract base type for all types of activities. It is important to note that the Activity type itself does not carry any specific semantics about the kind of action being taken.
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-activity>
//! <https://www.w3.org/TR/activitystreams-vocabulary/#activity-types>
//! Resource: <https://www.w3.org/TR/activitypub/#obj>

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

use crate::activitystream;
use crate::activitystream::activity::accept::Accept;
use crate::activitystream::activity::follow::Follow;
use crate::activitystream::Execute;
use crate::mastodon;
//use crate::activitystream::activity::undo::Undo;
use crate::mastodon::account::Account as MAccount;

pub mod accept;
pub mod create;
pub mod delete;
pub mod follow;

/// ActivityPub Object Types
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum ActivityType {
    Follow,
    Accept,
    Delete,
    Undo,
    Reject,
    Note,
    Create,
    Replies,
    RsaSignature2017,
    OrderedCollection,
    OrderedCollectionPage,
    #[default]
    NotDefined,
}

impl FromStr for ActivityType {
    type Err = ();
    fn from_str(input: &str) -> Result<ActivityType, Self::Err> {
        match input {
            "Follow" => Ok(ActivityType::Follow),
            "Accept" => Ok(ActivityType::Accept),
            "Delete" => Ok(ActivityType::Delete),
            "Undo" => Ok(ActivityType::Undo),
            "Reject" => Ok(ActivityType::Reject),
            "Note" => Ok(ActivityType::Note),
            "Create" => Ok(ActivityType::Create),
            "Replies" => Ok(ActivityType::Replies),
            "RsaSignature2017" => Ok(ActivityType::RsaSignature2017),
            "OrderedCollection" => Ok(ActivityType::OrderedCollection),
            "OrderedCollectionPage" => Ok(ActivityType::OrderedCollectionPage),
            _ => Ok(ActivityType::NotDefined),
        }
    }
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ActicityPub Object base template
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Activity<T>
where
    T: Debug + Serialize + ToString + Execute,
{
    #[serde(rename = "@context")]
    pub context: Value,
    pub id: String,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(rename = "object")]
    pub activity_object: T,
}

impl<T> Activity<T>
where
    T: Debug
        + Serialize
        + ToString
        + PartialEq
        + Eq
        + Clone
        + Default
        + Execute,
{
    /// Create new ActivityPub object.  
    pub fn new(
        id: String,
        activity_type: ActivityType,
        actor: String,
        published: Option<DateTime<Utc>>,
        to: Option<Vec<String>>,
        cc: Option<Vec<String>>,
        activity: T,
    ) -> Self {
        Activity {
            context: activitystream::default_context(),
            id,
            activity_type,
            actor,
            published,
            to,
            cc,
            activity_object: activity,
        }
    }

    /// Execute activity.  
    pub async fn execute(&self) -> Result<()> {
        // Todo: get an account from auth token.
        let (my_account, _) = MAccount::default().await?;

        // If an actor is self, publish/send to world
        if self.actor == my_account.actor_url.to_string() {
            match mastodon::publish_activity(self.to_owned()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    tracing::error!("{e:?}");
                    return Err(e);
                }
            }
        }

        // Run internal execution.
        self.activity_object.execute(self.actor.to_owned()).await
    }
}

use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}