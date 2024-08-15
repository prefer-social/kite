use anyhow::{Error, Result};
use serde_json::Value;
use spin_sdk::{
    http::{HeaderValue, IntoResponse, Method, Request, Response},
    http_component,
    key_value::Store,
};
use std::{io::Read, str::FromStr};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};
use url::Url;

//use sparrow::activitypub::action::follow::Follow as FollowAction;
use crate::http_response::HttpResponse;
use sparrow::activitystream::activity::accept::Accept as AcceptActivity;
use sparrow::activitystream::activity::create::Create as CreateActivity;
use sparrow::activitystream::activity::delete::Delete as DeleteActivity;
use sparrow::activitystream::activity::follow::Follow as FollowActivity;
use sparrow::activitystream::activity::undo::Undo as UndoActivity;
use sparrow::activitystream::activity::Activity;
use sparrow::activitystream::activity::ActivityType;
use sparrow::activitystream::object::note::Note as NoteObject;
use sparrow::activitystream::object::ObjectType;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::ValidationResult;
use sparrow::mstor;

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
        "<--------- ({}) {} ({}) {} --------->",
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

    let (body, activity_type, object_type) =
        inspect(String::from_utf8_lossy(req.body()).to_string());

    tracing::debug!(
        "{:?} {:?} by {:?}",
        activity_type,
        object_type,
        body.get("actor").unwrap()
    );

    // Case Actor/Account delete
    if activity_type == ActivityType::Delete {
        if object_type.is_some() {
            let actor_url = object_type.clone().unwrap();
            if Url::parse(actor_url.as_str()).is_ok() {
                // This is Actor Delete. Don't need to check validate_signature
                let activity =
                    serde_json::from_value::<Activity<DeleteActivity>>(body)
                        .unwrap();
                return match activity.execute().await {
                    Ok(_) => HttpResponse::accepted(),
                    Err(e) => {
                        tracing::error!(
                            "Error from Inbox's Delete request -> {e:?}",
                        );
                        HttpResponse::not_acceptable()
                    }
                };
            }
        }
    }

    let validation = sparrow::mastodon::validate_signature(&req).await?;

    if validation == ValidationResult::Invalid {
        tracing::debug!("NOT VALID SIGNATURE");
        tracing::debug!("{:?}", String::from_utf8(req.body().to_vec()));
        return HttpResponse::invalid_request();
    };

    tracing::debug!("VALID SIGNATURE");

    if let ValidationResult::Valid(acct) = validation {
        let store = Store::open("mem").unwrap();
        mstor::set_val(
            &store,
            "activity_actor",
            &serde_json::to_value(acct).unwrap(),
        )?;
    };

    match activity_type {
        ActivityType::Accept => {
            //action::accept::received(obj).await,
            let activity =
                serde_json::from_value::<Activity<AcceptActivity>>(body)
                    .unwrap();
            match activity.execute().await {
                Ok(_) => HttpResponse::accepted(),
                Err(e) => {
                    tracing::error!(
                        "Error from Inbox's Accpet request -> {e:?}",
                    );
                    HttpResponse::not_acceptable()
                }
            }
        }
        ActivityType::Create => {
            let activity =
                serde_json::from_value::<Activity<CreateActivity>>(body)
                    .unwrap();
            let _ot = ObjectType::from_str(object_type.unwrap().as_str());

            match activity.execute().await {
                Ok(_) => HttpResponse::accepted(),
                Err(e) => {
                    tracing::error!(
                        "Error from Inbox's Create request -> {e:?}",
                    );
                    HttpResponse::not_acceptable()
                }
            }
        }
        ActivityType::Delete => {
            let activity =
                serde_json::from_value::<Activity<DeleteActivity>>(body)
                    .unwrap();
            match activity.execute().await {
                Ok(_) => HttpResponse::accepted(),
                Err(e) => {
                    tracing::error!(
                        "Error from Inbox's Delete request -> {e:?}",
                    );
                    HttpResponse::not_acceptable()
                }
            }
        }
        ActivityType::Follow => {
            let activity =
                serde_json::from_value::<Activity<FollowActivity>>(body)
                    .unwrap();
            match activity.execute().await {
                Ok(_) => HttpResponse::accepted(),
                Err(e) => {
                    tracing::error!(
                        "Error from Inbox's Follow request -> {e:?}",
                    );
                    HttpResponse::not_acceptable()
                }
            }
        }
        ActivityType::Undo => {
            let activity =
                serde_json::from_value::<Activity<UndoActivity>>(body)
                    .unwrap();
            match activity.execute().await {
                Ok(_) => HttpResponse::accepted(),
                Err(e) => {
                    tracing::error!(
                        "Error from Inbox's Follow request -> {e:?}",
                    );
                    HttpResponse::not_acceptable()
                }
            }
        }
        action => {
            tracing::warn!("action '{:?}' is UNKNOWN", action);
            HttpResponse::not_acceptable()
        }
    }
}

pub fn inspect(body: String) -> (Value, ActivityType, Option<String>) {
    let v = serde_json::from_str::<Value>(body.as_str()).unwrap();
    let v_type = v.get("type").unwrap().as_str().unwrap();
    let activity_type = ActivityType::from_str(v_type).unwrap();

    let a = v.get("object").unwrap();
    let object_type = match a.is_object() {
        true => a.get("type").unwrap().as_str().map(String::from),
        _ => Some(a.as_str().unwrap().to_string()),
    };

    (v, activity_type, object_type)
}
