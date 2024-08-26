use spin_sdk::{
    http::{HeaderValue, IntoResponse, Method, Request, Response},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

//use sparrow::activitypub::action::follow::Follow as FollowAction;
use crate::http_response::HttpResponse;
use sparrow::activitystream::ordered_collection::OrderedCollection;
use sparrow::mastodon::account::Account as MAccount;

pub mod http_response;

#[http_component]
async fn handle_following(req: Request) -> anyhow::Result<impl IntoResponse> {
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

    match req.method() {
        Method::Get => get(req).await,
        _ => HttpResponse::method_not_allowed(),
    }
}

pub async fn get(req: Request) -> anyhow::Result<Response> {
    let (default_account, _) = MAccount::default().await?;

    let following = OrderedCollection::new(
        default_account.following_url.unwrap(),
        default_account.following_count as i64,
    );

    let json_string = serde_json::to_string(&following).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(json_string)
        .build())
}
