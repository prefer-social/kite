// https://kite.seungjin.net/users/seungjin/outbox
// https://kite.seungjin.net/users/seungjin/outbox


//use activitystreams::{collection::OrderedCollection, context, iri, object::ApObject, prelude::*};

use anyhow::Result;
use spin_sdk::{
    http::{responses, IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, Value},
};
use serde_derive::{Deserialize, Serialize};
use crate::utils::not_found;



// {
//     "@context": "https://www.w3.org/ns/activitystreams",
//     "id": "https://mas.to/users/seungjin/outbox",
//     "type": "OrderedCollection",
//     "totalItems": 0,
//     "first": "https://mas.to/users/seungjin/outbox?page=true",
//     "last": "https://mas.to/users/seungjin/outbox?min_id=0&page=true"
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboxActor {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub total_items: u32, 
    pub first: Option<String>, 
    pub last: Option<String>,
}

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get(_req: Request, params: Params) -> Result<Response> {
    let name = params.get("user").unwrap().to_string();
    let query_params = [Value::Text(name.to_string())];
    let connection = Connection::open_default()?;
    let rowset = connection.execute(
        "SELECT * FROM users WHERE users.name = ?",
        query_params.as_slice(),
    )?;
    let user = rowset.rows().next().unwrap();
    let id: &str = user.get("outbox").unwrap();
    let first = format!("{id}?page=true");
    let last = format!("{id}?min_id=0&page=true");
    let total_items: u32 = 0;

    let outbox = OutboxActor {
        context: vec!("arsars".to_string()),
        id: id.to_string(), 
        kind: "OrderedCollection".to_string(),
        total_items: total_items, 
        first: Some(first), 
        last: Some(last),
    };

    let s = serde_json::to_string(&outbox)?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(s.to_owned())
        .build())
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    not_found(req, params).await
}
