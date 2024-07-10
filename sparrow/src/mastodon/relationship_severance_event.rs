//! Summary of a moderation or block event that caused follow relationships to be severed.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/RelationshipSeveranceEvent/>

use serde::{Deserialize, Serialize};

/// Summary of a moderation or block event that caused follow relationships to be severed.  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/RelationshipSeveranceEvent/>
#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct RelationshipSeveranceEvent {
    /// The UID(uuid v7) of the relationship severance event in the database.
    pub uid: String,
    /// Type of event.
    /// String (Enumerable oneOf)
    /// `domain_block` = A moderator suspended a whole domain
    /// `user_domain_block` = The user blocked a whole domain
    /// `account_suspension` = A moderator suspended a specific account
    pub rse_type: String,
    /// hether the list of severed relationships is unavailable because the underlying issue has been purged.
    pub purged: bool,
    /// Name of the target of the moderation/block event. This is either a domain name or a user handle, depending on the event type.
    pub target_name: String,
    /// Number of follow relationships (in either direction) that were severed.
    pub relationships_count: Option<i64>,
    /// When the event took place.
    pub created_at: String,
}
