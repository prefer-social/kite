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
use spin_sdk::http::{self, Method, Request, RequestBuilder, Response};
use std::collections::HashMap;
use std::fmt::Debug;
<<<<<<< HEAD
=======

>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
use url::Url;

use crate::activitystream::activity::Activity;
use crate::activitystream::activity::ActivityType;
use crate::activitystream::Execute;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::mastodon::setting::Setting;
use crate::table::activity_log::ActivityLog;
use crate::utils::get_current_time_in_rfc_1123;

pub mod account;
pub mod activity_log;
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
/// This adds new actor to Account.  
/// This adds incoming request to activityLog table.
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
    // tracing::debug!("request_body: {body}");

    //let obj: Activity<Value> = serde_json::from_str(&body)?;
    let body_value: Value = serde_json::from_str(&body)?;
    let activity_type = body_value.get("type").unwrap().as_str().unwrap();
    let actor_url_str = body_value.get("actor").unwrap().as_str().unwrap();

    // Delete request returns true because it can't validate signature with already removed account.
    // Simply returns OK without validating key.
    if activity_type == "Delete" {
        ActivityLog::put(
            sig_header.to_string(),
            hostname.to_string(),
<<<<<<< HEAD
=======
            Some(req.method().to_owned()),
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
            body,
            None,
        )
        .await
        .unwrap();

        return Ok(true);
    }

    let sender_actor_url = ActorUrl::new(actor_url_str.to_string()).unwrap();

    // Add this actor to Account (only if this is not local account)
    let sender_actor = sender_actor_url.actor().await?.clone();
    let site_domain = Setting::domain().await;
    let sender_domain = sender_actor_url
        .clone()
        .0
        .unwrap()
        .host()
        .unwrap()
        .to_string();
    if sender_domain != site_domain {
        sender_actor.store().await?;
    }

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
                s.split_once('=')
                    .and_then(|t| Some((t.0.to_owned(), rem_first_and_last(t.1).to_owned())))
            })
            .collect()
    }
    let sig_header_map = parse_sig_header(sig_header);
    let _key_id = sig_header_map.get("keyId").unwrap();
    let signature = sig_header_map.get("signature").unwrap();
    let _headers = sig_header_map.get("headers").unwrap();
    // TODO: Check algorithm
    let _algorithm = sig_header_map.get("algorithm").unwrap();

    let decoded_signature = general_purpose::STANDARD.decode(signature).unwrap();

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

    let public_key =
        RsaPublicKey::from_public_key_pem(public_key_string).expect("RsaPublicKey creation failed");
    let verifying_key_openssl: VerifyingKey<Sha256> = VerifyingKey::new(public_key.clone());
    let t = Signature::try_from(decoded_signature.as_slice()).unwrap();
    let valid_key = verifying_key_openssl
        .verify(signature_string.as_bytes(), &t)
        .is_ok();
    // TODO: Check the signed request was made within the past 12 hours
    // https://docs.joinmastodon.org/spec/security/#http-verify
    let valid_date = true;

    if valid_key && valid_date {
        ActivityLog::put(
            sig_header.to_string(),
            hostname.to_string(),
<<<<<<< HEAD
=======
            Some(req.method().to_owned()),
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
            body,
            None,
        )
        .await
        .unwrap();
    }

    Ok(valid_key && valid_date)
}

