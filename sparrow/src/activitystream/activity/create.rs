//! Create activity
//!
//!

use std::fmt;

use anyhow::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

use crate::activitystream::activity::Activity;
use crate::activitystream::activity::ActivityType;
use crate::activitystream::activity::Execute;
use crate::activitystream::actor::person::Person as PersonActor;
use crate::activitystream::object::note::Note as NoteObject;
use crate::activitystream::object::ObjectType;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::activity_log::ActivityLog;
use crate::mastodon::setting::Setting;
use crate::mastodon::status::Status as MStatus;
use crate::mastodon::ACTOR_ACCOUNT;
use chrono::Utc;

/// Accept activity struct.  
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Create(Value);

impl Create {
    /// resturn Accept object.  
    pub async fn new(actor: String, object: Value) -> Activity<Create> {
        let uuid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uuid);
        let published = Utc::now();

        let create_object = Activity::new(
            true,
            id,
            ActivityType::Create,
            actor.clone(),
            Some(published),
            Some(vec![
                "https://www.w3.org/ns/activitystreams#Public".to_string()
            ]),
            Some(vec![format!("{}/followers", actor)]),
            Create(object),
        );

        create_object
    }
}

impl fmt::Display for Create {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl fmt::Debug for Create {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl Execute for Create {
    async fn execute(&self, activity_val: Value) -> Result<()> {
        let a: &str = self.0.get("type").unwrap().as_str().unwrap();
        let object_type = ObjectType::from_str(a).unwrap();

        let actor_account = ACTOR_ACCOUNT.get().unwrap().to_owned();
        match object_type {
            ObjectType::Note => {
                create_note(self.to_owned(), activity_val, actor_account).await
            }
            unkown_type => unkown(unkown_type).await,
        }
    }
}

async fn create_note(
    s: Create,
    activity: Value,
    actor_account: MAccount,
) -> Result<()> {
    match serde_json::from_value::<NoteObject>(s.0.to_owned()) {
        Ok(note) => MStatus::new(note, actor_account).await,
        Err(e) => {
            tracing::error!("Error from Parsing NoteObject: {e:?}");
            tracing::error!("{:?}", s.0);
            tracing::error!("{activity:?}");
            Err(Error::msg("Error from Parsing NoteObject: {e:?}"))
        }
    }
}

async fn unkown(unknown_type: ObjectType) -> Result<()> {
    tracing::error!("Create '{:?}' is not implemented!", unknown_type);
    Err(Error::msg(format!(
        "Create '{:?}' is not implemented!",
        unknown_type
    )))
}
