////////////////////////////////////////////////////////////////////////////////////////
/// No LONGER USED
///
///
///
///
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::Value,
};
use std::collections::HashMap;
use std::str;
use url::Url;

use crate::users::activities::{accept, follow, undo};
use crate::utils::{not_found, unauthorized};

use sparrow::activitypub::accepted_activity::AcceptedActivity;
use sparrow::activitypub::accepted_activity::AcceptedTypes;
use sparrow::utils::get_public_key;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("GET to INBOX");
    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .build())
}
pub async fn post(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("POSTED to INBOX");
    // TODO: First thing to do -> Create my actor/user struct based on user name!
    // Reuse one for all

    let sig_header = req.header("Signature").unwrap().as_str().unwrap();

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

    tracing::debug!(actor_from_key_id);

    let pubkey_str = match get_public_key(&actor_from_key_id).await {
        Ok(x) => x,
        Err(x) => {
            tracing::debug!("------------");
            tracing::debug!(
                "Getting an error from getting pubkey: {:?} {:?})",
                x,
                &actor_from_key_id
            );

            return Ok(Response::builder()
                .status(200)
                .header("Context-Type", "application/activity+json")
                .build());
        }
    };

    let valid_signature =
        sparrow::mastodon::validate_mastodon_request(&req, &pubkey_str)
            .await
            .unwrap();

    tracing::debug!("valid signature: {:?}", valid_signature);

    let b = str::from_utf8(req.body()).unwrap();

    if valid_signature {
        tracing::debug!("VALID SIGNATURE");
        tracing::debug!("body ->\n {b}");

        record_to_inbox(&req, &b).await?;
        handle_activity(b).await?;
    } else {
        tracing::debug!("NOT VALID SIGNATURE");
        return unauthorized().await;
    }

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .build())
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

pub async fn record_to_inbox(req: &Request, body: &str) -> Result<()> {
    let hostname = req.header("Host").unwrap().as_str().unwrap();
    let date = req.header("Date").unwrap().as_str().unwrap();
    let sig_header = req.header("Signature").unwrap().as_str().unwrap();
    let _digest = req.header("Digest").unwrap().as_str().unwrap();
    let _content_type = req.header("content-type").unwrap().as_str().unwrap();
    let _request_path =
        req.header("spin-path-info").unwrap().as_str().unwrap();
    let _request_method = req.method().to_string();

    let query_params = [
        Value::Text(sig_header.to_string()),
        Value::Text(hostname.to_string()),
        Value::Text(date.to_string()),
        Value::Text(body.to_string()),
    ];
    let _ = sparrow::db::Connection::builder()
        .await
        .execute(
            "INSERT INTO inbox(
                valid_sig,
                sig_header,
                hostname, 
                date,
                body
            ) VALUES(true,?,?,?,?)",
            query_params.as_slice(),
        )
        .await;
    return Ok(());
}
