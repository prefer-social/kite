use spin_sdk::{
    http::{HeaderValue, Params, Request, Response, Router},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};
use url::Url;

pub mod featured;
pub mod foo;
pub mod http_responses;
pub mod outbox;
pub mod tests;
pub mod users;
pub mod utils;

/// A Spin HTTP component that internally routes requests.
#[http_component]
async fn handle_route(req: Request) -> Response {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();

    let headers = req
        .headers()
        .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
        .collect::<Vec<_>>();

    let n_a_header_val = HeaderValue::string("".to_string());
    let ip = req.header("x-real-ip").unwrap_or(&n_a_header_val);

    tracing::debug!("<---------- ({request_method}) {request_path_and_query} ({ip:?}) --------->");

    // Modifying headers, Custom header
    let mut headers = req
        .headers()
        .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
        .collect::<Vec<_>>();

    headers.push(("foo".to_string(), "bar".as_bytes().to_vec()));

    let req = Request::builder()
        .uri(req.uri())
        .method(req.method().clone())
        .headers(headers.clone())
        .body(req.into_body())
        .build();

    // End of modifying headers

    let mut router = Router::new();

    let _domain = req.header("host").unwrap().as_str().unwrap();

    if let Some(a) = req.header("accept") {
        if let Some("application/activity+json") = a.as_str() {
            let original_uri_str = req.uri();
            let original_uri = Url::parse(original_uri_str).unwrap();

            let a = &req.path_and_query().unwrap()[..2];
            if a == "/@" {
                let user = &req.path_and_query().unwrap()[2..];
                let uri = format!(
                    "{}://{}/users/{}",
                    original_uri.scheme(),
                    original_uri.host_str().unwrap(),
                    user,
                );
                let req = Request::builder()
                    .uri(uri)
                    .method(req.method().clone())
                    .headers(headers)
                    .body(req.into_body())
                    .build();
                router.get_async("/users/:user", users::request);
                return router.handle_async(req).await;
            }
        }
    }

    router.get_async("/users/:user", users::request);
    router.any_async("/users/:user/inbox", users::inbox::request);
    router.any_async("/users/:user/outbox", users::outbox::request);

    router.any_async("/users/:user/following", users::following::request);
    router.any_async("/users/:user/followers", users::followers::request);
    router.any_async("/users/:user/collections/featured", featured::request);
    //router.get_async("/users/:user/collections/tags", tags::request);

    // tests
    router.any_async("/tests/db", tests::db);

    //
    router.any_async("/foo/following_request", foo::following_request);
    router.any_async("/foo", foo::modified_header_test);
    router.any_async("/", foo::root);
    router.handle_async(req).await
}
