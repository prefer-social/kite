use anyhow::Result;
use multipart_2021::server::Multipart;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;

use std::io::prelude::*;
use uuid::Uuid;

use crate::media::{allowed_mime_type, get_multipart_boundary};
use frameshop::preview_image;
use sparrow::mastodon::media::{MediaAttachment, MediaType};

pub async fn post(req: Request, params: Params) -> Result<Response> {
    tracing::debug!("requested -> POST /api/v2/media");

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

    let boundary = get_multipart_boundary(&req).unwrap(); // TODO: Match its error

    let body = req.body();

    let l = body.len();
    println!("body size: {}", l);

    let mut mp: Multipart<&[u8]> = Multipart::with_body(body, boundary);

    while let Some(field) = mp.read_entry().unwrap() {
        let data: Result<Vec<u8>, std::io::Error> =
            field.data.bytes().collect();
        let a = data.unwrap();
        let b: &[u8] = &a;
        let headers = &field.headers;
        if let Some(mime) = &headers.content_type {
            tracing::debug!("headers: {:?}", field.headers);
            let file_name = Uuid::now_v7().to_string();
            tracing::debug!(file_name);

            if let Some(ext) = allowed_mime_type(mime).unwrap() {
                let file_name_with_extention =
                    format!("{}.{}", file_name, ext);
                let preview_file_name_with_extention =
                    format!("{}.preview.{}", file_name, ext);

                //let b1 = preview_image(b).await.unwrap();
                if b.len() > 500000 {
                    tracing::debug!("Image should be resized for preview");
                };

                // if b / image size is less than XXX, lets resize preview img.

                let public_url =
                    sparrow::storage::send_to_s3(file_name_with_extention, b)
                        .await
                        .unwrap();
                //let preview_url =
                //   sparrow::storage::send_to_s3(preview_file_name_with_extention, b1.as_slice())
                //        .await
                //       .unwrap();

                let preview_url = public_url.clone();

                tracing::debug!(public_url);
                tracing::debug!(preview_url);

                let _ = sparrow::db::Connection::builder().await.execute("INSERT INTO media_attachement(id,userId,type,url,previewUrl) VALUES (?,?,?,?,?)" 
                    , &[SV::Text(file_name.clone()), SV::Integer(userid), SV::Text("image".to_string()), SV::Text(public_url.clone()), SV::Text(public_url.clone()) ]).await;

                //time to create https://docs.joinmastodon.org/entities/MediaAttachment/
                let media_attachement = MediaAttachment {
                    id: file_name,
                    kind: MediaType::Image,
                    url: Some(public_url),
                    preview_url: Some(preview_url),
                    remote_url: None,
                    text_url: None,
                    meta: None,
                    description: None,
                    blurhash: None,
                    created_at: None,
                    updated_at: None,
                };

                let media_attachement_json =
                    serde_json::to_vec(&media_attachement).unwrap();
                let maj =
                    std::str::from_utf8(&media_attachement_json).unwrap();
                tracing::debug!("{maj}");

                return Ok(Response::builder()
                    .status(200)
                    .header("Context-Type", "application/activity+json")
                    .body(media_attachement_json)
                    .build());
            }
        }
    }

    return Ok(Response::builder()
        .status(500)
        .header("Context-Type", "application/activity+json")
        .body("ars".to_string())
        .build());
}
