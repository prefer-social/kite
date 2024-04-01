use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1v15::Signature;
use rsa::pkcs1v15::VerifyingKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use rsa::RsaPublicKey;
use spin_sdk::http::Request;
use std::collections::HashMap;
use tracing::{debug, info};

// https://github.com/RustCrypto/RSA/issues/341

// TODO: Rename this to signature_verification
// https://docs.joinmastodon.org/spec/security/#http-verify
// https://github.com/mastodon/mastodon/blob/main/app/controllers/concerns/signature_verification.rb
pub async fn validate(req: &Request, public_key_string: &str) -> Result<bool> {
    let hostname = req.header("Host").unwrap().as_str().unwrap();
    let date = req.header("Date").unwrap().as_str().unwrap();
    let sig_header = req.header("Signature").unwrap().as_str().unwrap();
    let digest = req.header("Digest").unwrap().as_str().unwrap();
    let content_type = req.header("Content-Type").unwrap().as_str().unwrap();
    let request_path = req.header("spin-path-info").unwrap().as_str().unwrap();
    let request_method = req.method().to_string();

    debug!("hostname: {hostname}");
    debug!("date: {date}");
    debug!("sig_header: {sig_header}");
    debug!("digest: {digest}");
    debug!("content-type: {content_type}");
    debug!("request_path: {request_path}");
    debug!("request_method: {request_method}");

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
    //

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

    debug!("--> {signature_string}");

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

    Ok(valid_key && valid_date)
}

pub async fn sign(private_key_string: &str) {

  
}
