//! CustomEmoji struct  
//!
//! Each site can define and upload its own custom emoji to be attached to profiles or statuses.  
//! Mastodon doc: <https://docs.joinmastodon.org/entities/CustomEmoji/>  

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct CustomEmoji {
    pub shortcode: String,
    pub url: String,
    pub static_url: String,
    pub visible_in_picker: bool,
    pub category: String,
}
