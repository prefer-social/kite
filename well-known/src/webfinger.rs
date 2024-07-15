use anyhow::Result;
use serde::Serialize;
use spin_sdk::http::{IntoResponse, Params, Request, Response};
use std::collections::HashMap;
use url::Url;

/// webfenger service.  
pub async fn webfinger(
    req: Request,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
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
                    .header(
                        "Content-Type",
                        "application/jrd+json; charset=utf-8",
                    )
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

/// WebFinger struct
#[derive(Debug, Serialize)]
struct Webfinger {
    subject: String,
    aliases: Vec<String>,
    links: Vec<Link>,
}

/// a method that return webfinger json  
pub async fn get_webfinger(acct: &str) -> Result<Option<String>> {
    let at = acct.split("@").collect::<Vec<&str>>();

    let mut username = at[0];
    let hostname = at[1];

    if username == "" {
        username = at[1].split(".").collect::<Vec<&str>>()[0];
    };

    let account_result = sparrow::table::account::Account::get_with_account(
        username.to_string(),
        hostname.to_string(),
    )
    .await;

    // let account = match account_result.unwrap().unwrap().last().unwrap().clone();
    let account = match account_result.unwrap() {
        Some(a) => a.clone(),
        None => return Ok(None),
    };

    let links = Vec::from([
        Link {
            rel: "http://webfinger.net/rel/profile-page".to_string(),
            link_type: Some("text/html".to_string()),
            href: Some(format!("https://{}", hostname)),
            //href: Some(format!("https://{}/@{}", hostname, username)),
            template: None,
        },
        Link {
            rel: "self".to_string(),
            link_type: Some("application/activity+json".to_string()),
            //href: Some(format!("https://{}", hostname)), Mastodon does Not like this
            href: Some(format!("https://{}/users/{}", hostname, username)),
            template: None,
        },
        Link {
            rel: "http://ostatus.org/schema/1.0/subscribe".to_string(),
            link_type: None,
            href: None,
            template: Some(
                "https://dev.prefer.social/authorize_interaction?uri={uri}"
                    .to_string(),
            ),
        },
    ]);

    let webfinger = Webfinger {
        subject: format!("acct:{}", acct.to_string()),
        aliases: Vec::from([
            format!("https://{}", hostname),
            format!("https://{}/actor", hostname),
            format!("https://{}/self", hostname),
            format!("https://{}/@{}", hostname, username),
            format!("https://{}/users/{}", hostname, username),
            format!("https://{}/u/{}", hostname, username),
        ]),
        links: links,
    };

    let json_str = serde_json::to_string(&webfinger)?;

    Ok(Some(json_str))
}
