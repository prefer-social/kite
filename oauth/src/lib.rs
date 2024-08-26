use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};
use tracing::debug;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

pub(crate) mod http_response;
pub(crate) mod oauth;

// TODO: request by 'http://dev.prefer.social/oauth/authorize?response_type=code&client_id=client_id&redirect_uri=icecubesapp://&scope=read%20write%20follow%20push'
#[http_component]
async fn handle_route(req: Request) -> Response {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-forwarded-for").unwrap().as_str().unwrap()
    );

    let mut router = Router::new();
    router.any_async("/oauth/authorize", oauth::authorize::request);
    router.any_async("/oauth/token", oauth::token::request);
    //router.any_async("/oauth/revoke", oauth::revoke::request);
    router.handle(req)
}
