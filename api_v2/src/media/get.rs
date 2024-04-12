use std::thread::spawn;

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;

use sparrow::mastodon::media::{MediaAttachment, MediaType};

// https://docs.joinmastodon.org/methods/media/#get
pub async fn get(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("Requeted -> GET /api/v2/media");

    let media_id = params.get("id").unwrap();

    let userid: i64 = match sparrow::auth::check_api_auth(&req).await.unwrap()
    {
        sparrow::auth::TokenAuth::InValid => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::TokenNotProvided => {
            return crate::http_responses::unauthorized().await;
        }
        sparrow::auth::TokenAuth::Valid(userid) => {
            Some(userid).unwrap() as i64
        }
    };

    let media = sparrow::db::Connection::builder()
        .await
        .execute(
            "SELECT * FROM media_attachement WHERE id = ?",
            &[SV::Text(media_id.to_string())],
        )
        .await;

    //let media_attachement: Vec<_> = media
    //    .rows()
    //    .map(|row|

    let row = media.rows().next().unwrap();

    let media_attachement = MediaAttachment {
        id: row.get::<&str>("id").unwrap().to_string(),
        kind: MediaType::Image,
        url: None,
        preview_url: Some(row.get::<&str>("previewUrl").unwrap().to_string()),
        remote_url: None,
        text_url: None,
        meta: None,
        description: None,
        blurhash: None,
        created_at: None,
        updated_at: None,
    };

    let body = serde_json::to_string(&media_attachement)?;

    tracing::debug!(body);

    /*

       let todos: Vec<_> = rowset.rows().map(|row|
        ToDo {
            id: row.get::<u32>("id").unwrap(),
            description: row.get::<&str>("description").unwrap().to_owned(),
            due: row.get::<&str>("due").unwrap().to_owned(),
        }
    ).collect();

    let body = serde_json::to_vec(&todos)?;

     */

    return Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(body)
        .build());
}
