use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Method},
};

pub async fn request(
    req: Request,
    params: Params,
) -> anyhow::Result<Response> {
    match crate::utils::check_request(&req).await {
        (Method::Get, crate::utils::RenderType::Json) => emit_json(req, params).await,
        //(Method::Get, _) => emit_json(req, params).await,
        (Method::Get, _) => emit_html(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
    }
}

pub async fn emit_json(req: Request, _params: Params) -> anyhow::Result<Response> {
    let (user, account) =
                    sparrow::table::user::User::default_user().await.unwrap();
    let actor =  sparrow::activitypub::person_actor::PersonActor::build(user, account).await.unwrap();
    let s = serde_json::to_string(&actor).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(s)
        .build())
}

pub async fn emit_html(req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("dev.prefer.social")
        .build())
}


