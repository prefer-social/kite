use anyhow::Result;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use spin_sqlx::sqlite::Connection as dbcon;
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
        let sqlx_conn = dbcon::open_default()?;
        let s: Vec<Setting> = sqlx::query_as("SELECT rowid, * FROM setting")
            .fetch_all(&sqlx_conn)
            .await?;
        let mut settings = HashMap::new();
        for u in s.iter() {
            settings.insert(u.var.to_owned(), u.value.to_owned().unwrap());
        }
        Ok(settings)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Setting>>;
}

#[async_trait]
impl Get<(String, String)> for Setting {
    async fn get((colume, val): (String, String)) -> Result<Vec<Setting>> {
        let query_template =
            format!("SELECT rowid, * FROM setting WHERE {} = ?", colume);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
