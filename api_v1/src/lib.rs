use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Response, Router},
    http_component,
};
use tracing::{debug, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

pub mod accounts;
pub mod apps;
pub mod bookmarks;
pub mod favourites;
pub mod http_responses;
pub mod instance;
pub mod list;
pub mod media;
pub mod statuses;
pub mod streaming;
pub mod timelines;

#[http_component]
async fn handle_api(req: Request) -> Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();
    let ip = req.header("x-real-ip").unwrap().as_str().unwrap();

    debug!("<---------- ({request_method}) {request_path_and_query} ({ip}) --------->");

    let mut router = Router::new();

    router.any_async("/api/v1/instance", instance::request);
    router.any_async("/api/v1/apps", apps::request);
    router.any_async(
        "/api/v1/accounts/verify_credentials",
        accounts::verify_credentials::request,
    );
    //router.any_async("/api/v1/instance/peers", streaming::request);
    router.any_async("/api/v1/timelines/home", timelines::home::request);
    //router.any_async("/api/v1/push/subscription", streaming::request);

    //router.any_async("/api/v1/streaming", streaming::request);
    router.any_async("/api/v1/streaming/health", streaming::health::request);

    router.any_async("/api/v1/accounts/:id/statuses", accounts::statuses::request);
    router.any_async(
        "/api/v1/accounts/relationships",
        accounts::relationships::request,
    );
    router.any_async("/api/v1/accounts/:id/follow", accounts::follow::request);
    router.any_async("/api/v1/accounts/:id/unfollow", accounts::unfollow::request);
    router.any_async("/api/v1/statuses", statuses::request);
    router.any_async("/api/v1/favourites", favourites::request);
    router.any_async("/api/v1/bookmarks", bookmarks::request);

    router.any_async("/api/v1/media/:id", media::request);

    router.any_async("/api/v1/list", list::request);
    Ok(router.handle_async(req).await)
}
