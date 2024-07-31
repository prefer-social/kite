//! Create activity
//!
//!

use std::fmt;
use std::fmt::Debug;

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::activitystream;
use crate::activitystream::activity::Activity;
use crate::activitystream::activity::ActivityType;
use crate::activitystream::activity::Execute;
use crate::activitystream::actor::person::Person as PersonActor;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::activity_log::ActivityLog;
use crate::mastodon::follow::Follow as MFollow;
use crate::mastodon::setting::Setting;
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
    async fn execute(&self, actor: String) -> Result<()> {
        // Check activiy.object is what I really sent.
        // https://dev.prefer.social/0190fcb0-5272-77c3-acb1-3e9be71ff930
        // SELECT * FROM activity_log WHERE JSON_EXTRACT(body, '$.id') = ?
        match ActivityLog::get_with_id(
            self.0.get("id").unwrap().as_str().unwrap(),
        )
        .await
        .unwrap()
        {
            None => {
                tracing::error!(
                    "Havn't published this acticity {}",
                    self.0.get("id").unwrap().to_string()
                )
            }
            Some(x) => {
                let log_obj = activitystream::remove_context(x);
                let given_obj =
                    activitystream::remove_context(self.0.to_owned());
                if given_obj != log_obj {
                    tracing::error!(
                        "Integration error! No matching follow was published! {}", self.0.get("id").unwrap().to_string()
                    );
                    return Err(anyhow::Error::msg(
                        "Given activity is not published by SELF!",
                    ));
                }
            }
        };

        let subj = ActorUrl::new(
            self.0.get("actor").unwrap().as_str().unwrap().to_string(),
        )
        .unwrap();
        let obj = ActorUrl::new(
            self.0.get("object").unwrap().as_str().unwrap().to_string(),
        )
        .unwrap();
        let obj_id = self.0.get("id").unwrap().as_str().unwrap().to_string();

        let subj_account = MAccount::get(subj).await?;
        let subj_account_id = subj_account.uid;

        let obj_account = MAccount::get(obj).await?;
        let obj_account_id = obj_account.uid;

        MFollow::new(obj_id, subj_account_id, obj_account_id).await?;

        Ok(())
    }
}
