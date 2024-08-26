//! Mastodon API implementation for prefer.social.  

use std::fmt::Debug;

use anyhow::Result;
use http_response::HttpResponse;
use spin_sdk::{
    http::{IntoResponse, Request, Router},
    http_component,
};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

use crate::auth::Authentication;
use sparrow::mastodon::ME_ACCOUNT;
use sparrow::REQUEST_UID;

pub(crate) mod auth;
pub(crate) mod endpoint;
pub(crate) mod http_response;

#[http_component]
async fn handle_api(req: Request) -> Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!(
        "<---------- ({}) {} ({}) --------->",
        req.method().to_string(),
        req.path_and_query().unwrap(),
        req.header("x-forwarded-for")
            .unwrap_or_default()
            .as_str()
            .unwrap()
    );

    match REQUEST_UID.set(Uuid::now_v7()) {
        Ok(_) => {
            tracing::trace!("REQUEST_UID set")
        }
        Err(e) => {
            tracing::error!("REQUEST_UID sentting error {e:?}")
        }
    };

    // Check req auth and if it valid. set ME_ACCOUNT
    // Also authorization process.
    let me_account = match Authentication::verify(&req).await {
        Some(a) => a,
        None => return HttpResponse::forbidden(),
    };

    match ME_ACCOUNT.set(me_account.to_owned()) {
        Ok(_) => {
            tracing::trace!("ME_ACCOUNT loaded into global space")
        }
        Err(e) => {
            tracing::error!("ME_ACCOUNT sentting error {e:?}")
        }
    };

    let mut router = Router::new();

    router.any_async("/api/v1/instance", endpoint::v1::instance::request);
    router.any_async("/api/v1/apps", endpoint::v1::apps::request);
    router.any_async(
        "/api/v1/accounts/verify_credentials",
        endpoint::v1::accounts::verify_credentials::request,
    );
    router.any_async(
        "/api/v1/followed_tags",
        endpoint::v1::followed_tags::request,
    );
    router.any_async("/api/v1/lists", endpoint::v1::lists::request);
    router.any_async(
        "/api/v1/follow_requests",
        endpoint::v1::follow_requests::request,
    );
    router.any_async(
        "/api/v1/instance/peer",
        endpoint::v1::instance::peer::request,
    );
    router.any_async(
        "/api/v1/timelines/home",
        endpoint::v1::timelines::home::request,
    );
    router.any_async(
        "/api/v1/push/subscription",
        endpoint::v1::push::subscription::request,
    );
    router.any_async("/api/v1/streaming", endpoint::v1::streaming::request);
    router.any_async(
        "/api/v1/notifications",
        endpoint::v1::notifications::request,
    );
    router.any_async(
        "/api/v1/conversations",
        endpoint::v1::conversations::request,
    );
    router.any_async(
        "/api/v1/accounts/relationships",
        endpoint::v1::accounts::relationships::request,
    );

    // router.any_async("/api/v1/timelines/public", timelines::public::request);

    //
    // //router.any_async("/api/v1/streaming", streaming::request);
    // router.any_async("/api/v1/streaming/health", streaming::health::request);
    //

    // Account API requests
    router.any_async("/api/v1/accounts/:id", endpoint::v1::accounts::request);
    router.any_async(
        "/api/v1/accounts/:id/follow",
        endpoint::v1::accounts::follow::request,
    );
    router.any_async(
        "/api/v1/accounts/:id/unfollow",
        endpoint::v1::accounts::unfollow::request,
    );
    router.any_async(
        "/api/v1/accounts/:id/following",
        endpoint::v1::accounts::following::request,
    );
    // router.any_async(
    //     "/api/v1/accounts/:id/statuses",
    //     accounts::statuses::request,
    // );
    // router.any_async(
    //     "/api/v1/accounts/relationships",
    //     accounts::relationships::request,
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
