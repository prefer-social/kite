// https://docs.joinmastodon.org/entities/List/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct List {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    id: String,
    title: String,
    replies_policy: String,
}
