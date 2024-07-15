use spin_sdk::http::{Params, Request, Response, Router};

pub mod root_page;

pub async fn renderer(req: Request) -> Response {
    let mut router = Router::new();
    router.any_async("/", root_page::request);
    router.handle_async(req).await
}

// foo, for debug purpose
async fn foo(_req: Request, _params: Params) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html; charset=UTF-8")
        .body("foo foo")
        .build())
}
