use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};

pub async fn req(
    req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
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

pub async fn emit_html(
    req: Request,
    _params: Params,
) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("following")
        .build())
}
