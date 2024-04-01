// https://kite.seungjin.net/users/seungjin/following

use activitystreams::{collection::OrderedCollection, context, iri, object::ApObject, prelude::*};
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value as SV},
};
use tracing::debug;
use url::Url;

use crate::utils::not_found;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => not_found(req, params).await,
    }
}

// GET /users/:name:/followers
pub async fn get(_req: Request, params: Params) -> Result<Response> {
    let name = params.get("user").unwrap().to_string();
    let query_params = [SV::Text(name.to_string())];

    let following_count_qr = sparrow::db::Connection::builder().await.execute("SELECT user.following AS id, count(following.id) AS A FROM user FULL JOIN following ON user.id = following.userId where user.name = ?", &[SV::Text(name)]).await;
    let following_count = following_count_qr
        .rows()
        .next()
        .unwrap()
        .get::<u32>("A")
        .unwrap();

    let id = following_count_qr
        .rows()
        .next()
        .unwrap()
        .get::<&str>("id")
        .unwrap();

    let mut f = ApObject::new(OrderedCollection::new());
    f.set_content(context().to_string());
    f.set_id(iri!(id));
    f.set_total_items(following_count);
    f.set_first(format!("{}?page=1", id));

    let s = serde_json::to_string(&f)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(s)
        .build())
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Following {
    id: u32,
    user_id: u32,
    federation_id: Url,
}

struct Followings;

impl Following {
    pub fn build(rowset: QueryResult) -> Vec<Following> {
        let followers: Vec<Following> = rowset
            .rows()
            .map(|row| Following {
                id: row.get::<u32>("id").unwrap(),
                user_id: row.get::<u32>("user_id").unwrap(),
                federation_id: Url::parse(row.get::<&str>("federation_id").unwrap()).unwrap(),
            })
            .collect();
        followers
    }
}
