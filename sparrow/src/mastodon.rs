//! Mastodon library  
//!
//! Libraries used for Mastodon API and etc  

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1v15::VerifyingKey;
use rsa::pkcs1v15::{Signature, SigningKey};
use rsa::pkcs8::DecodePrivateKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::{Digest, Sha256};
use rsa::signature::SignatureEncoding;
use rsa::signature::Signer;
use rsa::signature::Verifier;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::Serialize;
use serde_json::Value;
use spin_sdk::http::{
    self, IncomingResponse, Method, Request, RequestBuilder,
};
use std::collections::HashMap;
use std::fmt::Debug;
use url::Url;

use crate::activitypub::object::Object as APObject;
use crate::activitypub::object::ObjectType;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::table::inbox_log::InboxLog;
use crate::utils::get_current_time_in_rfc_1123;

pub mod account;
pub mod application;
pub mod custom_emoji;
pub mod filter;
pub mod filter_keyword;
pub mod filter_result;
pub mod filter_status;
pub mod follow;
pub mod instance;
pub mod list;
pub mod media_attachment;
pub mod notification;
pub mod poll;
pub mod preview_card;
pub mod relationship;
pub mod relationship_severance_event;
pub mod report;
pub mod setting;
pub mod status;
pub mod tag;
pub mod token;
pub mod user;
pub mod user_role;

// https://github.com/RustCrypto/RSA/issues/341

/// Validate Mastodon signrature.  
///
/// This also adds new actor to Account.  
/// TODO: Rename this to signature_verification
/// https://docs.joinmastodon.org/spec/security/#http-verify
/// https://github.com/mastodon/mastodon/sender_actor_url_stringblob/main/app/controllers/concerns/signature_verification.rb
pub async fn validate_signature(req: &Request) -> Result<bool> {
    let sig_header = req.header("Signature").unwrap().as_str().unwrap();
    let hostname = req
        .header("Host")
        .unwrap_or(req.header("x-forwarded-host").unwrap())
        .as_str()
        .unwrap();
    let date = req.header("Date").unwrap().as_str().unwrap();
    let digest = req.header("Digest").unwrap().as_str().unwrap();
    let content_type = req.header("Content-Type").unwrap().as_str().unwrap();
    let requested_uri = Url::parse(req.uri())?;
    let request_path = requested_uri.path();
    let request_method = req.method().to_string();
    let body = String::from_utf8_lossy(req.body()).to_string();

    // tracing::debug!("sig_header: {sig_header}");
    // tracing::debug!("hostname: {hostname}");
    // tracing::debug!("date: {date}");
    // tracing::debug!("digest: {digest}");
    // tracing::debug!("content-type: {content_type}");
    // tracing::debug!("request_path: {request_path}");
    // tracing::debug!("request_method: {request_method}");
    tracing::debug!("request_body: {body}");

    let obj: APObject<Value> = serde_json::from_str(&body)?;

    // Delete request returns true because it can't validate signrature - pubkey is already removed.
    if obj.object_type == ObjectType::Delete {
        InboxLog::put(
            sig_header.to_string(),
            sig_header.to_string(),
            hostname.to_string(),
            body,
        )
        .await
        .unwrap();

        return Ok(true);
    }

    let sender_actor_url = ActorUrl::new(obj.actor).unwrap();

    // Add this actor to Account
    let sender_actor = sender_actor_url.actor().await?;
    sender_actor.store().await?;

    let sender_account = MAccount::get(sender_actor_url).await?;
    let public_key_string = sender_account.public_key.as_str();

    fn parse_sig_header(query: &str) -> HashMap<String, String> {
        fn rem_first_and_last(value: &str) -> &str {
            let mut chars = value.chars();
            chars.next();
            chars.next_back();
            chars.as_str()
        }
        query
            .split(',')
            .filter_map(|s| {
                s.split_once('=').and_then(|t| {
                    Some((t.0.to_owned(), rem_first_and_last(t.1).to_owned()))
                })
            })
            .collect()
    }
    let sig_header_map = parse_sig_header(sig_header);
    let _key_id = sig_header_map.get("keyId").unwrap();
    let signature = sig_header_map.get("signature").unwrap();
    let _headers = sig_header_map.get("headers").unwrap();
    // TODO: Check algorithm
    let _algorithm = sig_header_map.get("algorithm").unwrap();

    let decoded_signature =
        general_purpose::STANDARD.decode(signature).unwrap();

    // TODO: Generate signature string based on actual headers info got from sig_headers
    // See this: https://blog.joinmastodon.org/2018/07/how-to-make-friends-and-verify-requests/
    let signature_string = format!(
        "(request-target): {} {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
        request_method.to_lowercase(),
        request_path,
        hostname,
        date,
        digest,
        content_type,
    );

    // tracing::debug!("--> {signature_string}");

    let public_key = RsaPublicKey::from_public_key_pem(public_key_string)
        .expect("RsaPublicKey creation failed");
    let verifying_key_openssl: VerifyingKey<Sha256> =
        VerifyingKey::new(public_key.clone());
    let t = Signature::try_from(decoded_signature.as_slice()).unwrap();
    let valid_key = verifying_key_openssl
        .verify(signature_string.as_bytes(), &t)
        .is_ok();
    // TODO: Check the signed request was made within the past 12 hours
    // https://docs.joinmastodon.org/spec/security/#http-verify
    let valid_date = true;

    if valid_key && valid_date {
        InboxLog::put(
            sig_header.to_string(),
            sig_header.to_string(),
            hostname.to_string(),
            body,
        )
        .await
        .unwrap();
    }

    Ok(valid_key && valid_date)
}

