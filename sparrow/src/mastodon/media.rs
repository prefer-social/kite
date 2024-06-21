use crate::db::Connection as DbCon;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::{QueryResult, Row, Value as SV};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaAttachment {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blurhash: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at: Option<String>,
}

impl MediaAttachment {
    pub async fn create<'b>(id: &str) -> Self {
        let qr: QueryResult = DbCon::builder()
            .await
            .execute(
                "SELECT * FROM media_attachement WHERE id = ?",
                &[SV::Text(id.to_string())],
            )
            .await;
        let row = &qr.rows().next().unwrap();

        pub async fn foo(a: Option<&str>) -> Option<String> {
            match a {
                None => return None,
                Some(a) => return Some(a.to_string()),
            }
        }

        pub async fn foo2(a: Option<&str>) -> Option<Value> {
            match a {
                None => return None,
                Some(a) => return Some(serde_json::from_str(a).unwrap()),
            }
        }

        let ma = Self {
            id: row.get::<&str>("id").unwrap().to_string(),
            kind: MediaType::set(row.get::<&str>("type").unwrap()),
            url: row.get::<&str>("url").map(|s| s.to_string()),
            preview_url: row.get::<&str>("preview_url").map(|s| s.to_string()),
            remote_url: row.get::<&str>("remote_url").map(|s| s.to_string()),
            text_url: row.get::<&str>("text_url").map(|s| s.to_string()),
            meta: foo2(row.get::<&str>("meta")).await,
            description: foo(row.get::<&str>("description")).await,
            blurhash: row.get::<&str>("blurhash").map(|s| s.to_string()),
            created_at: row.get::<&str>("created_at").map(|s| s.to_string()),
            updated_at: row.get::<&str>("updated_at").map(|s| s.to_string()),
        };

        return ma;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Unknown,
    Image,
    Gifv,
    Video,
    Audio,
}

impl MediaType {
    pub fn set(a: &str) -> Self {
        match a.to_lowercase().as_str() {
            "image" => return MediaType::Image,
            "gifv" => return MediaType::Gifv,
            "video" => return MediaType::Video,
            "audio" => return MediaType::Audio,
            _ => return MediaType::Unknown,
        }
    }
}

impl Default for MediaType {
    fn default() -> Self {
        MediaType::Unknown
    }
}
