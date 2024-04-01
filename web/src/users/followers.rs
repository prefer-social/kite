// https://kite.seungjin.net/users/seungjin/following

use activitystreams::{collection::OrderedCollection, context, iri, object::ApObject, prelude::*};
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value as SV},
};
use tracing::{debug, info};
use url::Url;

use crate::utils::{get_req_query_hash, not_found};

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    let query_map = get_req_query_hash(&req).await;
    match query_map.get("page") {
        Some(n) => {
            return get_follower_page(req, params).await;
        }
        None => {
            return get_follower(req, params).await;
        }
    }
}

pub async fn get_follower(_req: Request, params: Params) -> Result<Response> {
    let name = params.get("user").unwrap().to_string();

    let follower_count_qr = sparrow::db::Connection::builder().await.execute("SELECT user.followers AS fid, count(follower.id) AS A FROM user FULL JOIN follower ON user.id = follower.userId where user.name = ?", &[SV::Text(name)]).await;
    let follower_count = follower_count_qr
        .rows()
        .next()
        .unwrap()
        .get::<u32>("A")
        .unwrap();

    let fid = follower_count_qr
        .rows()
        .next()
        .unwrap()
        .get::<&str>("fid")
        .unwrap()
        .to_string();

    let mut f = ApObject::new(OrderedCollection::new());
    f.set_content(context().to_string());
    f.set_id(iri!(fid));
    f.set_total_items(follower_count);
    f.set_first(format!("{}?page=1", fid));

    let s = serde_json::to_string(&f)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(s)
        .build())
}

pub async fn get_follower_page(_req: Request, params: Params) -> Result<Response> {
    let name = params.get("user").unwrap().to_string();
    let query_params = [SV::Text(name.to_string())];

    let connection = Connection::open_default()?;
    let rowset = connection.execute(
        "SELECT users.followers AS id, count(followers.federationId) AS c FROM users LEFT JOIN followers ON users.id = followers.id WHERE users.name == ? GROUP BY followers.federationId",
        query_params.as_slice(),
    )?;

    let followers = rowset.rows().next().unwrap();
    let id: &str = followers.get("id").unwrap();
    let total_items: u32 = followers.get("c").unwrap();

    let mut f = ApObject::new(OrderedCollection::new());
    f.set_content(context().to_string());
    f.set_id(iri!(id));
    f.set_total_items(total_items);
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
pub struct Follower {
    id: u32,
    user_id: u32,
    federation_id: Url,
}

struct Followers;

impl Followers {
    pub fn build(rowset: QueryResult) -> Vec<Follower> {
        let followers: Vec<Follower> = rowset
            .rows()
            .map(|row| Follower {
                id: row.get::<u32>("id").unwrap(),
                user_id: row.get::<u32>("user_id").unwrap(),
                federation_id: Url::parse(row.get::<&str>("federation_id").unwrap()).unwrap(),
            })
            .collect();
        followers
    }
}
