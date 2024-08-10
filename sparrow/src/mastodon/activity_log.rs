use anyhow::Result;
use serde_json::Value;

use crate::table::activity_log::ActivityLog as TActivityLog;

pub struct ActivityLog {}

impl ActivityLog {
    pub async fn get_with_id(id: &str) -> Result<Option<Value>> {
        TActivityLog::get_with_id(id).await
    }
}