/// Creating HTTP signature  
/// Signing POST requests and the Digest header  
/// Reference this doc: <https://docs.joinmastodon.org/spec/security/#http-sign>  
pub async fn signing_request(req: &Request, _priv_key_string: &str) {
    let _hostname = req.header("Host").unwrap().as_str().unwrap();
    let _date = req.header("Date").unwrap().as_str().unwrap();
    //let sig_header = req.header("Signature").unwrap().as_str().unwrap();
    //let digest = req.header("Digest").unwrap().as_str().unwrap();
    let _content_type = req.header("content-type").unwrap().as_str().unwrap();
    let _request_path =
        req.header("spin-path-info").unwrap().as_str().unwrap();
    let _request_method = req.method().to_string();
}

/// Send ActivityPub Object/Message
pub async fn send<T>(activitypub_obj: APObject<T>) -> Result<u16>
where
    T: Debug + Serialize + ToString,
{
    let sender_actor_url_string = activitypub_obj.actor.clone();

    let recipient_actor_url_string = match activitypub_obj.object_type {
        ObjectType::Follow => activitypub_obj.object.to_string(),
        //AcceptType::Undo => {}
        _ => return Err(anyhow::Error::msg("UNKOWN ObjectType")),
    };

    let sender_actor_url =
        ActorUrl::new(sender_actor_url_string.clone()).unwrap();
    let sender_account = MAccount::get(sender_actor_url).await?;
    let sender_private_key_pem = sender_account.private_key.unwrap();

    let recipient_actor_url =
        ActorUrl::new(recipient_actor_url_string).unwrap();
    let recipient_account = MAccount::get(recipient_actor_url).await?;

    // //let my_actor = format!("{}", Url::parse(me).unwrap().to_string());
    // let recipient_actor = recipient.actor().await?;
    // let recipient_server = recipient_actor.url;

    // let private_key_pem =
    //     get_privatekey_with_actor_url(me.to_string()).await.unwrap();
    let date = get_current_time_in_rfc_1123().await;
    let content_type = "application/activity+json".to_string();

    let request_body = serde_json::to_string(&activitypub_obj).unwrap();

    // tracing::debug!("me -> {me}");
    // //tracing::debug!("my_actor -> {my_actor}");
    // tracing::debug!("recipient_actor -> {recipient_actor}");
    // tracing::debug!("recipient_server -> {recipient_server}");
    // tracing::debug!("private_key_pem -> {private_key_pem}");
    // tracing::debug!("date -> {date}");
    // tracing::debug!("content_type -> {content_type}");

    // // TODO: This should be created from activity_stream crate not from string literal.

    // tracing::debug!("request_body -> {request_body}");

    let mut hasher = Sha256::new();
    hasher.update(request_body.clone());
    let digest = format!(
        "SHA-256={}",
        general_purpose::STANDARD.encode(hasher.finalize())
    );
    // tracing::debug!("digest --> {digest}");

    let hostname = recipient_account.account_uri.domain.unwrap();
    let inbox_url = recipient_account.inbox_url.unwrap();
    let inbox_path_url = url::Url::parse(inbox_url.as_str()).unwrap();
    let inbox_path = inbox_path_url.path();
    let signature_string = format!(
         "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
        inbox_path, hostname, date, digest, content_type
    );
    // tracing::debug!("signature_string --> \n{signature_string}");

    // The signature string is constructed using the values of the HTTP headers defined in headers, joined by newlines. Typically, you will want to include the request target, as well as the host and the date. Mastodon assumes Date: header if none are provided. For the above GET request, to generate a Signature: with headers="(request-target) host date"
    // https://github.com/RustCrypto/RSA/issues/341
    let private_key = RsaPrivateKey::from_pkcs8_pem(&sender_private_key_pem)
        .expect("RsaPrivateKey creation failed");
    let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);
    let signature = <SigningKey<Sha256> as Signer<Signature>>::sign(
        &signing_key,
        signature_string.as_bytes(),
    );
    let encoded_signature =
        general_purpose::STANDARD.encode(signature.to_bytes().as_ref());

    let sig_header = format!(
        r#"keyId="{}#main-key",algorithm="rsa-sha256",headers="(request-target) host date digest content-type",signature="{}""#,
        sender_actor_url_string, encoded_signature
    );

    // tracing::debug!("sig_header --> {sig_header}");

    let request = RequestBuilder::new(Method::Post, inbox_url)
        .header("Date", date)
        .header("Signature", sig_header)
        .header("Digest", digest)
        .header("Content-Type", &content_type)
        .header("Accept", &content_type)
        .body(request_body.to_string())
        .build();

    let response: IncomingResponse = http::send(request).await?;
    let status = response.status();

    //let response_body =
    //    String::from_utf8(response.into_body().await.unwrap()).unwrap();
    //tracing::debug!("status --> {status}");
    //tracing::debug!("response body -->\n{response_body}");

    Ok(status)
}
