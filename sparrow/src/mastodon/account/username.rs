use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct Username(pub String);

impl From<String> for Username {
    fn from(i: String) -> Self {
        Username(i)
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
