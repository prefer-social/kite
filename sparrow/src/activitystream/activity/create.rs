//! Create activity
//!
//!

use std::fmt;
use std::fmt::Debug;

use anyhow::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

use crate::activitystream;
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
use chrono::{DateTime, Utc};

/// Accept activity struct.  
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Create(Value);

/*
{ "@context":["https://www.w3.org/ns/activitystreams",{"ostatus":"http://ostatus.org#","atomUri":"ostatus:atomUri","inReplyToAtomUri":"ostatus:inReplyToAtomUri","conversation":"ostatus:conversation","sensitive":"as:sensitive","toot":"http://joinmastodon.org/ns#","votersCount":"toot:votersCount"}],
  "id":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/activity",
  "type":"Create",
  "actor":"https://mstd.seungjin.net/users/wsj",
  "published":"2024-07-31T13:23:08Z",
  "to":["https://www.w3.org/ns/activitystreams#Public"],
  "cc":["https://mstd.seungjin.net/users/wsj/followers"],
  "object":{
    "id":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598",
    "type":"Note",
    "summary":null,
    "inReplyTo":null,
    "published":"2024-07-31T13:23:08Z",
    "url":"https://mstd.seungjin.net/@wsj/112881315922274598",
    "attributedTo":"https://mstd.seungjin.net/users/wsj",
    "to":["https://www.w3.org/ns/activitystreams#Public"],
    "cc":["https://mstd.seungjin.net/users/wsj/followers"],
    "sensitive":false,
    "atomUri":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598",
    "inReplyToAtomUri":null,
    "conversation": "tag:mstd.seungjin.net,2024-07-31:objectId=2214270:objectType=Conversation",
    "content":"FooFoo",
    "contentMap":{"en":"FooFoo"},
    "attachment":[],
    "tag":[],
    "replies":{
      "id":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/replies",
      "type":"Collection",
      "first":{
        "type":"CollectionPage",
        "next":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/replies?only_other_accounts=true\u0026page=true",
        "partOf":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/replies",
        "items":[]
      }
    }
  },
  "signature": {
    "type":"RsaSignature2017",
    "creator":"https://mstd.seungjin.net/users/wsj#main-key",
    "created":"2024-07-31T13:23:14Z",
    "signatureValue":"Dlby/9rMnA6PJqqdUv/eVuNjTPPbwmG2JPf2SKiMqvLZ7BtJ+DFbraMHIp6NmnYbvN0CV9OZQ7HNo3Rvr4vVeARzBfwCLiyp2zNh/GVqtlyaDDVHmCiFvwYzbVqAMBj8dYklHcZiU5zOeaMfznt8q9I9WPuPruqrPrFcZZNEd5nbSSTe17CVkJLeAeQpMrg6uE8MurjolMkzqwakmOjn3pZbbXASXIJvbjOlR/c5+SG9zOnOqJLv8x9fVkanZJlwlZD3CuhfTaiokKcNZpwviYzOTlHxDvHQ5lmISXWQP3FC8seNy2+dvhCgtLdOZwduymOj2zW1fq/4QqHhx0jX7w=="
  }
}

  Ok( {
  "id":"https://mas.to/users/seungjin/statuses/112885048174202789",
  "type":"Note",
  "summary":null,
  "inReplyTo":null,
  "published":"2024-08-01T05:12:18Z",
  "url":"https://mas.to/@seungjin/112885048174202789",
  "attributedTo":"https://mas.to/users/seungjin",
  "to":["https://www.w3.org/ns/activitystreams#Public"],
  "bto":null,
  "cc":["https://mas.to/users/seungjin/followers"],
  "bcc":null,
  "sensitivity":null,
  "atomUrl":null,
  "inReplyToAtomUri":null,
  "conversation": "tag:mas.to,2024-08-01:objectId=369259710:objectType=Conversation",
  "content":"<p>tdd</p>",
  "contentMap":{"en":"<p>tdd</p>"},
  "attachment":[],
  "tag":[],
  "replies":
    {"id":"https://mas.to/users/seungjin/statuses/112885048174202789/replies",
      "type":"Collection","first":{"type":"CollectionPage",
      "next":"https://mas.to/users/seungjin/statuses/112885048174202789/replies?only_other_accounts=true&page=true",
      "partOf":"https://mas.to/users/seungjin/statuses/112885048174202789/replies","items":[]}}})



*/

impl Create {
    /// resturn Accept object.  
    pub async fn new(actor: String, object: Value) -> Activity<Create> {
        let uuid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uuid);
        let published = Utc::now();

        let create_object = Activity::new(
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

        match object_type {
            ObjectType::Note => create_note(self.to_owned(), activity_val).await,
            unkown_type => unkown(unkown_type).await,
        }
    }
}

async fn create_note(s: Create, activity: Value) -> Result<()> {
    match serde_json::from_value::<NoteObject>(s.0.to_owned()) {
        Ok(note) => MStatus::new(note).await,
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
