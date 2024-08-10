//! Accept activity.    
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
use crate::activitystream::object::ObjectType;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::activity_log::ActivityLog;
use crate::mastodon::follow::Follow as MFollow;
<<<<<<< HEAD
use crate::mastodon::publish_activity;
=======
use crate::mastodon::post_activity;
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
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
pub struct Accept(pub Value);

impl Accept {
    /// resturn Accept object.  
    pub async fn new(actor: String, object: Value) -> Activity<Accept> {
        let uuid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uuid);

        let accept_object = Activity::new(
            id,
            ActivityType::Accept,
            actor,
            None,
            None,
            None,
            Accept(object),
        );

        accept_object
    }
}

impl fmt::Display for Accept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl fmt::Debug for Accept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl Execute for Accept {
    async fn execute(&self, activity_val: Value) -> Result<()> {
        tracing::debug!("{:?}", activity_val);

        tracing::debug!("-----------> Accept execute <---------------");

        // Check activiy.object is what I really sent.
        // https://dev.prefer.social/0190fcb0-5272-77c3-acb1-3e9be71ff930
        // SELECT * FROM activity_log WHERE JSON_EXTRACT(body, '$.id') = ?
<<<<<<< HEAD
        match ActivityLog::get_with_id(
            self.0.get("id").unwrap().as_str().unwrap(),
        )
        .await
        .unwrap()
=======
        match ActivityLog::get_with_id(self.0.get("id").unwrap().as_str().unwrap())
            .await
            .unwrap()
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        {
            None => {
                tracing::error!(
                    "Havn't published this acticity {}",
                    self.0.get("id").unwrap().to_string()
                )
            }
            Some(x) => {
                let log_obj = activitystream::remove_context(x);
<<<<<<< HEAD
                let given_obj =
                    activitystream::remove_context(self.0.to_owned());
                if given_obj != log_obj {
                    tracing::error!(
                        "Integration error! No matching follow was published! {}", self.0.get("id").unwrap().to_string()
=======
                let given_obj = activitystream::remove_context(self.0.to_owned());
                if given_obj != log_obj {
                    tracing::error!(
                        "Integration error! No matching follow was published! {}",
                        self.0.get("id").unwrap().to_string()
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
                    );
                    return Err(anyhow::Error::msg(
                        "Given activity is not published by SELF!",
                    ));
                }
            }
        };

<<<<<<< HEAD
        let subj = ActorUrl::new(
            self.0.get("actor").unwrap().as_str().unwrap().to_string(),
        )
        .unwrap();
        let obj = ActorUrl::new(
            self.0.get("object").unwrap().as_str().unwrap().to_string(),
        )
        .unwrap();
=======
        let subj =
            ActorUrl::new(self.0.get("actor").unwrap().as_str().unwrap().to_string()).unwrap();
        let obj =
            ActorUrl::new(self.0.get("object").unwrap().as_str().unwrap().to_string()).unwrap();
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        let obj_id = self.0.get("id").unwrap().as_str().unwrap().to_string();

        let subj_account = MAccount::get(subj).await?;
        let subj_account_id = subj_account.uid;

        let obj_account = MAccount::get(obj).await?;
        let obj_account_id = obj_account.uid;

        MFollow::new(obj_id, subj_account_id, obj_account_id).await?;

        Ok(())
    }
}
