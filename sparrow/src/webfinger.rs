// https://docs.joinmastodon.org/spec/webfinger/#example

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

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

        WebFinger{
            subject: format!("acct:{}",acct),
            aliases: aliases,
            links: links,
        }

    }
}




