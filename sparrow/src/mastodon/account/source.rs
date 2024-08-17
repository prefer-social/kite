//! An extra attribute that contains source values to be used with API methods that verify credentials and update credentials.   
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Account/#source>

use anyhow::Result;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::mastodon::account::field::Field;

/// An extra attribute that contains source values to be used with API methods that verify credentials and update credentials.   
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Account/#source>
#[derive(
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    Encode,
    Decode,
)]
pub struct Source {
    /// The default post privacy to be used for new statuses.
    /// public = Public post
    /// unlisted = Unlisted post
    /// private = Followers-only post
    /// direct = Direct post
    pub privacy: String,
    /// Whether new statuses should be marked sensitive by default.
    pub sensitive: bool,
    /// The default posting language for new statuses.
    pub language: String,
    /// Profile bio, in plain-text instead of in HTML.
    pub note: String,
    /// The default post privacy to be used for new statuses.
    pub fields: Vec<Field>,
}

impl Source {
    pub async fn get() -> Result<Source> {
        let fields = Vec::new();

        Ok(Source {
            privacy: "public".to_string(),
            sensitive: false,
            language: "en".to_string(),
            note: "".to_string(),
            fields: fields,
        })
    }
}
