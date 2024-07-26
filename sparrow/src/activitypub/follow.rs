pub mod follower;
pub mod following;

use serde_derive::{Deserialize, Serialize};

use crate::activitypub::person_actor::PersonActor;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Follow {
    #[serde(default = "default_context")]
    pub context: String,
    pub id: String,
    #[serde(default = "follow")]
    pub kind: String,
    pub actor: PersonActor,
    pub object: PersonActor,
}

impl Follow {
    pub fn new(id: String, actor: PersonActor, object: PersonActor) -> Self {
        Follow {
            context: default_context(),
            id,
            kind: "Follow".to_string(),
            actor,
            object,
            ..Default::default()
        }
    }
}

fn default_context() -> String {
    "https://www.w3.org/ns/activitystreams".to_string()
}

fn follow() -> String {
    "Follow".to_string()
}
