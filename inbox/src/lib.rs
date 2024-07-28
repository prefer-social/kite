use anyhow::Result;
use serde_json::Value;
use spin_sdk::{
    http::{HeaderValue, IntoResponse, Method, Request, Response},
    http_component,
};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

//use sparrow::activitypub::action::follow::Follow as FollowAction;
use crate::http_response::HttpResponse;
use sparrow::activitypub::object::Object as APObject;
use sparrow::activitypub::object::ObjectType;

mod action;
mod http_response;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_inbox(req: Request) -> anyhow::Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!(
        "<---------- ({}) {} ({}) {}--------->",
        req.method().to_string(),
        req.path_and_query().unwrap_or_default(),
        req.header("x-forwarded-ip")
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
        Method::Post => post(req).await,
        _ => HttpResponse::method_not_allowed(),
    }
}

async fn get(_req: Request) -> Result<Response> {
    tracing::debug!("GET to INBOX");
    tracing::debug!("Do we get 'GETo request to inbox?");
    HttpResponse::not_implemented()
}

pub async fn post(req: Request) -> Result<Response> {
    tracing::debug!("POST to INBOX");

    // Validate signature
    if !sparrow::mastodon::validate_signature(&req).await? {
        tracing::debug!("NOT VALID SIGNATURE");
        return HttpResponse::invalid_request();
    }

    tracing::debug!("VALID SIGNATURE");

    // Get posted body
    let body = String::from_utf8_lossy(req.body()).to_string();
    let obj: APObject<Value> = serde_json::from_str(&body)?;
    let _actor_from_body = obj.actor.clone();

    // Delete(Account Gone) processing.
    return match obj.object_type {
        ObjectType::Delete => action::delete::delete(obj).await,
        ObjectType::Follow => action::follow::follow(obj).await,
        ObjectType::Accept => action::accept::accept(obj).await,
        action => {
            // returns
            // HttpResponse::invalid_request()
            tracing::debug!("action '{:?}' is not implemented yet", action);
            HttpResponse::teapot()
        }
    };
}
