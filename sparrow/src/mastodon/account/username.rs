use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Eq,
    PartialEq,
    Encode,
    Decode,
)]
pub struct Username(pub String);

impl From<String> for Username {
    fn from(i: String) -> Self {
        Username(i.to_lowercase())
    }
}
impl Into<String> for Username {
    fn into(self) -> String {
        self.0
    }
}
impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
