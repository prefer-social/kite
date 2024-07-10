//! Represents a file or media attachment that can be added to a status.  
//!
//! <https://docs.joinmastodon.org/entities/MediaAttachment/>  

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MediaAttachement: Represents a file or media attachment that can be added to a status.
/// <https://docs.joinmastodon.org/entities/MediaAttachment/>
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct MediaAttachment {
    /// rowid from sqlite
    pub rowid: i64,
    /// uid: uuid v7
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub uid: String,
    /// The type of the attachment.
    /// String (Enumerable, oneOf)
    /// unknown = unsupported or unrecognized file type
    /// image = Static image
    /// gifv = Looping, soundless animation
    /// video = Video clip
    /// audio = Audio track
    pub media_type: String,
    /// he location of the original full-size attachment.
    pub url: String,
    /// The location of a scaled-down preview of the attachment.
    pub preview_url: String,
    /// The location of the full-size original attachment on the remote website.
    pub remote_url: String,
    /// Metadata returned by Paperclip.
    pub meta: HashMap<String, String>,
    /// Alternate text that describes what is in the media attachment, to be used for the visually impaired or when media attachments do not load.
    pub description: String,
    /// A hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when media has not been downloaded yet.
    pub blurhash: String,
    /// A shorter URL for the attachment.
    pub text_url: String,
}

impl MediaAttachment {
    // pub async fn create<'b>(id: &str) -> Self {
    //     let qr: QueryResult = DbCon::builder()
    //         .await
    //         .execute(
    //             "SELECT * FROM media_attachement WHERE id = ?",
    //             &[SV::Text(id.to_string())],
    //         )
    //         .await;
    //     let row = &qr.rows().next().unwrap();

    //     pub async fn foo(a: Option<&str>) -> Option<String> {
    //         match a {
    //             None => return None,
    //             Some(a) => return Some(a.to_string()),
    //         }
    //     }

    //     pub async fn foo2(a: Option<&str>) -> Option<Value> {
    //         match a {
    //             None => return None,
    //             Some(a) => return Some(serde_json::from_str(a).unwrap()),
    //         }
    //     }

    //     let ma = Self {
    //         id: row.get::<&str>("id").unwrap().to_string(),
    //         kind: MediaType::set(row.get::<&str>("type").unwrap()),
    //         url: row.get::<&str>("url").map(|s| s.to_string()),
    //         preview_url: row.get::<&str>("preview_url").map(|s| s.to_string()),
    //         remote_url: row.get::<&str>("remote_url").map(|s| s.to_string()),
    //         text_url: row.get::<&str>("text_url").map(|s| s.to_string()),
    //         meta: foo2(row.get::<&str>("meta")).await,
    //         description: foo(row.get::<&str>("description")).await,
    //         blurhash: row.get::<&str>("blurhash").map(|s| s.to_string()),
    //         created_at: row.get::<&str>("created_at").map(|s| s.to_string()),
    //         updated_at: row.get::<&str>("updated_at").map(|s| s.to_string()),
    //     };

    //     return ma;
    // }
}

/// MediaType used at MediaAttachment.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    /// unsupported or unrecognized file type
    Unknown,
    /// Static image
    Image,
    /// Looping, soundless animation
    Gifv,
    /// Video clip
    Video,
    /// Audio track
    Audio,
}

impl MediaType {
    /// Set MediaType from table's string value
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
