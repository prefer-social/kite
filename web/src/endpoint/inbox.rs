use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use std::collections::HashMap;
use std::str;
use url::Url;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    tracing::debug!("rasars");

    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => sparrow::http_response::HttpResponse::method_not_allowed().await,
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
    // TODO: First thing to do -> Create my actor/user struct based on user name!
    // Reuse one for all

    let mut header_hashmap = HashMap::new();
    for (k, v) in req.headers() {
        let k = k.to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }

    let headers_json_string = serde_json::to_string(&header_hashmap).unwrap();
    let body = String::from_utf8_lossy(req.body()).to_string();

    //let sig_header = req.header("Signature").unwrap().as_str().unwrap();

    let sig_header = match req.header("Signature") {
        Some(s) => s.as_str().unwrap(),
        None => {
            return sparrow::http_response::HttpResponse::not_found().await
        }
    };

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

    let (account, user) = sparrow::mastodon::account::Account::default_user().await?;

    let pubkey_str = account.public_key.unwrap();

    return sparrow::http_response::HttpResponse::not_found().await;

    let valid_signature =
        sparrow::mastodon::validate_mastodon_request(&req, &pubkey_str)
            .await
            .unwrap();

    tracing::debug!("valid signature: {:?}", valid_signature);

    let b = str::from_utf8(req.body()).unwrap();

    if valid_signature {
        tracing::debug!("VALID SIGNATURE");
        tracing::debug!("body ->\n {b}");

        //record_to_inbox(&req, &b).await?;
        //handle_activity(b).await?;

        tracing::debug!("----------arisenasrienariseansrietsrrst----");
    } else {
        tracing::debug!("NOT VALID SIGNATURE");
        return sparrow::http_response::HttpResponse::unauthorized().await;
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
