use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Router},
    http_component,
};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

pub mod http_responses;
pub mod instance;
pub mod media;
pub mod search;
pub mod search_account;

#[http_component]
async fn handle_api(req: Request) -> Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();
    let ip = req.header("x-real-ip").unwrap().as_str().unwrap();

    tracing::debug!("<---------- ({request_method}) {request_path_and_query} ({ip}) --------->");

    let mut router = Router::new();

    router.any_async("/api/v2/search", search::request);
    router.any_async("/api/v2/media", media::request);
    router.any_async("/api/v2/instance", instance::request);
    Ok(router.handle_async(req).await)
}
