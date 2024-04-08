use anyhow::Result;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{Connection, QueryResult, Value},
};
use tracing::debug;

pub async fn request(
    req: Request,
    params: Params,
) -> Result<impl IntoResponse> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => not_found(req, params).await,
    }
}

// TODO: GET /api/v1/instance
pub async fn get(_req: Request, _params: Params) -> Result<Response> {
    let foo = r#"{
      "uri": "seungjin.ap.dev.seungjin.net",
      "title": "AP dev server",
      "short_description": "short description for ap dev server",
      "description": "",
      "email": "seungjin@duck.com",
      "version": "0.0.1",
      "urls": {
        "streaming_api": "wss://seungjin.ap.dev.seungjin.net"
      },
      "stats": {
        "user_count": 1,
        "status_count": 1,
        "domain_count": 1
      },
      "thumbnail": "https://media-mstd.seungjin.net/site_uploads/files/000/000/001/@1x/b3e8da26f8f02054.png",
      "languages": [
        "en"
      ],
      "registrations": false,
      "approval_required": false,
      "invites_enabled": true,
      "configuration": {
        "accounts": {
          "max_featured_tags": 10
        },
        "statuses": {
          "max_characters": 500,
          "max_media_attachments": 4,
          "characters_reserved_per_url": 23
        },
        "media_attachments": {
          "supported_mime_types": [
            "image/jpeg",
            "image/png",
            "image/gif",
            "image/heic",
            "image/heif",
            "image/webp",
            "image/avif",
            "video/webm",
            "video/mp4",
            "video/quicktime",
            "video/ogg",
            "audio/wave",
            "audio/wav",
            "audio/x-wav",
            "audio/x-pn-wave",
            "audio/vnd.wave",
            "audio/ogg",
            "audio/vorbis",
            "audio/mpeg",
            "audio/mp3",
            "audio/webm",
            "audio/flac",
            "audio/aac",
            "audio/m4a",
            "audio/x-m4a",
            "audio/mp4",
            "audio/3gpp",
            "video/x-ms-asf"
          ],
          "image_size_limit": 16777216,
          "image_matrix_limit": 33177600,
          "video_size_limit": 103809024,
          "video_frame_rate_limit": 120,
          "video_matrix_limit": 8294400
        },
        "polls": {
          "max_options": 4,
          "max_characters_per_option": 50,
          "min_expiration": 300,
          "max_expiration": 2629746
        }
      },
      "contact_account": {
        "id": "109737937659013254",
        "username": "seungjin",
        "acct": "seungjin",
        "display_name": "seungjin",
        "locked": false,
        "bot": false,
        "discoverable": false,
        "group": false,
        "created_at": "2023-01-23T00:00:00.000Z",
        "note": "<p>The future is already here <br />- it&#39;s just not very evenly distributed.<br />William Gibson</p>",
        "url": "https://mstd.seungjin.net/@seungjin",
        "uri": "https://mstd.seungjin.net/users/seungjin",
        "avatar": "https://media-mstd.seungjin.net/accounts/avatars/109/737/937/659/013/254/original/626c9187e341632b.jpg",
        "avatar_static": "https://media-mstd.seungjin.net/accounts/avatars/109/737/937/659/013/254/original/626c9187e341632b.jpg",
        "header": "https://media-mstd.seungjin.net/accounts/headers/109/737/937/659/013/254/original/9a714d77de20ae26.jpg",
        "header_static": "https://media-mstd.seungjin.net/accounts/headers/109/737/937/659/013/254/original/9a714d77de20ae26.jpg",
        "followers_count": 95,
        "following_count": 327,
        "statuses_count": 1878,
        "last_status_at": "2023-12-25",
        "noindex": true,
        "emojis": [],
        "roles": [
          {
            "id": "3",
            "name": "Owner",
            "color": ""
          }
        ],
        "fields": [
          {
            "name": "Speaks",
            "value": "English, 한국어",
            "verified_at": null
          },
          {
            "name": "Pronouns",
            "value": "He / His",
            "verified_at": null
          },
          {
            "name": "General interests",
            "value": "Environment, Animal right, Coffee, Tea, Poor man&#39;s food",
            "verified_at": null
          },
          {
            "name": "Technical interests",
            "value": "Rust Programming Language, WebAssembly, Cloud Computing",
            "verified_at": null
          }
        ]
      },
      "rules": [
        {
          "id": "3",
          "text": "Single user(and his bots) server. "
        }
      ]
    }"#;

    let json_val: serde_json::Value = serde_json::from_str(foo).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())
}

pub async fn not_found(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder().status(404).build())
}

pub async fn unauthorized() -> Result<Response> {
    let json_str = r#"{
      "error": "invalid_signature",
      "error_description": "The signature in the request is not valid."
  }"#;
    let json_val: serde_json::Value = serde_json::from_str(json_str).unwrap();

    Ok(Response::builder()
        .status(401)
        .header("Content-Type", "application/json")
        .body(json_val.to_string())
        .build())
}
