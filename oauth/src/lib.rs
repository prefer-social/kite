use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};
use tracing::debug;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

pub mod oauth;

// TODO: request by 'http://ap.dev.seungjin.net/oauth/authorize?response_type=code&client_id=client_id&redirect_uri=icecubesapp://&scope=read%20write%20follow%20push'
#[http_component]
async fn handle_route(req: Request) -> Response {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();
    let ip = req.header("x-real-ip").unwrap().as_str().unwrap();

    debug!("<---------- ({request_method}) {request_path_and_query} ({ip}) --------->");

    let mut router = Router::new();
    router.any_async("/oauth/authorize", oauth::authorize::request);
    router.any_async("/oauth/token", oauth::token::request);
    //router.any_async("/oauth/revoke", oauth::revoke::request);
    router.handle(req)
}
