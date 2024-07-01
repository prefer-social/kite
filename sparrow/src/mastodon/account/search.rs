use anyhow::Result;
use spin_sdk::http::{Method, Request, Response};
use std::str;

use crate::mastodon::account::Account;

impl Account {
    pub async fn search(search_term: &String) -> Result<Vec<Account>> {
        let webfinger =
            crate::webfinger::WebFinger::query(search_term).await?;

        let links = webfinger.unwrap().links;

        let mut link_type = None;
        let mut href = None;
        for link in links.iter() {
            if link.rel == "self" {
                link_type = link.link_type.to_owned();
                href = link.href.to_owned();
            }
        }

        tracing::debug!("{:?}", link_type);
        tracing::debug!("{:?}", href);

        Self::get_actor(href.unwrap(), link_type.unwrap()).await;

        Ok(Vec::new())
    }

    pub async fn get_actor(actor_url: String, ct: String) {
        let request = Request::builder()
            .method(Method::Get)
            .header("Accept", ct)
            .uri(actor_url)
            .build();
        let response: Response = spin_sdk::http::send(request).await.unwrap();
        let status = response.status();
        let ct = response.header("content-type").unwrap().as_str().unwrap();

        let body = str::from_utf8(response.body()).unwrap();

        // Convert this to ActivityPub Actor
        let actor: crate::activitypub::person_actor::PersonActor =
            serde_json::from_str(body).unwrap();

        tracing::debug!("{:?}", actor);
    }
}
