use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

#[derive(
    Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::FromRow,
)]
pub struct ActorJson {
    actor_json: String,
    created_at: i64,
    updated_at: i64,
}

impl ActorJson {
    pub async fn put(actor: Value) -> Result<()> {
        let actor_id = actor["id"].as_str().unwrap();
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let actor_json_string = serde_json::to_string(&actor).unwrap();

        let actor_json_rows = sqlx::query(
            "SELECT actor_json FROM actor_json WHERE actor_json ->> '$.id' == ?",
        )
        .bind(actor_id)
        .fetch_all(&sqlx_conn)
        .await?;

        tracing::debug!("{:?}", actor_json_rows.len());

        match actor_json_rows.len() {
            0 => {
                sqlx::query(
                    "INSERT INTO actor_json (uid, actor_json) VALUES(?,?)",
                )
                .bind(uuid::Uuid::now_v7().to_string())
                .bind(actor_json_string)
                .execute(&sqlx_conn)
                .await?;

                Ok(())
            }
            1 => Ok(()),
            _ => Err(anyhow::Error::msg(
                "Duplicated actor_json in actor_json table",
            )),
        }
    }
}