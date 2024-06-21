use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Method},
};

pub async fn request(
    req: Request,
    params: Params,
) -> anyhow::Result<Response> {
    match crate::utils::check_request(&req).await {
        (Method::Get, crate::utils::RenderType::Json) => emit_json(req, params).await,
        (Method::Get, _) => emit_html(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

pub async fn emit_json(req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "applicagtion/activity+json")
        .body(
            sparrow::activitypub::following::Following::build().await.to_json_string().await.unwrap()
        )
        .build())
}

pub async fn emit_html(req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("followers")
        .build())
}


