//! Represents a filter whose keywords matched a given status.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/FilterResult/>

use serde::{Deserialize, Serialize};

use crate::mastodon::filter::Filter;

/// Represents a filter whose keywords matched a given status.  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/FilterResult/>
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FilterResult {
    /// The filter that was matched.
    pub filter: Filter,
    /// The keyword within the filter that was matched.
    /// Nullable, Array of String, or null
    pub keyword_matches: Vec<String>,
    /// The status ID within the filter that was matched.
    /// Nullable, Array of String, or null
    pub status_matches: Vec<String>,
}
