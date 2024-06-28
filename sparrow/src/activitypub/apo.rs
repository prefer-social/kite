// ActivityPub Objects
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

//use crate::mastodon::media::MediaAttachment;
//use crate::mastodon::media::MediaType;

pub enum ObjectTypes {
    Note,
    Create,
    Replies,
    RsaSignature2017,
    OrderedCollection,
    OrderedCollectionPage,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Object<T> {
    #[serde(skip_deserializing)]
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    pub object: T,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    #[serde(skip_deserializing)]
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    pub object: String,
}

impl Follow {
    pub fn get_recipient(self) -> Result<String> {
        Ok(self.object)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Accept {
    #[serde(skip_deserializing)]
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    pub object: Follow,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Create<T> {
    #[serde(skip_deserializing)]
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub actor: String,
    pub published: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub object: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub summary: Option<String>,
    pub in_reply_to: Option<String>,
    pub published: String,
    pub url: String,
    pub attributed_to: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub sensitive: bool,
    pub atom_uri: String,
    pub in_relpay_to_atom_uri: Option<bool>,
    pub converation: String,
    pub content: String,
    pub content_map: Vec<HashMap<String, String>>,
    pub attachment: Vec<MediaAttachment>,
    pub tag: Vec<String>,
    pub replies: Replies,
    // pub signature: RsaSignature2017Object,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Replies {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub first: CollectionPage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(rename = "type")]
    pub kind: String,
    pub next: String,
    pub part_of: String,
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RsaSignature2017 {
    #[serde(rename = "type")]
    pub kind: String,
    pub creator: String,
    pub created: String,
    pub signature_value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(skip_deserializing)]
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub total_items: i64,
    pub first: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(skip_deserializing)]
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub total_items: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    pub part_of: String,
    pub ordered_items: Vec<String>,
}
