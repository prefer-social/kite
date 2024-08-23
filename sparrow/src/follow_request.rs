use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1v15::{Signature, SigningKey};
use rsa::pkcs8::DecodePrivateKey;
use rsa::sha2::{Digest, Sha256};
use rsa::signature::SignatureEncoding;
use rsa::signature::Signer;
use rsa::RsaPrivateKey;
use serde_json::{json, Value};
use spin_sdk::http::{
    self, IncomingResponse, IntoResponse, Method, Params, Request,
    RequestBuilder, Response,
};
use spin_sdk::sqlite::Value as SV;
use tracing::{debug, info};
use url::Url;
use uuid::Uuid;

use crate::utils::get_current_time_in_rfc_1123;

pub async fn following_request(
    my_actor: Url,
    recipient_actor: Url,
    private_key_pem: String,
) -> Result<()> {
    //let my_actor = Url::parse("https://ap.dev.seungjin.net/users/seungjin").unwrap();
    //let recipient_actor = Url::parse("https://mas.to/users/seungjin").unwrap();

    // TODO: Create Actor/User object/struct and get key from there
    // Temporary. It should be get from Actor/User object/struct later

    let uuid = Uuid::now_v7().to_string();
    let id = format!(
        "{}://{}/{}",
        String::from(my_actor.scheme()),
        String::from(my_actor.host_str().unwrap()),
        uuid
    );
    // TODO: user name should get from actor/webfinger, not from url parsing
    let user = my_actor
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap()
        .last()
        .unwrap()
        .clone();

    let date = get_current_time_in_rfc_1123().await;
    let content_type = "application/activity+json".to_string();

    debug!("date -> {date}");

    // TODO: This should be created from activity_stream crate not from string literal.
    let request_body: Value = json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": id,
        "type": "Follow",
        "actor": String::from(my_actor.clone()),
        "object": String::from(recipient_actor.clone())
    });

    debug!("request_body -> {request_body}");

    //postman::sender::post_inbox(request_body.to_string());

    let mut hasher = Sha256::new();
    hasher.update(request_body.to_string());
    let digest = format!(
        "SHA-256={}",
        general_purpose::STANDARD.encode(hasher.finalize())
    );
    debug!("digest --> {digest}");

    let recipient_server: &str = recipient_actor.host_str().unwrap();

    // FIXME: Doing this becaues Spin add port into its Host header value
    let hostname = recipient_server.to_string();
    //if recipient_actor.scheme() == "https" {
    //    hostname = format!("{}:443", hostname);
    //}

    // TODO: inbox path should get from actor (actor created from http request)
    let inbox_path = format!("{}/inbox", recipient_actor.path());
    //let inbox_path = "/inbox";
    let signature_string = format!(
        "(request-target): post {}\nhost: {}\ndate: {}\ndigest: {}\ncontent-type: {}",
        inbox_path, hostname, date, digest, content_type
    );
    debug!("signature_string --> \n{signature_string}");

    // The signature string is constructed using the values of the HTTP headers defined in headers, joined by newlines. Typically, you will want to include the request target, as well as the host and the date. Mastodon assumes Date: header if none are provided. For the above GET request, to generate a Signature: with headers="(request-target) host date"
    // https://github.com/RustCrypto/RSA/issues/341
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem)
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
        my_actor.clone(),
        encoded_signature
    );

    debug!("sig_header --> {sig_header}");

    let request = RequestBuilder::new(
        Method::Post,
        format!("{}/inbox", recipient_actor),
    ) // TODO: recipient uri should get from actor.
    .header("Date", date)
    .header("Signature", sig_header)
    .header("Digest", digest)
    .header("Content-Type", &content_type)
    .header("Accept", &content_type)
    .body(request_body.to_string())
    .build();
    let response: IncomingResponse = http::send(request).await?;
    let status = response.status();

    let body = String::from_utf8(response.into_body().await.unwrap()).unwrap();
    debug!("status --> {status}");
    debug!("response body -->\n{body}");

    // INSERT INTO DB
    if status == 202u16 {
        let object = json!({
            "id": id,
            "type": "Follow",
            "actor": String::from(my_actor.clone()),
            "object": String::from(recipient_actor.clone())
        });
        let _ = crate::db::Connection::builder()
            .await
            .execute(
                r#"INSERT OR IGNORE INTO following(userId, federationID, object) VALUES((select id from user where federationId = ?),?,json(?))"#,
                &[
                    SV::Text(my_actor.to_string()),
                    SV::Text(recipient_actor.to_string()),
                    SV::Text(object.to_string()),
                ],
            )
            .await;
    }

    Ok(())
}
