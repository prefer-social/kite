// https://docs.joinmastodon.org/methods/accounts/#verify_credentials
// GET /api/v1/accounts/verify_credentials HTTP/1.1
// Returns: CredentialAccount (https://docs.joinmastodon.org/entities/Account/#CredentialAccount)

use anyhow::Result;
use sparrow::http_response::HttpResponse;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use url::Url;
use sparrow::mastodon::account::Get;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

// TODO: After basic OAUTH, app is calling here with "/app/v1/accounts/verify_credentials"
// https://docs.joinmastodon.org/methods/accounts/#verify_credentials
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!("requested -> {} {}", req.method().to_string(), req.path_and_query().unwrap());

    let account = match req.header("authorization") {
        Some(a) => {

            let auth_header_string = a.as_str().unwrap();
            let mut auth_info = auth_header_string.split(" ").into_iter();

            let auth_type = auth_info.next().unwrap();
            let auth_token = auth_info.next().unwrap();

            let account = sparrow::mastodon::token::Token::validate(auth_type.to_string(),auth_token.to_string()).await?;
            if account.is_none() {
                let msg =  r#"{ "error": "This method requires an authenticated user"}"#.to_string();
                return Ok(Response::builder()
                    .status(442)
                    .header("Content-Type", "Application/json")
                    .body(msg)
                    .build());
            }
            account
        },
        None => {
            let msg =  r#"{ "error": "This method requires an authenticated user"}"#.to_string();
            return Ok(Response::builder()
                .status(442)
                .header("Content-Type", "Application/json")
                .body(msg)
                .build());
        }
    };

    // Should return https://docs.joinmastodon.org/entities/Account/#CredentialAccount
    let credential_account = sparrow::mastodon::account::Account::from(&account.unwrap());
    let ca: String = credential_account.into();

    tracing::debug!("veryfy_credentials passord.");
    tracing::debug!("account response / json returned");
    tracing::debug!(ca);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "Application/json")
        .body(ca)
        .build())

}
