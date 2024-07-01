use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Request, Response, Router},
    http_component,
};
use tracing::{debug, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

pub mod endpoint;

#[http_component]
async fn handle_api(req: Request) -> Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!("<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-real-ip").unwrap().as_str().unwrap()
    );

    let mut router = Router::new();

    router.any_async("/api/v1/instance", endpoint::v1::instance::request);
    router.any_async("/api/v1/apps", endpoint::v1::apps::request);
    router.any_async(
         "/api/v1/accounts/verify_credentials",
         endpoint::v1::accounts::verify_credentials::request,
    );
    router.any_async("/api/v1/followed_tags", endpoint::v1::followed_tags::request);
    router.any_async("/api/v1/lists", endpoint::v1::lists::request);
    router.any_async("/api/v1/follow_requests", endpoint::v1::follow_requests::request);
    router.any_async("/api/v1/instance/peer", endpoint::v1::instance::peer::request);
    router.any_async("/api/v1/timelines/home", endpoint::v1::timelines::home::request);
    router.any_async("/api/v1/push/subscription", endpoint::v1::push::subscription::request);
    router.any_async("/api/v1/streaming", endpoint::v1::streaming::request);
    router.any_async("/api/v1/notifications", endpoint::v1::notifications::request);
    router.any_async("/api/v1/conversations", endpoint::v1::conversations::request);
    router.any_async("/api/v1/accounts/relationships", endpoint::v1::accounts::relationships::request);

    // router.any_async("/api/v1/timelines/public", timelines::public::request);

    //
    // //router.any_async("/api/v1/streaming", streaming::request);
    // router.any_async("/api/v1/streaming/health", streaming::health::request);
    //
    // router.any_async(
    //     "/api/v1/accounts/:id/statuses",
    //     accounts::statuses::request,
    // );
    // router.any_async(
    //     "/api/v1/accounts/relationships",
    //     accounts::relationships::request,
    // );
    // router.any_async("/api/v1/accounts/:id", accounts::request);
    // router.any_async("/api/v1/accounts/:id/follow", accounts::follow::request);
    // router.any_async(
    //     "/api/v1/accounts/:id/unfollow",
    //     accounts::unfollow::request,
    // );
    // router.any_async("/api/v1/statuses", statuses::request);
    // router.any_async("/api/v1/favourites", favourites::request);
    // router.any_async("/api/v1/bookmarks", bookmarks::request);
    //
    // router.any_async("/api/v1/media/:id", media::request);
    //
    // router.any_async("/api/v1/list", list::request);

    router.any_async("/api/v2/search", endpoint::v2::search::request);
    router.any_async("/api/v2/media", endpoint::v2::media::request);
    router.any_async("/api/v2/instance", endpoint::v2::instance::request);

    Ok(router.handle_async(req).await)
}
