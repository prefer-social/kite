//! Reports filed against users and/or statuses, to be taken action on by moderators.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Report/>

use serde::{Deserialize, Serialize};

use crate::mastodon::account::Account as MAccount;

/// Reports filed against users and/or statuses, to be taken action on by moderators.   
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Report/>
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Report {
    /// The ID(uuid v7) of the report in the database.
    pub uid: String,
    /// Whether an action was taken yet.
    pub action_taken: bool,
    /// When an action was taken against the report.
    /// Nullable, String (ISO 8601 Datetime) or null
    pub action_taken_at: String,
    /// the generic reason for the report.
    /// String (Enumerable oneOf)
    /// `spam` = Unwanted or repetitive content
    /// `violation` = A specific rule was violated
    /// `other` = Some other reason
    pub category: String,
    /// The reason for the report.
    pub comment: String,
    /// Whether the report was forwarded to a remote domain.
    pub forwarded: bool,
    /// When the report was created.
    /// String (ISO 8601 Datetime)
    pub created_at: String,
    /// IDs of statuses that have been attached to this report for additional context.
    /// Nullable, Array of String (cast from integer), or null
    pub status_ids: String,
    /// IDs of the rules that have been cited as a violation by this report.
    /// Nullable, Array of String (cast from integer), or null
    pub rule_ids: String,
    /// The account that was reported.
    pub target_account: MAccount,
}
