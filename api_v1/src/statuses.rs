// (POST) /api/v1/statuses
// https://docs.joinmastodon.org/methods/statuses/#create
// Returns: Status.
// https://docs.joinmastodon.org/entities/Status/

use anyhow::Result;
use maplit::hashmap;
use serde_json::Value;
use sparrow::mastodon::media::MediaAttachment;
use spin_sdk::http::{Method, Params, Request, Response};
use spin_sdk::redis;
use spin_sdk::sqlite::Value as SV;
use spin_sdk::variables;
use std::str;
use uuid::Uuid;
use comrak::{markdown_to_html, Options};
use chrono::{TimeZone, Utc};

use sparrow::activitypub::apo::{CollectionPage, Create, Note, Replies, RsaSignature2017};
use sparrow::mastodon::{account::Account, application::Application, status::Status};
use sparrow::postbox::Envelop;
use sparrow::utils::get_current_time_in_iso_8601;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Post => post(req, params).await,
        _ => crate::http_responses::notfound().await,
    }
}

// https://docs.joinmastodon.org/methods/statuses/#create
pub async fn post(req: Request, _params: Params) -> Result<Response> {
    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap() {
        sparrow::auth::TokenAuth::InValid => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => Some(userid).unwrap() as i64,
    };

    // Get my actor address

    let a = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT federationId FROM user WHERE id = ?",
            &[SV::Integer(userid)],
        )
        .await;

    let my_actor = a
        .rows()
        .next()
        .unwrap()
        .get::<&str>("federationId")
        .unwrap();

    // a="{\"language\":\"en\",\"media_attributes\":[],\"media_ids\":[],\"status\":\"bffvbb\",\"visibility\":\"public\"}"
    let body_str = str::from_utf8(req.body()).unwrap();
    let body: Value = serde_json::from_str(body_str).unwrap();
    let status = body.get::<&str>("status").unwrap().as_str().unwrap();


    let mut options = Options::default();
    options.extension.autolink = true;
    let status_html = markdown_to_html(status, &options);

    let media_ids: &Value = body.get("media_ids").unwrap();
    let mut media_ids_1 = media_ids.as_array().unwrap();


    let mut media_attachements: Vec<MediaAttachment> = Vec::new();

    for media in media_ids_1 {
        let id = media.as_str().unwrap();
        let ma = sparrow::mastodon::media::MediaAttachment::create(id).await;
        //let c = serde_json::to_string(&b).unwrap();
        media_attachements.push(ma);
    }

    let uuid = Uuid::now_v7().to_string();
    let now = get_current_time_in_iso_8601().await;
    let url = format!("{my_actor}/{uuid}");

    // get my followers;
    let b = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT federationId FROM follower WHERE userID = ?",
            &[SV::Integer(userid)],
        )
        .await;


    for f in b.rows() {
        let recipient = f.get::<&str>("federationId").unwrap();

        // CREATE OBJECTS

        let collection_page = CollectionPage {
            kind: "CollectionPage".to_string(),
            next: format!(
                "{}/statuses/112001985738281244/replies?only_other_accounts=true&page=true",
                my_actor
            ),
            part_of: format!("{}/statuses/112001985738281244/replies", my_actor),
            items: vec![],
        };

        let replies = Replies {
            id: Uuid::now_v7().to_string(),
            kind: "Replies".to_string(),
            first: collection_page,
        };

        let rsa_signature_2017 = RsaSignature2017 {
            kind: "RsaSignature2017".to_string(),
            creator: "".to_string(),
            created: "".to_string(),
            signature_value: "".to_string(),
        };

        let note = Note {
            id: format!("{my_actor}/statuses/{uuid}"),
            kind: "Note".to_string(),
            summary: None,
            in_reply_to: None,
            published: now.clone(),
            url: url.clone(),
            attributed_to: my_actor.to_string(),
            to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
            cc: vec![format!("{recipient}/followers")],
            sensitive: false,
            atom_uri: format!("{my_actor}/statuses/112001985738281244"),
            in_relpay_to_atom_uri: None,
            converation:
                "tag:seungjin.ap.dev.seungjin.net,2024-02-27:objectId=306721066:objectType=Conversation"
                    .to_string(),
            content: status_html.clone(),
            content_map: vec![hashmap! {
              "en".to_string() => status.to_string(),
            }],
            attachment: media_attachements.clone(),
            tag: vec![],
            replies: replies,
            //signature: rsa_signature_2017,
        };

        let create_object = Create {
            context: vec!["https://www.w3.org/ns/activitystreams".to_string()],
            id: format!("{my_actor}/statuses/{uuid}/activity"),
            kind: "Create".to_string(),
            actor: my_actor.to_string(),
            published: now.clone(),
            to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
            cc: vec![format!("{recipient}/followers")],
            object: note,
        };

        let b = serde_json::to_string(&create_object).unwrap();
        tracing::debug!(b);

        // GET all followers:

        let envelop = Envelop {
            address: recipient.to_string(),
            letter: &create_object,
        };

        let payload = serde_json::to_vec(&envelop).unwrap();

        // let address = format!(
        //    "redis://{}@{}:{}",
        //    std::env::var("REDIS_CREDENTIAL").unwrap(),
        //    std::env::var("REDIS_HOST").unwrap(),
        //    std::env::var("REDIS_PORT").unwrap()
        // );

        let address = format!(
            "redis://{}@{}:{}",
            variables::get("redis_credential").unwrap(),
            variables::get("redis_host").unwrap(),
            variables::get("redis_port").unwrap(),
        );
        let conn = redis::Connection::open(&address)?;
        let channel = variables::get("redis_channel").unwrap();
        let _ = conn.publish(channel.as_str(), &payload);
    }
  
    // PASS TO POSTBOX

    let status = Status {
        id: uuid.clone(),
        created_at: now,
        in_reply_to_id: None,
        in_reply_to_account_id: None,
        sensitive: false,
        spoiler_text: None,
        visibility: "public".to_string(),
        language: "en".to_string(),
        uri: format!("https://mastodon.social/users/Gargron/statuses/{uuid}"),
        url: format!("https://mastodon.social/@Gargron/{uuid}"),
        replies_count: 5,
        reblogs_count: 6,
        favourites_count: 11,
        favourited: false,
        reblogged: false,
        muted: false,
        bookmarked: false,
        content: status_html.clone(),
        reblog: None,
        application: Application {
            name: Some("web".to_string()),
            website: None,
        },
        account: Account {
            id: "1".to_string(),
            username: "seungjin".to_string(),
            acct: "seungjin@seungjin.ap.dev.seungjin.net".to_string(),
            display_name: "Seungjin Kim".to_string(),
            locked: false,
            bot: false,
            discoverable: true,
            group: false,
            created_at: Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap(),
            note: "<p>FOOOFOOOFFOOO</p>".to_string(),
            url: "https://mastodon.social/@Gargron".to_string(),
            avatar: "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg".to_string(),
            avatar_static: "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg".to_string(),
            header: "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png".to_string(),
            header_static: "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png".to_string(),
            followers_count: 322930,
            following_count: 459,
            statuses_count: 61323,
            last_status_at: Some(Utc.with_ymd_and_hms(2015, 5, 15, 0, 0, 0).unwrap()),
            emojis: None,
            fields: None, 
        },
        media_attachments: Some(media_attachements),
        mentions: None,
        tags: None,
        emojis: None,
        card: None, 
        poll: None,
    };



    let json_str = serde_json::to_string(&status).unwrap();
    




    Ok(
        Response::builder()
        .status(200)
        .header("Content-Type", "application/activity+json")
        .build()
    )
}
