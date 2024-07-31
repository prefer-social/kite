//! Describes an object of any kind. The Object type serves as the base type for most of the other kinds of objects defined in the Activity Vocabulary, including other Core types such as Activity, IntransitiveActivity, Collection and OrderedCollection.
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-object>

use serde_json::Value;

pub mod note;

pub enum ObjectType {
    Article,
    Document,
    Event,
    Note,
    Place,
    Profile,
    Relationship,
    Tombstone,
}

pub struct Object {
    context: Value,
    object_type: String,
    id: String,
    name: String,
    attachment: String,
    attributedTo: String,
    audience: Option<String>,
    content: String,
    endTime: String,
    generator: String,
    icon: String,
    image: String,
    inReplyTo: String,
    location: String,
    preview: String,
    published: String,
    replies: String,
    startTime: String,
    summary: String,
    tag: String,
    updated: String,
    url: String,
    to: String,
    bto: String,
    cc: String,
    bcc: String,
    mediaType: String,
    duration: String,
}

impl Object {
    pub fn new() -> Self {
        todo!()
    }
}
