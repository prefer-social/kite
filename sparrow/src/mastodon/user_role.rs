use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::mastodon::user::User;
use crate::table::user_role::Get;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct UserRole {
    pub uid: String,
    pub name: String,
    pub permissions: i64,
    pub color: String,
    pub highlighted: bool,
}

impl UserRole {
    pub async fn get(user: User) -> Result<Option<UserRole>> {
        let role_id = user.role_id.unwrap();

        let role_tbl = crate::table::user_role::UserRole::get((
            "uid".to_string(),
            role_id,
        ))
        .await?;

        if role_tbl == None {
            return Ok(None);
        };

        let a = role_tbl.unwrap();
        let user_role = UserRole {
            uid: a.uid,
            name: a.name,
            permissions: a.permissions,
            color: a.color,
            highlighted: a.highlighted,
        };

        Ok(Some(user_role))
    }
}
