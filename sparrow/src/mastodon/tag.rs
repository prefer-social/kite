// https://docs.joinmastodon.org/entities/Tag/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    name: String,
    url: String,
    history: Vec<HashMap<String, String>>,
    following: Option<bool>,
}
