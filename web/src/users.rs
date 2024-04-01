use activitystreams::{
    actor::{ApActor, Person},
    context, iri,
    iri_string::types::IriString,
    object::{ApObject, Image},
    prelude::*,
    security,
    unparsed::UnparsedMutExt,
};
use activitystreams_ext::{Ext1, Ext2, UnparsedExtension};
use anyhow::Result;
use chrono::NaiveDateTime;
use mime;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value as SV},
};
use time::OffsetDateTime;
use tracing::debug;
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

pub async fn get(_req: Request, params: Params) -> Result<Response> {
    debug!("user info requested");
    let mut name = params.get("user").unwrap().to_string();
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
    let user = users.get(0).unwrap();

    let signing_key_rowset = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT publicKey FROM signing_key WHERE userId = ?",
            &[SV::Integer(user.id as i64)],
        )
        .await;
    let public_key_pem = signing_key_rowset
        .rows()
        .next()
        .unwrap()
        .get::<&str>("publicKey")
        .unwrap();

    let actor = ApActor::new(iri!(user.inbox.to_string()), Person::new());

    let mut person = Ext2::new(
        actor,
        PublicKey {
            public_key: PublicKeyInner {
                id: format!("{}#main-key", user.federation_id).parse()?,
                owner: iri!(user.federation_id.to_string()),
                public_key_pem: public_key_pem.to_owned(),
            },
        },
        MyThing {
            my_thing: "arsars".to_string(),
        },
    );

    let mut image = ApObject::new(Image::new());
    image.set_url(iri!(user.image_location.to_string()));
    image.set_media_type(mime::IMAGE_JPEG);

    let mut icon = ApObject::new(Image::new());
    icon.set_url(iri!(user.icon_location.to_string()));
    icon.set_media_type(mime::IMAGE_JPEG);

    person.set_context(context()).add_context(security());

    person
        .set_context(context())
        .add_context(security())
        .set_id(iri!(user.federation_id.to_string()))
        .set_preferred_username(user.name.to_owned())
        .set_name(user.display_name.to_owned())
        .set_summary(user.summary.to_owned().unwrap())
        .set_url(user.url.to_string())
        .set_outbox(iri!(user.outbox.to_string()))
        .set_following(iri!(user.following.to_string()))
        .set_followers(iri!(user.followers.to_string()))
        .set_published(OffsetDateTime::from_unix_timestamp(user.published.timestamp()).unwrap())
        .set_image(image.into_any_base().unwrap())
        .set_icon(icon.into_any_base().unwrap());
    //.set_icon(icon);

    //let any_base = person.into_any_base()?;
    //println!("any_base: {:#?}", any_base);
    //let person = ExtendedPerson::from_any_base(any_base)?;

    let s = serde_json::to_string(&person)?;

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
    federation_id: Url,
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

struct Users;

impl Users {
    pub fn build(rowset: QueryResult) -> Vec<User> {
        let users: Vec<User> = rowset
            .rows()
            .map(|row| User {
                id: row.get::<u32>("id").unwrap(),
                federation_id: Url::parse(row.get::<&str>("federationId").unwrap())
                    .expect("Not Valid Url"),
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyThing {
    my_thing: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    public_key: PublicKeyInner,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyInner {
    id: IriString,
    owner: IriString,
    public_key_pem: String,
}

impl<U> UnparsedExtension<U> for PublicKey
where
    U: UnparsedMutExt,
{
    type Error = serde_json::Error;

    fn try_from_unparsed(unparsed_mut: &mut U) -> Result<Self, Self::Error> {
        Ok(PublicKey {
            public_key: unparsed_mut.remove("publicKey")?,
        })
    }

    fn try_into_unparsed(self, unparsed_mut: &mut U) -> Result<(), Self::Error> {
        unparsed_mut.insert("publicKey", self.public_key)?;
        Ok(())
    }
}

pub type ExtendedPerson = Ext1<ApActor<Person>, PublicKey>;
