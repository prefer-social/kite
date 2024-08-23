//! ActivityPub's Follow Object.  

use std::fmt;
use std::fmt::Debug;

use anyhow::{Error, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use uuid::Uuid;

use crate::activitystream::activity::accept::Accept;
use crate::activitystream::activity::Activity;
use crate::activitystream::activity::ActivityType;
use crate::activitystream::default_context;
use crate::activitystream::remove_context;
use crate::activitystream::Execute;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::follow::Follow as MFollow;
use crate::mastodon::setting::Setting;
use crate::mastodon::ME_ACCOUNT;

/// Follow actor object.  
#[derive(Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Follow(pub String);

impl Serialize for Follow {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(self.0.as_str())
    }
}

impl Follow {
    /// resturn Follow object.  
    pub async fn new<T>(actor: String, object: String) -> Activity<Follow>
    where
        T: Debug + Serialize + ToString + Execute,
    {
        let uid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uid);
        let follow = Follow(object);

        let follow_object = Activity::new(
            true,
            id,
            ActivityType::Follow,
            actor,
            None,
            None,
            None,
            follow,
        );

        follow_object
    }

    /// when follow action received at inbox.
    pub async fn parse<T>(
        activity: Activity<Follow>,
    ) -> Result<Activity<Self>> {
        let subj = ActorUrl::new(activity.actor.clone()).unwrap();
        let obj = ActorUrl::new(activity.activity_object.to_string()).unwrap();
        let obj_id = obj.to_string();

        let subj_account = MAccount::get(subj).await?;
        let subj_account_id = subj_account.uid;

        let obj_account = MAccount::get(obj).await?;
        let obj_account_id = obj_account.uid;

        MFollow::new(obj_id.clone(), subj_account_id, obj_account_id).await?;

        let follow_activity = Activity {
            context: Some(default_context()),
            id: obj_id,
            activity_type: ActivityType::Follow,
            actor: activity.actor,
            published: None,
            to: None,
            cc: None,
            activity_object: Follow(activity.activity_object.to_string()),
        };

        Ok(follow_activity)
    }
}

impl fmt::Display for Follow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl fmt::Debug for Follow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Execute for Follow {
    async fn execute(&self, activity_val: Value) -> Result<()> {
        let v = serde_json::to_value(self).unwrap();
        let actor = activity_val.get("actor").unwrap().as_str().unwrap();

        let activity_id = activity_val.get("id").unwrap().to_string();

        let subj_account = ME_ACCOUNT.get().unwrap().to_owned(); // Todo: Error checking
        let subj_account_id = subj_account.uid.to_owned();

        let obj = ActorUrl::new(self.0.to_string()).unwrap();
        let obj_id = obj.to_string();
        let obj_account = MAccount::get(obj).await?;
        let obj_account_id = obj_account.uid;

        MFollow::new(
            //obj_id.clone(),
            activity_id,
            subj_account_id.to_owned(),
            obj_account_id.to_owned(),
        )
        .await?;

        // Now send back "accept" signal to follower.

        let published = Some(Utc::now());
        let my_actor_url = obj_id;
        let accept = Accept(remove_context(activity_val.to_owned()));
        let to = None;
        let cc = None;
        let accept_activity = Activity::new(
            true,
            format!(
                "https://{}#accepts/follows/{}",
                my_actor_url, subj_account_id
            ),
            ActivityType::Accept,
            my_actor_url,
            published,
            to,
            cc,
            accept,
        );

        let res =
            crate::mastodon::post_activity(subj_account, accept_activity)
                .await?;
        match res {
            202u16 => {
                tracing::debug!("Activity published({})", res);
                Ok(())
            }
            _ => {
                tracing::error!("Activity published({})", res);
                Err(Error::msg(format!("Published Activity received {}", res)))
            }
        }

        // tracing::debug!("{:?}", accept_activity);
        // match accept_activity.activity_object.execute(activity_val).await {
        //     Ok(_) => return Ok(()),
        //     Err(e) => {
        //         tracing::error!("Resending Accept signal error: {:?}", e);
        //         tracing::error!("{:?}", accept_activity);
        //         return Err(Error::msg("{e:?}"));
        //     }
        // }
    }
}
