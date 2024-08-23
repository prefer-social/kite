//! Represents a list of some users that the authenticated user follows.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/List/>

use serde::{Deserialize, Serialize};

/// Represents a list of some users that the authenticated user follows.
/// Mastodon doc: <https://docs.joinmastodon.org/entities/List/>
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct List {
    /// unique(uuid v7) of this list.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    uid: String,
    /// The user-defined title of the list.
    title: String,
    /// Which replies should be shown in the list
    /// followed = Show replies to any followed user
    /// list = Show replies to members of the list
    /// none = Show replies to no one
    replies_policy: String,
}
