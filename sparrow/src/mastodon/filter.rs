//! Represents a user-defined filter for determining which statuses should not be shown to the user.  
//!
//! <https://docs.joinmastodon.org/entities/Filter/>

use serde::{Deserialize, Serialize};

use crate::mastodon::filter_keyword::FilterKeyword;
use crate::mastodon::filter_status::FilterStatus;

/// Represents a user-defined filter for determining which statuses should not be shown to the user.  
/// <https://docs.joinmastodon.org/entities/Filter/>
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Filter {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    /// The ID(uuid v7) of the Filter in the database.
    pub uid: String,
    /// A title given by the user to name the filter.
    pub title: String,
    /// The contexts in which the filter should be applied.
    /// Array of String (Enumerable, anyOf)
    /// `home` = home timeline and lists
    /// `notifications` = notifications timeline
    /// `public` = public timelines
    /// `thread` = expanded thread of a detailed status
    /// `account` = when viewing a profile
    pub context: Vec<String>,
    /// When the filter should no longer be applied.
    /// nullable String (ISO 8601 Datetime), or null if the filter does not expire
    pub expires_at: String,
    /// The action to be taken when a status matches this filter.
    /// String (Enumerable, oneOf)
    /// `warn`` = show a warning that identifies the matching filter by title, and allow the user to expand the filtered status. This is the default (and unknown values should be treated as equivalent to warn).
    /// `hide`` = do not show this status if it is received
    pub filter_action: String,
    /// The keywords grouped under this filter.
    pub keywords: Vec<FilterKeyword>,
    /// The statuses grouped under this filter.
    pub statuses: Vec<FilterStatus>,
}
