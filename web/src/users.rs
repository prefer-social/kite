use anyhow::Result;
use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{QueryResult, Value as SV},
};
use time::OffsetDateTime;
use url::Url;

use crate::utils::not_found;

pub mod activities;
pub mod followers;
pub mod following;
pub mod inbox;
pub mod outbox;

pub async fn request(req: Request, params: Params) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        Method::Post => post(req, params).await,
        _ => not_found(req, params).await,
    }
}

pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("user info requested");

    // get domain id:
    tracing::debug!("{}", req.uri());

    let mut name: String = match params.get("user") {
        Some(name) => name.to_string(),
        None => {
            let u: Url = req.uri().parse().unwrap();
            let hostname = u.host_str().unwrap();
            let id = hostname.split(".").next().unwrap();
            id.to_string()
        }
    };
    // If parmas has "user" to follow:
    //let mut name = params.get("user").unwrap().to_string();

    name = String::from(if name[..1].to_string() == "@".to_string() {
        name[1..].to_string()
    } else {
        name
    });

    let user_rowset = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT * FROM user WHERE name = ?",
            &[SV::Text(name.to_string())],
        )
        .await;

    if user_rowset.rows().count() == 0 {
        return Ok(Response::builder().status(404).build());
    }

    let users = Users::build(user_rowset);
    let user: User = users.get(0).unwrap().clone();

    let actor: PersonActor = user.to_actor().await;
    let s = serde_json::to_string(&actor)?;

    tracing::debug!(s);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body(s.to_owned())
        .build())
}

pub async fn post(req: Request, params: Params) -> Result<Response> {
    not_found(req, params).await
}

pub async fn outbox(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .body("arsars".to_owned())
        .build())
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: u32,
    federation_id: String,
    name: String,
    display_name: String,
    email: Option<String>,
    url: Url,
    summary: Option<String>,
    local: bool,
    discoverable: bool,
    manually_approves_followers: bool,
    indexable: bool,
    inbox: Url,
    outbox: Url,
    following: Url,
    followers: Url,
    featured: Url,
    featured_tags: Url,
    image_location: Url,
    icon_location: Url,
    published: NaiveDateTime,
    last_refreshed_at: NaiveDateTime,
}

impl User {
    pub async fn get_public_key(&self) -> PublicKey {
        let signing_key_rowset = sparrow::db::Connection::builder()
            .await
            .execute(
                "SELECT publicKey FROM signing_key WHERE userId = ?",
                &[SV::Integer(self.id as i64)],
            )
            .await;
        let public_key_pem = signing_key_rowset
            .rows()
            .next()
            .unwrap()
            .get::<&str>("publicKey")
            .unwrap();

        PublicKey {
            id: format!("{}#main-key", self.federation_id),
            owner: self.federation_id.to_string(),
            public_key_pem: public_key_pem.to_string(),
        }
    }

    pub async fn to_actor(&self) -> PersonActor {
        let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
        let pub_date = self.published.format_with_items(fmt.clone()).to_string();
        let pk = self.get_public_key().await;

        let icon = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: self.icon_location.to_string(),
        };

        let image = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: self.image_location.to_string(),
        };

        PersonActor {
            context: vec![
                "https://www.w3.org/ns/activitystreams".to_string(),
                "https://w3id.org/security/v1".to_string(),
            ],
            id: self.federation_id.to_string(),
            kind: "Person".to_string(),
            following: self.following.to_string(),
            followers: self.followers.to_string(),
            inbox: self.inbox.to_string(),
            outbox: self.outbox.to_string(),
            featured: self.featured.to_string(),
            featured_tags: self.featured_tags.to_string(),
            preferred_username: self.display_name.to_string(),
            name: self.name.to_string(),
            summary: self.summary.to_owned(),
            url: self.url.to_string(),
            manually_approves_followers: self.manually_approves_followers,
            discoverable: self.discoverable,
            indexable: self.indexable,
            published: pub_date,
            memorial: Some(false),
            devices: None,
            public_key: pk,
            tags: vec![Value::Null],
            attachment: vec![Value::Null],
            icon: icon,
            image: image,
        }
    }
}

struct Users;

impl Users {
    pub fn build(rowset: QueryResult) -> Vec<User> {
        fn get_fed_id(u: &str) -> String {
            let a = Url::parse(u).expect("Not Valid Url");
            if a.path() != "/" {
                return a.to_string();
            }

            format!("{}://{}", a.scheme(), a.host().unwrap())
        }

        let users: Vec<User> = rowset
            .rows()
            .map(|row| User {
                id: row.get::<u32>("id").unwrap(),
                federation_id: get_fed_id(row.get::<&str>("federationId").unwrap()),
                name: row.get::<&str>("name").unwrap().to_owned(),
                display_name: row.get::<&str>("displayName").unwrap().to_owned(),
                email: Some(row.get::<&str>("email").unwrap().to_owned()),
                url: Url::parse(row.get::<&str>("url").unwrap()).unwrap(),
                summary: Some(row.get::<&str>("summary").unwrap().to_owned()),
                local: row.get::<bool>("local").unwrap(),
                discoverable: row.get::<bool>("discoverable").unwrap(),
                manually_approves_followers: row.get::<bool>("manuallyApprovesFollowers").unwrap(),
                indexable: row.get::<bool>("indexable").unwrap(),
                inbox: Url::parse(row.get::<&str>("inbox").unwrap().to_owned().as_str()).unwrap(),
                outbox: Url::parse(row.get::<&str>("outbox").unwrap().to_owned().as_str()).unwrap(),
                following: Url::parse(row.get::<&str>("following").unwrap().to_owned().as_str())
                    .unwrap(),
                followers: Url::parse(row.get::<&str>("followers").unwrap().to_owned().as_str())
                    .unwrap(),
                featured: Url::parse(row.get::<&str>("featured").unwrap().to_owned().as_str())
                    .unwrap(),
                featured_tags: Url::parse(
                    row.get::<&str>("featuredTags").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                image_location: Url::parse(
                    row.get::<&str>("imageLocation")
                        .unwrap()
                        .to_owned()
                        .as_str(),
                )
                .unwrap(),
                icon_location: Url::parse(
                    row.get::<&str>("iconLocation").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                published: NaiveDateTime::from_timestamp_opt(
                    row.get::<i64>("published").unwrap(),
                    0,
                )
                .unwrap(),
                last_refreshed_at: NaiveDateTime::from_timestamp_opt(
                    row.get::<i64>("lastRefreshedAt").unwrap(),
                    0,
                )
                .unwrap(),
            })
            .collect();
        users
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonActor {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    pub featured: String,
    pub featured_tags: String,
    pub preferred_username: String,
    pub name: String,
    pub summary: Option<String>,
    pub url: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memorial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<String>,
    pub public_key: PublicKey,
    pub tags: Vec<Value>,
    pub attachment: Vec<Value>,
    pub icon: Image,
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    kind: String,
    media_type: String,
    url: String,
}
