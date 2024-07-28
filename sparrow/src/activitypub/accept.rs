//! Accept activity.    
//!
//!

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

/*
{
  "@context":"https://www.w3.org/ns/activitystreams",
  "id":"https://mas.to/users/seungjin#accepts/follows/",
  "type":"Accept",
  "actor":"https://mas.to/users/seungjin",
  "object": {
      "id":"https://dev.prefer.social/0190f4bf-aad1-7290-ac1f-86333df63b82",
      "type":"Follow",
      "actor":"https://dev.prefer.social/self",
      "object":"https://mas.to/users/seungjin"
   }
}
*/

/// Accept activity struct.  
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Accept(Value);

impl Accept {
    /// resturn Accept object.  
    pub async fn new(actor: String, object: Value) -> APObject<Accept> {
        let uuid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uuid);

        let accept_object = APObject::new(
            id,
            ObjectType::Accept,
            actor,
            None,
            None,
            None,
            Accept(object),
        );

        follow_object
    }

    /// when follow action received at inbox.  
    pub async fn receive(ap_obj: APObject<Value>) -> Result<APObject<Self>> {
        tracing::debug!("{:?}", ap_obj);

        tracing::debug!(ap_obj.actor);
        tracing::debug!("{}", ap_obj.object.as_str().unwrap().to_string());

        let subj = ActorUrl::new(ap_obj.actor.clone()).unwrap();
        let obj = ActorUrl::new(ap_obj.object.as_str().unwrap().to_string())
            .unwrap();
        let obj_id = ap_obj.id;

        let subj_account = MAccount::get(subj).await?;
        let subj_account_id = subj_account.uid;

        let obj_account = MAccount::get(obj).await?;
        let obj_account_id = obj_account.uid;

        tracing::debug!("{} follows {}", obj_account_id, subj_account_id);
        MFollow::new(obj_id.clone(), subj_account_id, obj_account_id).await?;

        //let a2 = serde_json::from_value(ap_obj).unwrap();

        // let follow_object = APObject::new(
        //     id,
        //     ObjectType::Follow,
        //     actor,
        //     None,
        //     None,
        //     None,
        //     Follow(object),
        // );

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
