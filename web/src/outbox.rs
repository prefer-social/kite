// https://kite.seungjin.net/users/seungjin/outbox
// https://kite.seungjin.net/users/seungjin/outbox
use activitystreams::{collection::OrderedCollection, context, iri, object::ApObject, prelude::*};
use anyhow::Result;
use spin_sdk::{
    http::{responses, IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, Value},
};

use crate::utils::not_found;

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

    let mut outbox = ApObject::new(OrderedCollection::new());
    outbox.set_content(context().to_string());
    outbox.set_id(iri!(id));
    outbox.set_first(first);
    outbox.set_last(last);
    outbox.set_total_items(total_items);
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
