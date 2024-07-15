//! Represents a file or media attachment that can be added to a status.  
//!
//! <https://docs.joinmastodon.org/entities/MediaAttachment/>  

use anyhow::Result;
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
    /// For now , it returns empty vector Result.  
    /// TODO: Make it real.  
    pub async fn get(media_ia: String) -> Result<Vec<MediaAttachment>> {
        Ok(Vec::new())
    }
}
