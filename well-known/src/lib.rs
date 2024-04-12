use anyhow::Result;
use itertools::Itertools;
use serde::Serialize;
use spin_sdk::{
    http::{IntoResponse, Request, Response, Router, Params},
    http_component,
    sqlite::{QueryResult, Value},
};
use std::collections::HashMap;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;
use url::Url;

#[http_component]
async fn handle_route(req: Request) -> Response {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let request_path_and_query = req.path_and_query().unwrap();
    let request_method = req.method().to_string();
    tracing::debug!(
        "<---------- ({request_method}) {request_path_and_query} --------->"
    );

    let mut router = Router::new();

    router.get_async("/.well-known/webfinger", webfinger);
    router.get_async("/.well-known/host-meta", hostmeta);

    //router.any_async("/", foo::root);
    router.handle_async(req).await
}


async fn webfinger(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    
    let from = req.header("spin-client-addr").unwrap().as_str().unwrap();
    tracing::debug!("-> Webfinger requested from: {from}");

    let k = req.query();
    tracing::debug!("{k}");

    let parsed_url = Url::parse(req.uri())?;

    let hash_query: HashMap<String, String> =
        parsed_url.query_pairs().into_owned().collect();

    // TODO: This gets Error, no resource in query when I search the server in Mobile App.
    let resource = hash_query.get("resource").unwrap();
    let mut ww = resource.split(":");
    if ww.next().unwrap() == "acct" {
        match get_webfinger(ww.next().unwrap()).await.unwrap() {
            Some(wf) => {
                return Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/jrd+json")
                    .body(wf)
                    .build())
            }
            None => {}
        };
    }
    Ok(Response::builder()
        .status(404)
        .header("content-type", "text/html")
        .build())
}

async fn hostmeta(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    
    let from = req.header("spin-client-addr").unwrap().as_str().unwrap();
    tracing::debug!("-> host-meta requested from: {from}");

    let host: Url = req.uri().parse().unwrap();
        
    let a = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
    <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource={{uri}}"/>
    </XRD>"#,host.host().unwrap());
        
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/xrd+xml")
        .body(a)
        .build())
}

#[derive(Debug, Serialize)]
struct Link {
    rel: String,
    #[serde(
        default,
        rename = "type",
        skip_serializing_if = "Option::is_none"
    )]
    link_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    template: Option<String>,
}

#[derive(Debug, Serialize)]
struct Webfinger {
    subject: String,
    aliases: Vec<String>,
    links: Vec<Link>,
}

pub async fn get_webfinger(acct: &str) -> Result<Option<String>> {
    let at = acct.split("@").collect::<Vec<&str>>();

    let mut username = at[0];
    let hostname = at[1];

    if username == "" {
        username = at[1].split(".").collect::<Vec<&str>>()[0];
    }

    let aliases_rowset = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT 
                    ws.username, 
                    ws.hostname, 
                    wa.value
                FROM webfinger_subject AS ws 
                LEFT OUTER JOIN webfinger_aliase AS wa 
                ON wa.subjectId = ws.id 
                WHERE ws.username = ? AND ws.hostname = ? 
                ORDER BY wa.value",
            &[
                Value::Text(username.to_string()),
                Value::Text(hostname.to_string()),
            ],
        )
        .await;

    // Getting subject
    let subject = format!("acct:{username}@{hostname}");

    let r1 = aliases_rowset
        .rows()
        .map(|row| row.get::<&str>("value").unwrap());
    let aliases: Vec<String> = r1.unique().map(|e| e.to_string()).collect();

    if aliases.len() == 0 {
        return Ok(None);
    }

    // Getting links

    let links_rowset = sparrow::db::Connection::builder()
        .await
        .execute(
            r#"SELECT 
                    wl.linkId,
                    wl.key,
                    wl.value
                FROM webfinger_subject AS ws
                LEFT OUTER JOIN webfinger_link AS wl ON ws.id = wl.subjectId
                WHERE ws.username = ? AND ws.hostname = ?"#,
            &[
                Value::Text(username.to_string()),
                Value::Text(hostname.to_string()),
            ],
        )
        .await;

    let r2 = links_rowset
        .rows()
        .map(|row| row.get::<&str>("linkId").unwrap());

    let unique_linkids: Vec<String> =
        r2.unique().map(|e| e.to_string()).collect();

    fn query_tool(
        qr: &QueryResult,
        linkId: &str,
        key: &str,
    ) -> Option<String> {
        let mut val = String::new();
        for row in qr.rows() {
            if row.get::<&str>("linkId").unwrap() == linkId {
                if row.get::<&str>("key").unwrap() == key {
                    val = row.get::<&str>("value").unwrap().to_string();
                }
            }
        }
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    let mut links: Vec<Link> = Vec::new();
    for uniq_id in unique_linkids {
        let rel = query_tool(&links_rowset, uniq_id.as_str(), "rel");
        let href = query_tool(&links_rowset, uniq_id.as_str(), "href");
        let link_type = query_tool(&links_rowset, uniq_id.as_str(), "type");
        let template = query_tool(&links_rowset, uniq_id.as_str(), "template");

        let link = Link {
            rel: rel.unwrap(),
            link_type: link_type,
            href: href,
            template: template,
        };

        links.push(link)
    }

    //println!("==============================");

    //let links: Vec<String> = r2.unique().map(|e| e.to_string()).collect();

    //let links = vec!["r".to_string(), "r".to_string()];

    let wf = Webfinger {
        subject: subject,
        aliases: aliases,
        links: links,
    };

    let json_str = serde_json::to_string(&wf)?;
    //println!("{json_str}");

    Ok(Some(json_str))
}

/*
SELECT username, hostname FROM webfinger_subject WHERE type = 'acct' and username = 'seungjin' and hostname = 'kite.seungjin.net';

SELECT ws.username, ws.hostname, wa.value FROM webfinger_subjects AS ws LE
FT JOIN webfinger_aliases AS wa ON wa.subjectId = ws.id;

SELECT ws.username, ws.hostname, wa.value FROM webfinger_subjects AS ws LEFT OUTER JOIN webfinger_aliases AS wa ON wa.subjectId = ws.id WHERE ws.username = 'seungjin' AND ws.hostname = 'kite.seungjin.net';

SELECT ws.username, ws.hostname, wa.value, wl.linkId, wl.key, wl.value FROM webfinger_subjects AS ws LEFT OUTER JOIN webfinger_aliases AS wa ON wa.subjectId = ws.id LEFT OUTER JOIN webfinger_links AS wl ON wl.subjectId = ws.id WHERE ws.username = 'seungjin' AND ws.hostname = 'kite.seungjin.net';

SELECT ws.username, ws.hostname, wa.value, wl.linkId, wl.key, wl.value FROM webfinger_subjects AS ws LEFT OUTER JOIN webfinger_aliases AS wa ON wa.subjectId = ws.id LEFT OUTER JOIN webfinger_links AS wl ON wl.subjectId = ws.id WHERE ws.username = 'seungjin' AND ws.hostname = 'kite.seungjin.net' order by wa.value, wl.linkId;
*/

pub async fn get_http_headers_map(req: &Request) {
    let headers = req.headers();
    for header in headers {
        println!("{header:?}");
    }
}
