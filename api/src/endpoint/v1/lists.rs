// https://docs.joinmastodon.org/methods/lists/#get
use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response}
};

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

// Returns: Array of List
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    // TODO: implement this.
    // Return empty array for now.

    let content: Vec<sparrow::mastodon::list::List> = Vec::new();
    let return_body = serde_json::to_string(&content).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(return_body)
        .build())
}
