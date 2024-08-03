//! A Tombstone represents a content object that has been deleted. It can be used in Collections to signify that there used to be an object at this position, but it has been deleted.
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tombstone>

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

use crate::activitystream::object::ObjectType;
use crate::activitystream::Execute;

/*
{
 "id":"https://mstd.seungjin.net/users/wsj/statuses/112818277812710620",
 "type":"Tombstone",
 "atomUri":"https://mstd.seungjin.net/users/wsj/statuses/112818277812710620"
}
*/

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tombstone {
    id: String,
    tombstone_type: String,
    atom_uri: String,
}

impl fmt::Display for Tombstone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl fmt::Debug for Tombstone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl Execute for Tombstone {
    async fn execute(&self, actor: String) -> Result<()> {
        tracing::debug!("##########################");
        Ok(())
    }
}

todo!();
