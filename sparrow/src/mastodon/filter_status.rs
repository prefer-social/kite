//! Represents a status ID that, if matched, should cause the filter action to be taken.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/FilterStatus/>

use serde::{Deserialize, Serialize};

/// Represents a status ID that, if matched, should cause the filter action to be taken.  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/FilterStatus/>
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FilterStatus {
    /// The UID(uuid v7) of the FilterStatus in the database.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    /// The UID(uuid v7) of the Status that will be filtered.
    pub status_id: String,
}
