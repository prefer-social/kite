//! WebFinger library.
//!
//! Mastodon Doc: <https://docs.joinmastodon.org/spec/webfinger/#example>

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use spin_sdk::http::{Method, Request, Response};
use std::str;

/// WebFinger Struct
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct WebFinger {
    /// WebFinger subject
    pub subject: String,
    /// WebFinger alaises
    pub aliases: Vec<String>,
    /// WebFinger Link
    pub links: Vec<Link>,
}

/// Link struct for WebFinger
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Link {
    /// rel filed
    pub rel: String,
    /// type filed
    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// href field
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// template field
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
    /// WebFinger query/request
    /// Requesting `https://{domain}/.well-known/webfinger?resource=acct:seungjin@mas.to`
    pub async fn query(acct: &str) -> Result<Option<WebFinger>> {
        let mut acct_splited = acct.split("@").into_iter();
        let username = acct_splited.next().unwrap();
        let domain = acct_splited.next().unwrap();

        let webfinger_url = format!(
            "https://{}/.well-known/webfinger?resource=acct:{}",
            domain, acct
        );

        tracing::debug!("----------------------------------");

        tracing::debug!("----------------------------------");

        tracing::debug!("Requesting webfinger: {}", webfinger_url);

        let request = Request::builder()
            .method(Method::Get)
            .uri(webfinger_url)
            .header("User-Agent", "prefer.social")
            .build();
        let response: Response = spin_sdk::http::send(request).await?;

        tracing::debug!("Request response: {}", response.status());

        if response.status().to_owned() != 200u16 {
            return Ok(None);
        }
        let _ct = response.header("content-type").unwrap().as_str().unwrap();

        let body = str::from_utf8(response.body()).unwrap();
        let webfinger: WebFinger = serde_json::from_str(body).unwrap();

        tracing::debug!("WebFinger struct: {:?}", webfinger);

        Ok(Some(webfinger))
    }
}
