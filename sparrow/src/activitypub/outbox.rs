//! ActivityPub Outbox  
//!
//! Kind: OrderedCollection
//!

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbox {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub outbox_type: String,
    pub total_items: i64,
    pub first: String,
    pub last: String,
}

// impl TryInto<Value> for Outbox {
//     type Error = &'static str;

//     fn try_into(
//         Value,
//     ) -> Result<Value> {
//         let current_epoch = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_secs() as i64;

//         tracing::debug!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
//         tracing::debug!("{:?}", actor);

//         let avatar_remote_url = match &actor.icon {
//             Some(i) => Some(i.to_owned().url),
//             None => None,
//         };

//         let avatar_content_type = match &actor.icon {
//             Some(i) => Some(i.to_owned().media_type),
//             None => None,
//         };
//         let header_remote_url = match &actor.image {
//             Some(i) => Some(i.to_owned().url),
//             None => None,
//         };
//         let header_content_type = match &actor.image {
//             Some(i) => Some(i.to_owned().media_type),
//             None => None,
//         };

//         let account = Account {
//             uid: uuid::Uuid::now_v7().to_string(),
//             username: actor.preferred_username,
//             domain: Some(
//                 Url::parse(actor.id.as_str())
//                     .unwrap()
//                     .domain()
//                     .unwrap()
//                     .to_string(),
//             ),
//             public_key: actor.public_key.public_key_pem,
//             created_at: current_epoch, // not null
//             updated_at: current_epoch, // not null
//             note: actor.summary,       // default(""), not null
//             display_name: actor.name,  // default(""), not null
//             uri: actor.id,             // default(""), not null
//             url: Some(actor.url),
//             avatar_content_type: avatar_content_type,
//             header_content_type: header_content_type,
//             avatar_remote_url: avatar_remote_url,
//             header_remote_url: header_remote_url,
//             last_webfingered_at: Some(current_epoch),
//             inbox_url: Some(actor.inbox),
//             outbox_url: Some(actor.outbox),
//             shared_inbox_url: Some(actor.endpoints.shared_inbox), // default(""), not null
//             following_url: Some(actor.following),
//             followers_url: Some(actor.followers), // default(""), not null
//             memorial: actor.memorial,
//             featured_collection_url: Some(actor.featured),
//             actor_type: Some(actor.actor_type),
//             discoverable: Some(actor.discoverable),
//             devices_url: actor.devices,
//             indexable: Some(actor.indexable),
//             ..Default::default() // default(FALSE), not null
//         };
//         Ok(account)
//     }
// }
