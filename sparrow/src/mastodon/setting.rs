use anyhow::Result;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::table::setting::Get;
use crate::table::setting::Setting as TSetting;

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
    pub async fn get(key: &str) -> Option<String> {
        match TSetting::get(("var".to_string(), key.to_string())).await {
            Ok(s) => match s.last().to_owned() {
                Some(s) => s.to_owned().value,
                None => None,
            },
            Err(e) => None,
        }
    }

    pub async fn domain() -> String {
        Self::get("site_domain").await.unwrap()
    }
}
