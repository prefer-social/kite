// https://docs.joinmastodon.org/entities/MediaAttachment/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct MediaAttachement {
    pub id: i64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub media_type: MediaType,
    pub uid: String,
    pub url: String,
    pub preview_url: String,
    pub remote_url: String,
    pub meta: HashMap<String, String>,
    pub description: String,
    pub blurhash: String,
    pub text_url: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum MediaType {
    #[default]
    Unknown,
    Image,
    Gifv,
    Video,
    Audio,
}
