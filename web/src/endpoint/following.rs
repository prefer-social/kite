use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

use crate::http_response::HttpResponse;

pub async fn req(
    req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found(),
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(
            sparrow::activitypub::follow::following::Following::build()
                .await
                .to_json_string()
                .await
                .unwrap(),
        )
        .build())
}
