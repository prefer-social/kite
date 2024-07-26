use spin_sdk::{
    http::{HeaderValue, Request, Response},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

pub mod endpoint;
pub mod http_response;
pub mod util;

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
        req.path_and_query().unwrap_or_default(),
        req.header("x-real-ip")
            .unwrap_or(&HeaderValue::string("EMPTY".to_string()))
            .as_str()
            .unwrap(),
        req.header("Accept")
            .unwrap_or(&HeaderValue::string("EMPTY Accept header".to_string()))
            .as_str()
            .unwrap(),
    );

    let _request_path_and_query = req.path_and_query().unwrap();
    let _request_method = req.method();
    let _requestd_uri = req.uri();

    //match what_type_asked(&req).await {
    //    Some("application/activity+json") => json_render::renderer(req).await,
    //    _ => html_render::renderer(req).await,
    //}

    endpoint::router(req).await
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
