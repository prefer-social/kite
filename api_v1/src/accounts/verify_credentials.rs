// https://docs.joinmastodon.org/methods/accounts/#verify_credentials
// GET /api/v1/accounts/verify_credentials HTTP/1.1
use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => sparrow::http_response::HttpResponse::not_found().await,
        //_ => get(req, params).await,
    }
}

// TODO: After basic OAUTH, app is calling here with "/app/v1/accounts/verify_credentials"
// https://docs.joinmastodon.org/methods/accounts/#verify_credentials
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap()
    {
        sparrow::auth::TokenAuth::InValid => {
            return sparrow::http_response::HttpResponse::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return sparrow::http_response::HttpResponse::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => {
            Some(userid).unwrap() as i64
        }
    };

    let user = sparrow::db::Connection::builder()
        .await
        .execute("SELECT * FROM user WHERE id = ?", &[SV::Integer(userid)])
        .await;

    let username = user.rows().next().unwrap().get::<&str>("name").unwrap();
    let federationId = user
        .rows()
        .next()
        .unwrap()
        .get::<&str>("federationId")
        .unwrap();
    let displayname = user
        .rows()
        .next()
        .unwrap()
        .get::<&str>("displayName")
        .unwrap();
    let locked = false;
    let image_location = user
        .rows()
        .next()
        .unwrap()
        .get::<&str>("imageLocation")
        .unwrap();
    let icon_location = user
        .rows()
        .next()
        .unwrap()
        .get::<&str>("iconLocation")
        .unwrap();

    let follower_qr = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT COUNT(*) AS A FROM follower WHERE userId = ?",
            &[SV::Integer(userid)],
        )
        .await;
    let followers_count =
        follower_qr.rows().next().unwrap().get::<i32>("A").unwrap();

    let following_qr = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT COUNT(*) AS A FROM following WHERE userId = ?",
            &[SV::Integer(userid)],
        )
        .await;
    let followings_count =
        following_qr.rows().next().unwrap().get::<i32>("A").unwrap();

    let a = format!(
        r#"{{
        "id": "{userid}",
        "username": "{username}",
        "acct": "{federationId}",
        "display_name": "{displayname}",
        "locked": {locked},
        "bot": false,
        "created_at": "2016-11-24T10:02:12.085Z",
        "note": "<p>THIS IS NOTE FILED</p>",
        "url": "{federationId}",
        "avatar": "{icon_location}",
        "avatar_static": "{icon_location}",
        "header": "{image_location}",
        "header_static": "{image_location}",
        "followers_count": {followers_count},
        "following_count": {followings_count},
        "statuses_count": 1,
        "last_status_at": "2019-11-24T15:49:42.251Z",
        "emojis": [
          {{
            "shortcode": "fatyoshi",
            "url": "https://files.mastodon.social/custom_emojis/images/000/023/920/original/e57ecb623faa0dc9.png",
            "static_url": "https://files.mastodon.social/custom_emojis/images/000/023/920/static/e57ecb623faa0dc9.png",
            "visible_in_picker": true
          }}
        ],
        "fields": []
      }}"#
    );

    let j: serde_json::Value = serde_json::from_str(a.as_str()).unwrap();

    Ok(Response::builder()
        .status(404)
        .header("Content-Type", "Application/json")
        .body(j.to_string())
        .build())
}
