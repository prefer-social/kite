// https://docs.joinmastodon.org/entities/Tag/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    name: String,
    url: String,
    history: Vec<HashMap<String, String>>,
    following: Option<bool>,
}

impl Tag {
    pub async fn search(search_term: &String) -> Result<Vec<Tag>> {
        Ok(Vec::new())
    }
}
