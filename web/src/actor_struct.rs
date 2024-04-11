use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{QueryResult, Value as SV},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    #[serde(rename = "@context")]
    pub context: Vec<Value>,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    pub featured: String,
    pub featured_tags: String,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    pub memorial: bool,
    pub devices: String,
    pub public_key: PublicKey,
}

pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

impl Actor {
    async fn build(username: String) -> Option<Actor> {
        let user_rowset = sparrow::db::Connection::builder()
            .await
            .execute(
                "SELECT * FROM user WHERE name = ?",
                &[SV::Text(username.to_string())],
            )
            .await;

        if user_rowset.rows().count() == 0 {
            return None;
        }

        let users = Users::build(user_rowset);
        let user = users.get(0).unwrap();

        let signing_key_rowset = sparrow::db::Connection::builder()
            .await
            .execute(
                "SELECT publicKey FROM signing_key WHERE userId = ?",
                &[SV::Integer(user.id as i64)],
            )
            .await;
        let public_key_pem = signing_key_rowset
            .rows()
            .next()
            .unwrap()
            .get::<&str>("publicKey")
            .unwrap();
    }
}
