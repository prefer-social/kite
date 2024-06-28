use spin_sdk::{
    http::{HeaderValue, Params, Request, Response, Router},
    http_component, variables,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};
use url::Url;

pub mod featured;
//pub mod foo;

pub mod endpoint;
pub mod tests;
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

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();

    let headers = req
        .headers()
        .map(|(k, v)| (k.to_string(), v.as_bytes().to_vec()))
        .collect::<Vec<_>>();

    let n_a_header_val = HeaderValue::string("".to_string());
    let ip = req.header("x-real-ip").unwrap_or(&n_a_header_val);

    tracing::debug!("<---------- ({request_method}) {request_path_and_query} ({ip:?}) --------->");

    // Modifying headers, Custom header (Sample)
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

    // Get Admin / Main /


    //let _domain = req.header("host").unwrap().as_str().unwrap();
    //let _domain_from_env = variables::get("DOMAIN").unwrap();

    let path = req.path_and_query().unwrap().to_string();

    match path.starts_with("/@") {
        true => endpoint::actor::request(req, Params::new()).await.unwrap(),
        false => {
            let mut router = Router::new();

            router.any_async("/", endpoint::actor::request);
            router.get_async("/u/:user", endpoint::actor::request);
            router.get_async("/users/:user", endpoint::actor::request);
            router.get_async("/actor", endpoint::actor::request);
            router.get_async("/self", endpoint::actor::request);

            router.any_async("/inbox", endpoint::inbox::request);

            router.any_async("/outbox", endpoint::outbox::request);

            router.any_async("/following", endpoint::following::request);
            router.any_async("/followers", endpoint::followers::request);
            // router.any_async("/users/:user/following", users::following::request);

            // router.any_async("/users/:user/followers", users::followers::request);
            // router.any_async("/users/:user/collections/featured", featured::request);
            //router.get_async("/users/:user/collections/tags", tags::request);

            // tests
            //router.any_async("/tests/db", tests::db);

            //
            //router.any_async("/foo/following_request", foo::following_request);
            //router.any_async("/foo", foo::modified_header_test);
            //router.any_async("/", foo::root);
            router.handle_async(req).await
        }
    }
}
