use anyhow::Result;
use serde_json::Value;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use std::collections::HashMap;
use std::str;
use url::Url;

use crate::http_response::HttpResponse;

use sparrow::activitypub::object::Object as APObject;
use sparrow::activitypub::person_actor::PersonActor;
use sparrow::mastodon::account::actor_url::ActorUrl;
use sparrow::mastodon::account::uri as AccountUri;
use sparrow::mastodon::account::Account as MAccount;
use sparrow::mastodon::account::Get as _;

// Very special case, web is accessing directly TAccount

pub async fn req(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => HttpResponse::method_not_allowed().await,
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("GET to INBOX");
    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body("/inbox")
        .build())
}
pub async fn post(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("POSTED to INBOX");

    let mut header_hashmap = HashMap::new();
    for (k, v) in req.headers() {
        let k = k.to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }

    // Get sig_headers
    let sig_header = match req.header("Signature") {
        Some(s) => s.as_str().unwrap(),
        None => return HttpResponse::invalid_request().await,
    };

    // Get posted body
    let body = String::from_utf8_lossy(req.body()).to_string();

    // Get Actor id from sig_header. key id
    fn get_id_from_sig_header(query: &str) -> String {
        fn rem_first_and_last(value: &str) -> &str {
            let mut chars = value.chars();
            chars.next();
            chars.next_back();
            chars.as_str()
        }
        query
            .split(',')
            .filter_map(|s| {
                s.split_once('=').and_then(|t| {
                    Some((t.0.to_owned(), rem_first_and_last(t.1).to_owned()))
                })
            })
            .collect::<HashMap<String, String>>()
            .get("keyId")
            .unwrap()
            .to_string()
    }
    let key_id = get_id_from_sig_header(sig_header);
    let ki: Url = key_id.parse().unwrap();
    let actor_from_key_id =
        format!("{}://{}{}", ki.scheme(), ki.host().unwrap(), ki.path());

    // Get Actor id from posted body
    let obj: APObject<Value> = serde_json::from_str(&body)?;
    let actor_from_body = obj.actor;

    // Match them.
    if actor_from_body != actor_from_key_id {
        return sparrow::http_response::HttpResponse::invalid_request().await;
    }

    let v = serde_json::from_str::<Value>(&body).unwrap();
    let actor_url_value = v.get::<&str>("actor").unwrap().as_str().unwrap();
    let actor_url = ActorUrl::new(actor_url_value.to_string()).unwrap();
    let person_actor = actor_url.actor().await?;
    person_actor.store().await?;

    // Validate sig header
    let account = MAccount::get(actor_url).await?.to_owned();
    let public_key = account.public_key.as_str();
    if !sparrow::mastodon::validate_signature(&req, public_key).await? {
        // NOT VALID signature
        tracing::debug!("NOT VALID SIGNATURE");
        return HttpResponse::unauthorized().await;
    }

    // LETS STORE this INBOX Post to DB

    match obj.kind.as_str() {
        "Follow" => {
            tracing::debug!("FOLLOW");
            return HttpResponse::not_implemented().await;
        }
        "Undo" => {
            tracing::debug!("UNDO");
            return HttpResponse::not_implemented().await;
        }
        verb => {
            tracing::debug!("{} is not implemented yet", obj.kind.as_str());
            tracing::debug!("{} is not implemented yet", verb);
            return HttpResponse::invalid_request().await;
        }
    }
}

/*
{
"@context":"https://www.w3.org/ns/activitystreams",
"id":"https://mas.to/10a53d78-4f95-4ee0-b2c7-89458a761298",
"type":"Follow",
"actor":"https://mas.to/users/seungjin",
"object":"https://seungjin.ap.dev.seungjin.net/users/seungjin"
}
*/

/*

pub async fn handle_activity(a: &str) -> Result<(), anyhow::Error> {
    //debug!("Actor: {:?}", activity.actor());
    //debug!("Object: {:?}", activity.object());

    tracing::debug!("--- handling acticity ---");
    tracing::debug!("----> {}", a);

    let activity: AcceptedActivity = serde_json::from_str(a)?;
    tracing::debug!("{:?}", activity);
    let b: serde_json::Value = serde_json::from_str(a).unwrap();
    tracing::debug!("{:?}", b);

    match activity.kind() {
        Some(AcceptedTypes::Accept) => accept::accept_action(b).await,
        Some(AcceptedTypes::Announce) => println!("Announce"),
        Some(AcceptedTypes::Create) => println!("Create"),
        Some(AcceptedTypes::Delete) => println!("Delete"),
        Some(AcceptedTypes::Follow) => follow::follow_action(b).await,
        Some(AcceptedTypes::Reject) => println!("Reject"),
        Some(AcceptedTypes::Update) => println!("Update"),
        Some(AcceptedTypes::Undo) => undo::undo_action(b).await,
        None => return Err(anyhow::Error::msg("No activity type provided")),
    }

    Ok(())
}



pub async fn undo_action(activity: AcceptedActivity) {
    println!("########### undo_action");
    //println!("Actor: {:?}", activity.actor());
    //println!("Object: {:?}", activity.object());
    println!("undo_action ##############");
}

*/

pub async fn record_to_inbox(req: &Request, body: &str) -> Result<()> {
    tracing::debug!("??????????????????????????????????");

    //method: Method,
    //uri: (Option<hyperium::Uri>, String),
    //headers: HashMap<String, HeaderValue>,
    //body: Vec<u8>,

    // let method = req.method();
    // let uri = req.uri();
    // let headers = req.headers();
    // let body = req.body();

    // let hostname = req.header("Host").unwrap().as_str().unwrap();
    // let date = req.header("Date").unwrap().as_str().unwrap();
    // let sig_header = req.header("Signature").unwrap().as_str().unwrap();
    // let _digest = req.header("Digest").unwrap().as_str().unwrap();
    // let _content_type = req.header("content-type").unwrap().as_str().unwrap();
    // let _request_path =
    //     req.header("spin-path-info").unwrap().as_str().unwrap();
    // let _request_method = req.method().to_string();

    // let query_params = [
    //     Value::Text(sig_header.to_string()),
    //     Value::Text(hostname.to_string()),
    //     Value::Text(date.to_string()),
    //     Value::Text(body.to_string()),
    // ];

    // //let a = sparrow::table::inbox_log::InboxLog.put();
    // sparrow::table::inbox_logs::InboxLog::put(

    //     sig_header: String,
    //     hostname: String,
    //     body: String,
    // );

    Ok(())
}
