// https://docs.joinmastodon.org/spec/webfinger/#example

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::str;

use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component,
};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct WebFinger {
    pub subject: String,
    pub aliases: Vec<String>,
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Link {
    pub rel: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl Into<String> for WebFinger {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for WebFinger {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

impl From<String> for WebFinger {
    fn from(acct: String) -> Self {
        let aliases: Vec<String> = Vec::new();
        let links: Vec<Link> = Vec::new();

        WebFinger {
            subject: format!("acct:{}", acct),
            aliases: aliases,
            links: links,
        }
    }
}

impl WebFinger {
    pub async fn query(acct: &str) -> Result<()> {
        let mut acct_splited = acct.split("@").into_iter();
        let username = acct_splited.next().unwrap();
        let host = acct_splited.next().unwrap();

        // https://mastodon.social/.well-known/webfinger?resource=acct:gargron@mastodon.social
        let webfinger_url = format!(
            "https://{}/.well-known/webfiger?resource=acct:{}",
            host, acct
        );

        let request = Request::builder()
            .method(Method::Get)
            .uri(webfinger_url)
            .build();
        let response: Response = spin_sdk::http::send(request).await?;

        let body = str::from_utf8(response.body()).unwrap();

        tracing::debug!(body);

        Ok(())
    }
}

pub async fn ac(account: &str) -> Result<(&str, &str)> {
    let mut acct = account.split("@").into_iter();
    let username = acct.next().unwrap();
    let host = acct.next().unwrap();

    Ok((username, host))
}
