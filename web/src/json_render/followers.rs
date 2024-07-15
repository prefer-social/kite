use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

pub async fn req(req: Request, params: Params) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

pub async fn get(req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "applicagtion/activity+json")
        .body(
            sparrow::activitypub::following::Following::build()
                .await
                .to_json_string()
                .await
                .unwrap(),
        )
        .build())
}
