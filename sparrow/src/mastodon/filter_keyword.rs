//! Represents a keyword that, if matched, should cause the filter action to be taken.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/FilterKeyword/>

use serde::{Deserialize, Serialize};

/// Represents a keyword that, if matched, should cause the filter action to be taken.  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/FilterKeyword/>
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct FilterKeyword {
    /// The ID(uuid v7) of the FilterKeyword in the database.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    uid: String,
    /// The phrase to be matched against.
    keyword: String,
    /// Should the filter consider word boundaries?
    whole_word: bool,
}
