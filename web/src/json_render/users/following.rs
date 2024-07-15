// https://kite.seungjin.net/users/seungjin/following

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value as SV},
};
use tracing::debug;
use url::Url;

use crate::utils::not_found;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => not_found(req, params).await,
    }
}

// GET /users/:name:/following
// GET /following
pub async fn get(req: Request, params: Params) -> Result<Response> {
    let mut name = match params.get("user") {
        Some(name) => name.to_string(),
        None => {
            tracing::debug!("{}", req.uri());
            let u: Url = req.uri().parse().unwrap();
            let host = u.host_str().unwrap();
            host.split(".").next().unwrap().to_string()
        }
    };

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

    let following_actor = FollowingActor {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id : id.to_string(), 
        kind: "OrderedCollection".to_string(),
        total_items: following_count,
        first: format!("{}?page=1", id),
    };

    let s = serde_json::to_string(&following_actor)?;

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
                federation_id: Url::parse(
                    row.get::<&str>("federation_id").unwrap(),
                )
                .unwrap(),
            })
            .collect();
        followers
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FollowingActor {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    #[serde(rename = "@type")]
    kind: String, 
    total_items: u32, 
    first: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FollowingPageActor {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    #[serde(rename = "@type")]
    kind: String, 
    total_items: u32, 
    part_of: String, 
    ordered_items: Option<Vec<String>>,
}

