//https://docs.joinmastodon.org/entities/Account/#source

use serde::{Deserialize, Serialize};

use crate::mastodon::account::field::Field;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Source {
    pub privacy: String,
    pub sensitive: bool,
    pub language: String,
    pub note: String,
    pub fields: Vec<Field>,
}