/// Send ActivityPub Object/Message
<<<<<<< HEAD
pub async fn publish_activity_1<T>(activity: Activity<T>) -> Result<u16>
=======
pub async fn post_activity<T>(activity: Activity<T>) -> Result<u16>
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
where
    T: Debug + Serialize + ToString + Execute,
{
    let sender_actor_url_string = activity.actor.clone();

    let recipient_actor_url_string = match activity.activity_type {
        ActivityType::Follow => activity.activity_object.to_string(),
        //ActivityType::Undo => {}
        ActivityType::Accept => {
            let a = activity.activity_object.to_string();
            let b: Value = serde_json::from_str(a.as_str()).unwrap();
            let c = b.get("actor").unwrap().as_str().unwrap();
            c.to_string()
        }
        ob_type => {
            return Err(anyhow::Error::msg(format!(
                "UNKOWN ObjectType {:?}",
                ob_type
            )))
        }
    };

    tracing::debug!("??????????????????????????????");
    tracing::debug!(sender_actor_url_string);
    tracing::debug!(recipient_actor_url_string);

<<<<<<< HEAD
    let sender_actor_url =
        ActorUrl::new(sender_actor_url_string.clone()).unwrap();
=======
    let sender_actor_url = ActorUrl::new(sender_actor_url_string.clone()).unwrap();
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
    let sender_account = MAccount::get(sender_actor_url.clone()).await?;

    let sender_private_key_pem = sender_account.private_key.clone().unwrap();

<<<<<<< HEAD
    let recipient_actor_url =
        ActorUrl::new(recipient_actor_url_string).unwrap();
=======
    let recipient_actor_url = ActorUrl::new(recipient_actor_url_string).unwrap();
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
    let recipient_account = MAccount::get(recipient_actor_url).await?;
    let date = get_current_time_in_rfc_1123().await;
    let content_type = "application/activity+json".to_string();

    let request_body = serde_json::to_string(&activity).unwrap();

    // tracing::debug!("me -> {me}");
    // tracing::debug!("my_actor -> {my_actor}");
    // tracing::debug!("recipient_actor -> {recipient_actor}");
    // tracing::debug!("recipient_server -> {recipient_server}");
    // tracing::debug!("private_key_pem -> {private_key_pem}");
    // tracing::debug!("date -> {date}");
    // tracing::debug!("content_type -> {content_type}");

    let mut hasher = Sha256::new();
    hasher.update(request_body.clone());
    let digest = format!(
        "SHA-256={}",
        general_purpose::STANDARD.encode(hasher.finalize())
    );

    let site_domain = Setting::domain().await;

    let hostname = recipient_account.account_uri.domain.unwrap_or(site_domain);
    let inbox_url = recipient_account.inbox_url.unwrap();
    let inbox_path_url = url::Url::parse(inbox_url.as_str()).unwrap();
    let inbox_path = inbox_path_url.path();
    let signature_string = format!(
<<<<<<< HEAD
         "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
=======
        "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        inbox_path, hostname, date, digest, content_type
    );

    // The signature string is constructed using the values of the HTTP headers defined in headers, joined by newlines. Typically, you will want to include the request target, as well as the host and the date. Mastodon assumes Date: header if none are provided. For the above GET request, to generate a Signature: with headers="(request-target) host date"
    // https://github.com/RustCrypto/RSA/issues/341
    let private_key = RsaPrivateKey::from_pkcs8_pem(&sender_private_key_pem)
        .expect("RsaPrivateKey creation failed");
    let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);
<<<<<<< HEAD
    let signature = <SigningKey<Sha256> as Signer<Signature>>::sign(
        &signing_key,
        signature_string.as_bytes(),
    );
    let encoded_signature =
        general_purpose::STANDARD.encode(signature.to_bytes().as_ref());
=======
    let signature =
        <SigningKey<Sha256> as Signer<Signature>>::sign(&signing_key, signature_string.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(signature.to_bytes().as_ref());
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac

    let sig_header = format!(
        r#"keyId="{}#main-key",algorithm="rsa-sha256",headers="(request-target) host date digest content-type",signature="{}""#,
        sender_actor_url_string, encoded_signature
    );

    let request = RequestBuilder::new(Method::Post, inbox_url)
        .header("Date", date)
        .header("Signature", sig_header.clone())
        .header("Digest", digest)
        .header("Content-Type", &content_type)
        .header("Accept", &content_type)
        .body(request_body.to_string())
        .build();

    let response: Response = http::send(request).await?;
    let status = response.status();

    //if status == 202u16 { // Only 202? or
    ActivityLog::put(
        sig_header.to_string(),
        hostname.to_string(),
<<<<<<< HEAD
=======
        Some(Method::Post),
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        request_body,
        Some(status.to_string()),
    )
    .await
    .unwrap();
    //}ActivityLog

    Ok(*status)
}

