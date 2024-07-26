
use anyhow::Result;
use spin_sdk::http::{Method, Params, Request, Response};

use sparrow::mastodon::account::Account as MAccount;
pub struct Authentication;

impl Authentication {
    pub async fn verify(req: Request) -> Option<MAccount> {
        match req.header("authorization") {
            Some(a) => {
                let auth_header_string = a.as_str().unwrap();
                let mut auth_info = auth_header_string.split(" ").into_iter();

                let auth_type = auth_info.next().unwrap();
                let auth_token = auth_info.next().unwrap();

                match sparrow::mastodon::token::Token::validate(
                    auth_type.to_string(),
                    auth_token.to_string(),
                ).await {
                    Ok(a) => {
                        if a.is_none() {
                            return None;
                        }
                        a
                    },
                    Err(r) => {
                        None
                    }
                }



            }
            None => {
                None
            }
        }
    }
}

pub struct Authorization;