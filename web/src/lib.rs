use spin_sdk::{
    http::{HeaderValue, Request, Response, Router},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

pub mod actor;
pub(crate) mod http_response;
pub(crate) mod util;

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
        req.header("x-forwarded-for")
            .unwrap_or(&HeaderValue::string("EMPTY".to_string()))
            .as_str()
            .unwrap(),
        req.header("Accept")
            .unwrap_or(&HeaderValue::string("EMPTY Accept header".to_string()))
            .as_str()
            .unwrap(),
    );

    let a = cfg!(target_family = "wasm");

    let headers = req
        .headers()
        .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
        .collect::<Vec<_>>();

    let req = Request::builder()
        //.uri(modified_uri)
        .uri(req.uri())
        .method(req.method().clone())
        .headers(headers.clone())
        .body(req.into_body())
        .build();

    // Don't need this. Too many sql queries
    let owner =
        sparrow::mastodon::setting::Setting::get("site_contact_username")
            .await
            .unwrap();

    let mut router = Router::new();

    // Actor endpoints (multiple for compatibility)
    router.any_async("/", actor::req);
    router.any_async("/self", actor::req);
    router.any_async(format!("@{}", owner).as_str(), actor::req);
    router.any_async(format!("/users/{}", owner).as_str(), actor::req);

    router.handle_async(req).await
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