<<<<<<< HEAD
pub async fn publish_activity<T>(activity: Activity<T>) -> Result<u16>
where
    T: Debug + Serialize + ToString + Execute,
{
    let sender_actor_url_string = activity.actor.clone();

    let recipient_actor_url_string = match activity.activity_type {
        ActivityType::Follow => activity.activity_object.to_string(),
        //ActivityType::Undo => {}
        ActivityType::Accept => {
            let a = activity.activity_object.to_string();
            let b: Value = serde_json::from_str(a.as_str()).unwrap();
            let c = b.get("actor").unwrap().as_str().unwrap();
            c.to_string()
        }
        ob_type => {
            return Err(anyhow::Error::msg(format!(
                "UNKOWN ObjectType {:?}",
                ob_type
            )))
        }
    };

    tracing::debug!("??????????????????????????????");
    tracing::debug!(sender_actor_url_string);
    tracing::debug!(recipient_actor_url_string);

    let sender_actor_url =
        ActorUrl::new(sender_actor_url_string.clone()).unwrap();
    let sender_account = MAccount::get(sender_actor_url.clone()).await?;

    let sender_private_key_pem = sender_account.private_key.unwrap();

    let recipient_actor_url =
        ActorUrl::new(recipient_actor_url_string).unwrap();
    let recipient_account = MAccount::get(recipient_actor_url).await?;
    let date = get_current_time_in_rfc_1123().await;
    let content_type = "application/activity+json";

    let message = serde_json::to_string(&activity).unwrap();

    // tracing::debug!("me -> {me}");
    // tracing::debug!("my_actor -> {my_actor}");
    // tracing::debug!("recipient_actor -> {recipient_actor}");
    // tracing::debug!("recipient_server -> {recipient_server}");
    // tracing::debug!("private_key_pem -> {private_key_pem}");
    // tracing::debug!("date -> {date}");
    // tracing::debug!("content_type -> {content_type}");

    let (signature, digest) = create_signrature(
        sender_account.inbox_url.unwrap().as_str(),
        &sender_private_key_pem,
        recipient_account.inbox_url.to_owned().unwrap().as_str(),
        &message,
        &date,
        &content_type,
    );

    let request = RequestBuilder::new(
        Method::Post,
        recipient_account.inbox_url.unwrap(),
    )
    .header("Date", date)
    .header("Signature", &signature)
    .header("Digest", digest)
    .header("Content-Type", content_type)
    .header("Accept", content_type)
    .body(message.as_str())
    .build();

    let response: Response = http::send(request).await?;
    let status = response.status();

    //if status == 202u16 { // Only 202? or
    ActivityLog::put(
        signature,
        Setting::domain().await,
        message,
        Some(status.to_string()),
    )
    .await
    .unwrap();
    //}ActivityLog

    Ok(*status)
}

