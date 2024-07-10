// https://docs.joinmastodon.org/entities/PreviewCard/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct PreviewCard {
    pub url: String,
    pub title: String,
    pub description: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub card_type: CardType,
    pub author_name: String,
    pub author_url: String,
    pub provider_name: String,
    pub provider_url: String,
    pub html: String,
    pub width: i64,
    pub height: i64,
    pub image: String,
    pub embed_url: String,
    pub blurhash: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub enum CardType {
    #[default]
    Link,
    Photo,
    Video,
    Rich,
}
