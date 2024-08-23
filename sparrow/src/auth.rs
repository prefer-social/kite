use anyhow::{anyhow, Result};
use spin_sdk::{http::Request, sqlite::Value as SV};
use tracing::{debug, info};

pub enum TokenAuth {
    Valid(u32),
    InValid,
    TokenNotProvided,
}

// If it is valid user, return with UserId
pub async fn check_api_auth(req: &Request) -> Result<TokenAuth> {
    let token = match req.header("Authorization") {
        Some(x) => x.as_str().unwrap(),
        None => return Ok(TokenAuth::TokenNotProvided),
    };
    let mut a = token.split_whitespace();
    let token_type = a.next().unwrap().to_string();
    let token = a.next().unwrap().to_string();

    let a = crate::db::Connection::builder()
        .await
        .execute(
            "SELECT userId FROM auth_token WHERE token = ? AND token_type = ? AND valid = 1",
            &[SV::Text(token), SV::Text(token_type)],
        )
        .await;

    if a.rows.len() == 1 {
        debug!("{a:?}");
        let userid = a.rows().next().unwrap().get::<i32>("userId").unwrap();
        return Ok(TokenAuth::Valid(userid as u32));
    } else {
        return Ok(TokenAuth::InValid);
    }
}
