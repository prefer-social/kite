use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptedActivity {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    pub object: Value,
}

impl AcceptedActivity {
    pub fn kind(&self) -> Option<AcceptedTypes> {
        match self.kind.as_str() {
            "Accept" => Some(AcceptedTypes::Accept),
            "Announce" => Some(AcceptedTypes::Announce),
            "Create" => Some(AcceptedTypes::Create),
            "Delete" => Some(AcceptedTypes::Delete),
            "Follow" => Some(AcceptedTypes::Follow),
            "Reject" => Some(AcceptedTypes::Reject),
            "Update" => Some(AcceptedTypes::Update),
            "Undo" => Some(AcceptedTypes::Undo),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum AcceptedTypes {
    Accept,
    Announce,
    Create,
    Delete,
    Follow,
    Reject,
    Update,
    Undo,
}
