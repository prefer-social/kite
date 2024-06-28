// https://docs.joinmastodon.org/entities/FilterKeyword/

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct FilterKeyword {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    uid: String,
    keyword: String,
    whole_word: bool,
}

