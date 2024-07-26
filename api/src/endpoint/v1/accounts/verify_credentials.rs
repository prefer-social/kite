// https://docs.joinmastodon.org/methods/accounts/#verify_credentials
// GET /api/v1/accounts/verify_credentials HTTP/1.1
// Returns: CredentialAccount (https://docs.joinmastodon.org/entities/Account/#CredentialAccount)

use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};
use sparrow::http_response::HttpResponse;

use crate::auth::Authentication as Auth;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

// TODO: After basic OAUTH, app is calling here with "/app/v1/accounts/verify_credentials"
// https://docs.joinmastodon.org/methods/accounts/#verify_credentials
pub async fn get(req: Request, _params: Params) -> Result<Response> {
    tracing::debug!(
        "requested -> {} {}",
        req.method().to_string(),
        req.path_and_query().unwrap()
    );

    let account = Auth::verify(req).await;

    // Should return https://docs.joinmastodon.org/entities/Account/#CredentialAccount
    let credential_account =
        sparrow::mastodon::account::Account::from(account.unwrap().to_owned());
    let ca: String = credential_account.try_into().unwrap();

    tracing::debug!("veryfy_credentials password.");
    tracing::debug!("account response / json returned");
    tracing::debug!(ca);

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "Application/json")
        .body(ca)
        .build())
}