=======
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
pub async fn get_fediverse(request_url: Url) -> Result<Response> {
    // What I need:
    // sender inbox url,
    // sender private key,
    // recipient inbox url,

    // Todo: get from auth
    let (sender, _) = MAccount::default().await?;

    let _sender_inbox = sender.inbox_url.unwrap();
    let sender_priv_key = sender.private_key.unwrap();
    let date = get_current_time_in_rfc_1123().await;
    let content_type = "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";

<<<<<<< HEAD
    let signature = get_signrature(
=======
    let signature = create_get_signrature(
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        "https://dev.prefer.social/self",
        &sender_priv_key,
        &request_url.to_string(),
        &date,
    );

    let request = Request::builder()
        .method(Method::Get)
        .uri(request_url.to_owned())
        .header("Date", date)
        .header("Signature", &signature)
<<<<<<< HEAD
        //.header("Digest", digest)
        //.header("Content-Type", content_type)
        .header("Accept", content_type)
        //.body(message.as_str())
=======
        .header("Accept", content_type)
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        .build();

    let response: Response = spin_sdk::http::send(request).await.unwrap();

    tracing::debug!("$$$$$$$$$$$$$$$$$");
    tracing::debug!("{:?}", request_url);
    tracing::debug!("{:?}", signature);
    tracing::debug!("{:?}", response.status());

    Ok(response)
}

<<<<<<< HEAD
pub fn create_signrature(
=======
pub fn create_post_signrature(
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
    sender_inbox_url: &str,
    sender_private_key_pem: &str,
    recipient_inbox_url: &str,
    message: &String,
    date_in_rfc_1123: &String,
    content_type: &str,
) -> (String, String) {
    let sender = Url::parse(sender_inbox_url).unwrap();
    let recipient = Url::parse(recipient_inbox_url).unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&sender_private_key_pem)
        .expect("RsaPrivateKey creation failed");

    let mut hasher = Sha256::new();
    hasher.update(message);
    let digest = format!(
        "SHA-256={}",
        general_purpose::STANDARD.encode(hasher.finalize())
    );

    let signature_string = format!(
<<<<<<< HEAD
         "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
        sender.path(), sender.domain().unwrap(), date_in_rfc_1123, digest, content_type
    );

    let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);
    let signature = <SigningKey<Sha256> as Signer<Signature>>::sign(
        &signing_key,
        signature_string.as_bytes(),
    );
    let encoded_signature =
        general_purpose::STANDARD.encode(signature.to_bytes().as_ref());
=======
        "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
        sender.path(),
        sender.domain().unwrap(),
        date_in_rfc_1123,
        digest,
        content_type
    );

    let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);
    let signature =
        <SigningKey<Sha256> as Signer<Signature>>::sign(&signing_key, signature_string.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(signature.to_bytes().as_ref());
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac

    let signature = format!(
        r#"keyId="{}#main-key",algorithm="rsa-sha256",headers="(request-target) host date digest content-type",signature="{}""#,
        sender.as_str(),
        encoded_signature
    );

    (signature, digest)
}

<<<<<<< HEAD
pub fn get_signrature(
=======
pub fn create_get_signrature(
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
    sender_actor_url: &str,
    sender_private_key_pem: &str,
    request_url: &str,
    date_in_rfc_1123: &String,
) -> String {
    let sender = Url::parse(sender_actor_url).unwrap();
    let recipient = Url::parse(request_url).unwrap();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&sender_private_key_pem)
        .expect("RsaPrivateKey creation failed");

    let signature_string = format!(
        "(request-target): get {}\nhost: {}\ndate: {}",
        recipient.path(),
        recipient.domain().unwrap(),
        date_in_rfc_1123
    );

    let signing_key: SigningKey<Sha256> = SigningKey::new(private_key);
<<<<<<< HEAD
    let signature = <SigningKey<Sha256> as Signer<Signature>>::sign(
        &signing_key,
        signature_string.as_bytes(),
    );
    let encoded_signature =
        general_purpose::STANDARD.encode(signature.to_bytes().as_ref());

    tracing::debug!(signature_string);
    tracing::debug!("--->sender---> {:?}", sender);

    // keyId="https://my.example.com/actor#main-key",headers="(request-target) host date",signature="Y2FiYW...IxNGRiZDk4ZA=="
    let signature = format!(
=======
    let signature =
        <SigningKey<Sha256> as Signer<Signature>>::sign(&signing_key, signature_string.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(signature.to_bytes().as_ref());

    // keyId="https://my.example.com/actor#main-key",headers="(request-target) host date",signature="Y2FiYW...IxNGRiZDk4ZA=="
    let sig_header = format!(
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
        r#"keyId="{}#main-key",headers="(request-target) host date",signature="{}""#,
        sender.to_string(),
        encoded_signature
    );

<<<<<<< HEAD
    signature
=======
    tracing::debug!(signature_string);
    tracing::debug!(sig_header);

    sig_header
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
}
