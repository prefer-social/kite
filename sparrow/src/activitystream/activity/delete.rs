//! Delete activity.  
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete>

use anyhow::Result;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use spin_sdk::http::{Method, RequestBuilder, Response};
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;
use uuid::Uuid;

use crate::activitystream::activity::Activity;
use crate::activitystream::activity::ActivityType;
use crate::activitystream::default_context;
use crate::activitystream::object::ObjectType;
use crate::activitystream::Execute;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::account::Remove as _;
use crate::mastodon::follow::Follow as MFollow;
use crate::mastodon::setting::Setting;
use crate::mastodon::ACTOR_ACCOUNT;

const MAX_RETRY: usize = 8;

#[derive(Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Delete(Value);

impl Serialize for Delete {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(serde_json::to_string(&self.0).unwrap().as_str())
    }
}

impl Delete {
    /// resturn Follow object.  
    pub async fn new<T>(
        actor: String,
        object: Value,
    ) -> Result<Activity<Delete>>
    where
        T: Debug + Serialize + ToString,
    {
        let delete = Delete(object.clone());

        let uid = Uuid::now_v7().to_string();
        let id = format!("https://{}/{}", Setting::domain().await, uid);
        //let object = serde_json::from_value(obj).unwrap();

        let follow_object = Activity::new(
            true,
            id,
            ActivityType::Delete,
            actor.clone(),
            None,
            None,
            None,
            delete,
        );

        Ok(follow_object)
    }

    /// when follow action received at inbox.
    pub async fn parse<T>(
        activity: Activity<Delete>,
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
            activity_object: activity.activity_object,
        };

        Ok(follow_activity)
    }
}

impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap().as_str())
    }
}

impl fmt::Debug for Delete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap().as_str())
    }
}

impl Execute for Delete {
    async fn execute(&self, activity_val: Value) -> Result<()> {
        // Check object
        let object_type_str = match self.0.is_object() {
            true => self.0.get("type").unwrap().as_str().unwrap(),
            _ => self.0.as_str().unwrap(),
        };

        let object_type = ObjectType::from_str(object_type_str).unwrap();

        match object_type {
            ObjectType::Tombstone => {
                // Get id and delete it from database' status table.
                tracing::warn!("Delete from statuse/tombstone process not yet implemented");
                return Ok(());
            }
            ObjectType::Url(url) => {
                // Todo: Validate URL
                // Delete this url from account.
                let actor =
                    activity_val.get("actor").unwrap().as_str().unwrap();
                let actor_url = ActorUrl::new(actor.to_owned()).unwrap();

                let response =
                    redirect_http_request(url.as_str(), MAX_RETRY).await?;
                if *response.status() == 410u16 {
                    // HTTP 410 is Gone
                    tracing::debug!("Account '{}' is gone.", actor);
                    match MAccount::remove(actor_url).await {
                        Ok(_) => {
                            tracing::debug!(
                                "Account (actor:{}) is removed.",
                                { actor.to_string() }
                            );
                            return Ok(());
                        }
                        Err(e) => {
                            tracing::error!("{e:?}");
                            return Err(e);
                        }
                    }
                } else {
                    tracing::warn!(
                        "ActorUri {} is NOT Gone/401 instead status {}",
                        actor_url.to_string(),
                        *response.status(),
                    );
                    return Ok(());
                };
            }
            not_implemented => {
                tracing::error!("{} is not implemented yet", not_implemented);
                return Ok(());
            }
        };
    }
}

async fn redirect_http_request(
    url: &str,
    max_retry: usize,
) -> Result<Response> {
    tracing::warn!("redirect_http_request! {}", max_retry);
    let request = RequestBuilder::new(Method::Get, url)
        .header("Content-Type", "application/activity+json")
        .build();

    let response: Response = spin_sdk::http::send(request).await.unwrap();
    if max_retry <= 1 {
        tracing::warn!("Number of max retries attemption reached!");
        return Ok(response);
    }

    if vec![301u16, 307u16, 308u16].contains(response.status()) {
        tracing::debug!(
            "Requesting redirected location: {}",
            response.header("Location").unwrap().as_str().unwrap()
        );
        Box::pin(redirect_http_request(
            response.header("Location").unwrap().as_str().unwrap(),
            max_retry - 1,
        ))
        .await?;
    };

    Ok(response)
}
