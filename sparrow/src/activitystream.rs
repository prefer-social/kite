//! Activity(Pub|Stream) Library
//!
//! ActivityStream vocabulary: <https://www.w3.org/TR/activitystreams-vocabulary/>
//! Mastodon doc about ActivityPub <https://docs.joinmastodon.org/spec/activitypub/>
//!

use serde_json::Value;

pub mod activity;
pub mod actor;
pub mod collection;
pub mod object;
pub mod ordered_collection;

/// Helper function that remove @context key from serde_json::Value object.
pub fn remove_context(mut v: Value) -> Value {
    let a = v.as_object_mut().unwrap();
    a.remove_entry("@context");
    serde_json::to_value(a).unwrap()
}

pub(self) fn default_context() -> Value {
    Value::String("https://www.w3.org/ns/activitystreams".to_string())
}
