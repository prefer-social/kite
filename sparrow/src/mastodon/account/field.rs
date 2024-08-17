//! Account Field
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Account field
#[derive(
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    Encode,
    Decode,
)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: String,
}
