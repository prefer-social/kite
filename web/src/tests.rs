use std::thread::spawn;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Params, Request, Response};

use tracing::debug;

pub async fn db(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let ep = [spin_sdk::sqlite::Value::Text("Hello".to_owned())];

    let a_result = sparrow::db::Connection::builder()
        .await
        .execute("select ? AS a1", &ep)
        .await;
    debug!("{a_result:?}");

    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }

    let b_result = sparrow::db::Connection::builder()
        .await
        .execute("select ? AS a2", &ep)
        .await;
    debug!("{b_result:?}");

    // Builder patter. Simpler
    let c = sparrow::db::Connection::builder()
        .await
        .execute("select 1123123123", &[])
        .await;
    debug!("{c:?}");

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("arsararsarsarsarss".to_owned())
        .build())
}
