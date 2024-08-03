//! Describes an object of any kind. The Object type serves as the base type for most of the other kinds of objects defined in the Activity Vocabulary, including other Core types such as Activity, IntransitiveActivity, Collection and OrderedCollection.
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-object>

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::str::FromStr;

pub mod note;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum ObjectType {
    Article,
    Document,
    Event,
    Note,
    Place,
    Profile,
    Relationship,
    Tombstone,
    #[default]
    NotDefined,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for ObjectType {
    type Err = ();
    fn from_str(input: &str) -> Result<ObjectType, Self::Err> {
        match input {
            "Article" => Ok(ObjectType::Article),
            "Document" => Ok(ObjectType::Document),
            "Event" => Ok(ObjectType::Event),
            "Note" => Ok(ObjectType::Note),
            "Place" => Ok(ObjectType::Place),
            "Profile" => Ok(ObjectType::Profile),
            "Relationship" => Ok(ObjectType::Relationship),
            "Tombstone" => Ok(ObjectType::Tombstone),
            _ => Ok(ObjectType::NotDefined),
        }
    }
}
