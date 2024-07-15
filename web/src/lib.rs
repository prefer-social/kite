use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

pub mod html_render;
pub mod json_render;

//pub mod users;
pub mod utils;

/// A Spin HTTP component that internally routes requests.
#[http_component]
async fn handle_route(req: Request) -> Response {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!(
        "<---------- ({}) {} ({}) {}--------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap(),
        req.header("Accept").unwrap().as_str().unwrap(),
    );

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method();
    let requestd_uri = req.uri();

    match what_type_asked(&req).await {
        Some("application/activity+json") => json_render::renderer(req).await,
        _ => html_render::renderer(req).await,
    }
}

/// Return HTTP Mime type
/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
async fn what_type_asked<'a>(req: &Request) -> Option<&'a str> {
    let asked_string = match req.header("Accept") {
        None => return None,
        Some(x) => x.as_str().unwrap(),
    };

    if asked_string.contains("json") {
        return Some("application/activity+json");
    }

    Some("text/html")
}
