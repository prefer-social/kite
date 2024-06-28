use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct Uid(pub String);
impl From<String> for Uid {
    fn from(i: String) -> Self {
        Uid(i)
    }
}

impl Into<String> for Uid {
    fn into(self) -> String {
        self.0
    }
}
impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
