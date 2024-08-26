// https://docs.joinmastodon.org/methods/preferences

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::http_response::HttpResponse;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

// https://docs.joinmastodon.org/methods/preferences/#get
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-forwarded-for").unwrap().as_str().unwrap()
    );

    // TODO: implement this.
    // Return empty array for now.

    let content: Vec<sparrow::mastodon::list::List> = Vec::new();
    let return_body = serde_json::to_string(&content).unwrap();

    let rbody = r#"{
        "posting:default:visibility": "public",
        "posting:default:sensitive": false,
        "posting:default:language": null,
        "reading:expand:media": "default",
        "reading:expand:spoilers": false
    }"#;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(rbody)
        .build())
}
