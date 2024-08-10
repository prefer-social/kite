//! Represents a notification of an event relevant to the user.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Notification/>

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mastodon::account::Account as MAccount;
use crate::mastodon::relationship_severance_event::RelationshipSeveranceEvent;
use crate::mastodon::report::Report;
use crate::mastodon::status::Status;

/// Represents a notification of an event relevant to the user.
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Notification/>
#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct Notification {
    /// The id(uud v7) of the notification in the database.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    /// The type of event that resulted in the notification.
    /// String (Enumerable oneOf)
    /// `mention` = Someone mentioned you in their status
    /// `status` = Someone you enabled notifications for has posted a status
    /// `reblog` = Someone boosted one of your statuses
    /// `follow` = Someone followed you
    /// `follow_request` = Someone requested to follow you
    /// `favourite` = Someone favourited one of your statuses
    /// `poll` = A poll you have voted in or created has ended
    /// `update` = A status you interacted with has been edited
    /// `admin.sign_up` = Someone signed up (optionally sent to admins)
    /// `admin.report` = A new report has been filed
    /// `severed_relationships` = Some of your follow relationships have been severed as a result of a moderation or block event
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub notification_type: String,
    /// The timestamp of the notification.
    pub created_at: DateTime<Utc>,
    /// The account that performed the action that generated the notification.
    pub account: MAccount,
    /// Status that was the object of the notification.
    /// Attached when type of the notification is favourite, reblog, status, mention, poll, or update.
    pub status: Option<Status>,
    /// Report that was the object of the notification.
    /// Attached when type of the notification is admin.report.
    pub report: Option<Report>,
    /// Summary of the event that caused follow relationships to be severed.
    /// Attached when type of the notification is severed_relationships.
    pub relationship_severance_event: Option<RelationshipSeveranceEvent>,
}
