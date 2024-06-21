use crate::table::account::Account;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(
    Clone, Debug, Deserialize, Serialize, PartialEq, Default, sqlx::FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    pub rowid: i64,
    pub var: String,
    pub value: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub thing_id: Option<i64>,
}

impl Setting {
    pub async fn all() -> Result<HashMap<String, String>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let s: Vec<Setting> =
            sqlx::query_as("SELECT rowid, * FROM setting")
                .fetch_all(&sqlx_conn)
                .await?;
        let mut settings = HashMap::new();
        for u in s.iter() {
            settings.insert(u.var.to_owned(), u.value.to_owned().unwrap());
        }
        Ok(settings)
    }
}
