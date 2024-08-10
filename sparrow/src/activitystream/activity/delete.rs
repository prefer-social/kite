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
use crate::mastodon;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::account::Remove as _;
use crate::mastodon::follow::Follow as MFollow;
use crate::mastodon::setting::Setting;

const MAX_RETRY: usize = 8;

/*

{ "@context":["https://www.w3.org/ns/activitystreams",
            {"ostatus":"http://ostatus.org#","atomUri":"ostatus:atomUri"}],
  "id":"https://mstd.seungjin.net/users/wsj/statuses/112796333484021547#delete",
  "type":"Delete","actor":"https://mstd.seungjin.net/users/wsj",
  "to":["https://www.w3.org/ns/activitystreams#Public"],
  "object": {
    "id": "https://mstd.seungjin.net/users/wsj/statuses/112796333484021547",
    "type": "Tombstone",
    "atomUri":"https://mstd.seungjin.net/users/wsj/statuses/112796333484021547"
  },
  "signature":{
    "type":"RsaSignature2017",
    "creator":"https://mstd.seungjin.net/users/wsj#main-key",
    "created":"2024-07-30T13:11:27Z",
    "signatureValue":"hRwSe7dubUEMS82L1sD9K7P4FA5+kaOXiV3+YE3Jl7TFHqZsKbMxYmT/b9Hnsb1US28+wcsMDFwAlVsYynZZoVSgdgG+UbNgULdTXAnOMhGlEUnb1pKPFHgwzNpglpHVKTteK+IexHXL0QRrTDiCTeCiKDHDalUb6nUE5P77SwDSVtwuezeKbj2QOBMv9CfJMMx+kAHQpNaGjg71GT+kc6HTirQqRhox2FPWyLxw/G5gEVBmPz2T8gXncpZ8GeevapVFaUZwn00ArNELUiT7x+79CthXNn24vkilPxD6/YLv957p4qXxVdZLqA/BXHnNk9k6+NulM/X6GUx3wnv0pA=="
  }
}


{ "@context":"https://www.w3.org/ns/activitystreams",
  "id":"https://mas.to/users/astoriacrimelife#delete",
  "type":"Delete",
  "actor":"https://mas.to/users/astoriacrimelife",
  "to":["https://www.w3.org/ns/activitystreams#Public"],
  "object":"https://mas.to/users/astoriacrimelife",
  "signature":{
    "type":"RsaSignature2017",
    "creator":"https://mas.to/users/astoriacrimelife#main-key",
    "created":"2024-07-28T13:29:25Z",
    "signatureValue":"VGxj/ke0+9UrSU42ILLUrgXMXqhji2AWm0RMDeyaMHpsgoYHar9fKnEcWxQUuRobFAihfR8v179ZBPWu9UZmL0UCOViWVtfkgaJckfnXaPwIr+Nt7kg8HdeMl9MmcSh7ZWh9kFG0gwA/upcKpt+4KORVeYB5lpS9pcEV8w2EqBSgnh007M+6rq6nN0aU2vHKdXq6S1+Es5U+DAcebATCtl/KzuensjVrKdTf8sCQ1tcTxZYLt3DvPInjrixd32dTWLfIABFmTx/cekHG0hPzABo5XPLEBYKSKsJHzJe0oCLtzziOjIjH0a8u7eMjuxBRFHd0MjmujSxDjqjTms3lCg=="
  }
}
*/

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
            context: default_context(),
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

                // let request = RequestBuilder::new(Method::Get, url)
                //     .header("Content-Type", "application/activity+json")
                //     .build();
                // let response: Response =
                //     spin_sdk::http::send(request).await.unwrap();

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
