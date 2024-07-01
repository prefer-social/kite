// https://docs.joinmastodon.org/entities/Notification/

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct Notification {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    pub activity_id: String,
    pub activity_type: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub account_id: String,
    pub from_account_id: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub notification_type: String,
}
