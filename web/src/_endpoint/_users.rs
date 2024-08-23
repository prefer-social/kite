use anyhow::Result;
use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{QueryResult, Value as SV},
};
use time::OffsetDateTime;
use url::Url;

use crate::utils::not_found;

use sparrow::activitypub::person_actor::PersonActor;
use sparrow::table::user::User;

pub mod activities;
pub mod followers;
pub mod following;
pub mod inbox;
pub mod outbox;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("user info requested");

    // get domain id:
    tracing::debug!("{}", req.uri());

    let mut name: String = match params.get("user") {
        Some(name) => name.to_string(),
        None => {
            let u: Url = req.uri().parse().unwrap();
            let hostname = u.host_str().unwrap();
            let id = hostname.split(".").next().unwrap();
            id.to_string()
        }
    };
    // If parmas has "user" to follow:
    //let mut name = params.get("user").unwrap().to_string();

    name = String::from(if name[..1].to_string() == "@".to_string() {
        name[1..].to_string()
    } else {
        name
    });

    // let user_rowset = sparrow::db::Connection::builder()
    //     .await
    //     .execute(
    //         "SELECT * FROM user WHERE name = ?",
    //         &[SV::Text(name.to_string())],
    //     )
    //     .await;

    // if user_rowset.rows().count() == 0 {
    //     return Ok(Response::builder().status(404).build());
    // }

    // //let users = Users::build(user_rowset);
    // let user = User::get(name.as_str()).await.unwrap();
    // //let user: User = users.get(0).unwrap().clone();

    // //let actor: PersonActor = user.to_actor().await;
    // let actor = PersonActor::create(user).await.unwrap();
    // let s = serde_json::to_string(&actor)?;

    // tracing::debug!(s);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("s".to_owned())
        .build())
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    not_found(req, params).await
}

pub async fn outbox(
    _req: Request,
    _params: Params,
) -> Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("arsars".to_owned())
        .build())
}
