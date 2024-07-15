use anyhow::Result;
use spin_sdk::http::{IntoResponse, Params, Request, Response};

use url::Url;

pub async fn hostmeta(
    req: Request,
    _params: Params,
) -> Result<impl IntoResponse> {
    let from = req.header("spin-client-addr").unwrap().as_str().unwrap();
    tracing::debug!("-> host-meta requested from: {from}");

    let host: Url = req.uri().parse().unwrap();

    let a = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource={{uri}}"/>
    </XRD>"#,
        host.host().unwrap()
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/xrd+xml")
        .body(a)
        .build())
}
