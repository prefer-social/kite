//! Undo activity

use std::fmt;
use std::fmt::Debug;

use anyhow::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::activitystream::activity::ActivityType;
use crate::activitystream::activity::Execute;
use crate::mastodon::follow::Follow;

/*
{
  "@context":"https://www.w3.org/ns/activitystreams",
  "id":"https://mas.to/users/seungjin#follows/6620256/undo",
  "type":"Undo",
  "actor":"https://mas.to/users/seungjin",
  "object":{
    "id":"https://mas.to/0614bc2a-9db6-463d-b23b-772fca54b47b",
    "type":"Follow",
    "actor":"https://mas.to/users/seungjin",
    "object":"https://dev.prefer.social/self"
  }
}
*/

/// Accept activity struct.  
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct Undo(Value);

impl fmt::Display for Undo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl fmt::Debug for Undo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl Execute for Undo {
    async fn execute(&self, activity_val: Value) -> Result<()> {
        let a: &str = activity_val
            .get("object")
            .unwrap()
            .get("type")
            .unwrap()
            .as_str()
            .unwrap();
        let activity_type = ActivityType::from_str(a).unwrap();

        match activity_type {
            ActivityType::Follow => undo_follow(activity_val).await,
            unknown_type => unknown(unknown_type).await,
        }
    }
}

async fn undo_follow(activity: Value) -> Result<()> {
    tracing::debug!("Undo follow");

    let a = serde_json::to_string(&activity).unwrap();
    tracing::debug!("{:?}", a);

    let actor = activity.get("actor").unwrap();
    let follow_object = activity.get("object").unwrap();
    let follow_object_id = follow_object.get("id").unwrap();

    if actor.to_owned() == follow_object.get("actor").unwrap().to_owned() {
        return Follow::undo(follow_object_id.to_string()).await;
    }

    Err(Error::msg("Something wrong when undo follower"))
}

async fn unknown(unknown_type: ActivityType) -> Result<()> {
    tracing::error!("Create '{:?}' is not implemented!", unknown_type);
    Err(Error::msg(format!(
        "Create '{:?}' is not implemented!",
        unknown_type
    )))
}
