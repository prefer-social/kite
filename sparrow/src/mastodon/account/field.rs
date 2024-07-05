use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: String,
}
