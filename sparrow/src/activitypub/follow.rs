//! ActivityPub's Follow Object.  

use std::fmt;
use std::fmt::Debug;

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::activitypub::actor::Actor;
use crate::activitypub::object::Object as APObject;
use crate::activitypub::object::ObjectType;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::follow::Follow as MFollow;
use crate::mastodon::setting::Setting;

pub mod follower;
pub mod following;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Follow {
//     #[serde(default = "default_context")]
//     pub context: String,
//     pub id: String,
//     #[serde(default = "follow")]
//     pub kind: String,
//     //pub actor: PersonActor,
//     //pub object: PersonActor,
//     pub actor: String,
//     pub object: String,
// }

/// Follow actor object.  
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Follow(String);

impl Follow {
    /// resturn Follow object.  
    pub async fn new(actor: String, object: String) -> APObject<Follow> {
        let uuid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uuid);

        let follow_object = APObject::new(
            id,
            ObjectType::Follow,
            actor,
            None,
            None,
            None,
            Follow(object),
        );

        follow_object
    }

    /// when follow action received at inbox.  
    pub async fn receive(ap_obj: APObject<Value>) -> Result<APObject<Self>> {
        let subj = ActorUrl::new(ap_obj.actor.clone()).unwrap();
        let obj = ActorUrl::new(ap_obj.object.as_str().unwrap().to_string())
            .unwrap();
        let obj_id = ap_obj.id;

        let subj_account = MAccount::get(subj).await?;
        let subj_account_id = subj_account.uid;

        let obj_account = MAccount::get(obj).await?;
        let obj_account_id = obj_account.uid;

        MFollow::new(obj_id.clone(), subj_account_id, obj_account_id).await?;

        let follow_object = APObject {
            context: ap_obj.context,
            id: obj_id,
            object_type: ObjectType::Follow,
            actor: ap_obj.actor,
            published: None,
            to: None,
            cc: None,
            object: Follow(ap_obj.object.to_string()),
        };

        Ok(follow_object)
    }
}

impl fmt::Display for Follow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Follow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn default_context() -> String {
    "https://www.w3.org/ns/activitystreams".to_string()
}

fn follow() -> String {
    "Follow".to_string()
}
