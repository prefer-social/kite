use anyhow::{bail, Result};
use base64::{engine, engine::general_purpose, Engine as _};
use thiserror::Error;
//use openssl::rsa::Rsa;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
//use openssl::sign::Signer;
//use openssl::hash::MessageDigest;
//use openssl::pkey::PKey;
use ed25519_dalek::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use ed25519_dalek::pkcs8::{DecodePublicKey, EncodePublicKey};
use ed25519_dalek::Signature;
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use ed25519_dalek::{KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};
use rand::rngs::OsRng;

use bech32::{self, FromBase32, ToBase32, Variant};
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use pem::parse;
use pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use std::convert::TryInto;
use std::str;

#[derive(Debug)]
pub enum KeyType {
    Rsa,
    Ed25519,
}

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("Unknown key type")]
    UnknownKeyType,
}

#[derive(Error, Debug)]
#[error("Signature error")]
pub enum SingnatureError {}

pub async fn create_keypair(keytype: KeyType) -> Result<(String, String)> {
    return match keytype {
        KeyType::Rsa => {
            let mut rng = rand::thread_rng();
            let bits = 2048;
            let privkey = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
            let pubkey = RsaPublicKey::from(&privkey);
            Ok((
                privkey.to_pkcs1_pem(LineEnding::LF).unwrap().to_string(),
                pubkey.to_public_key_pem(LineEnding::LF).unwrap(),
            ))
        }
        KeyType::Ed25519 => {
            // Rename Keypair → SigningKey and PublicKey → VerifyingKey
            let mut csprng = OsRng;
            let signing_key: SigningKey = SigningKey::generate(&mut csprng);
            let verifying_key = signing_key.verifying_key();
            let pubkey_pem =
                VerifyingKey::to_public_key_pem(&verifying_key, Default::default()).unwrap();
            let privkey_pem = SigningKey::to_pkcs8_pem(&signing_key, Default::default()).unwrap();
            Ok((privkey_pem.to_string(), pubkey_pem))
        }
    };
}

/*
pub async fn signature(privkey: &str, signed_signature: String) -> Result<String, SingnatureError> {
    //let keypair = PKey::private_key_from_pem(
    //    &*privkey.as_bytes().to_vec()).unwrap();
    // signature = Base64.strict_encode64(
    //             keypair.sign(OpenSSL::Digest::SHA256.new, signed_string)
    // ) in Ruby

    let pem = parse(privkey).unwrap();



    let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(signed_signature.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    // Encode the signature as a base64 string
    let encoded_signature = general_purpose::STANDARD.encode(&signature);
    Ok(encoded_signature)
}

 */

pub async fn bech32_encode(hrp: &str, src: &str) -> String {
    let s: VerifyingKey = DecodePublicKey::from_public_key_pem(src).unwrap();
    bech32::encode(hrp, s.as_bytes().to_base32(), Variant::Bech32).unwrap()
}

pub async fn bech32_decode(src: &str) -> (String, String) {
    let s = src.to_string();
    let (hrp, data, _variant) = bech32::decode(&s).unwrap();
    let d = Vec::<u8>::from_base32(&data).unwrap();
    let d2: [u8; 32] = d.as_slice().try_into().unwrap();
    let d3 = VerifyingKey::from_bytes(&d2).unwrap();
    (
        hrp.to_string(),
        VerifyingKey::to_public_key_pem(&d3, Default::default()).unwrap(),
    )
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ed25519_dalek::pkcs8;
//     use ed25519_dalek::pkcs8::spki::der::Decode;
//     //use nostr::key::SecretKey;

//     #[tokio::test]
//     async fn create_rsa_keypair_test() {
//         let (privkey, pubkey) = create_keypair(KeyType::Rsa).await.unwrap();
//         dbg!(&pubkey);
//         dbg!(&privkey);
//         assert_eq!(privkey.len(), 1679);
//         assert_eq!(pubkey.len(), 451);
//     }

//     #[tokio::test]
//     async fn create_ed25519_keypair_test() {
//         let (privkey, pubkey) = create_keypair(KeyType::Ed25519).await.unwrap();
//         dbg!(&pubkey);
//         dbg!(&privkey);
//         assert_eq!(privkey.len(), 168);
//         assert_eq!(pubkey.len(), 113);
//     }

//     #[tokio::test]
//     async fn create_ed25519_keypair_in_bech32_encode_test() {
//         let (privkey, pubkey) = create_keypair(KeyType::Ed25519).await.unwrap();
//         dbg!(&privkey);
//         dbg!(&pubkey);

//         let encoded = bech32_encode("seungjin", pubkey.as_str());
//         //dbg!(&encoded);
//         //dbg!(bech32_decode(&encoded.as_str()).await);
//     }

//     #[tokio::test]
//     async fn signature_test() {
//         use super::*;
//     }

//     #[tokio::test]
//     async fn foo() {
//         use super::*;
//         let p = "nsec1xvxajydhmquq9u09r5ya07wtyzjh6s5y448auk5jjtr0wtyag60qn0w6ng";
//         let s = "npub1r2dyxxpedec3jkh0fxcw4v2cmca9r7zjmykwmn5jgcf7tcq0tw0qdsg9e5";

//         let (hrp, data, _variant) = bech32::decode(&s).unwrap();
//         let d = Vec::<u8>::from_base32(&data).unwrap();
//         let d2: [u8; 32] = d.as_slice().try_into().unwrap();
//         let d3 = SigningKey::from_bytes(&d2);
//         //let d3 = VerifyingKey::from_bytes(&d2).unwrap();

//         let d4 = SigningKey::to_pkcs8_pem(&d3, Default::default()).unwrap();
//         dbg!(d4.to_string());

//         //VerifyingKey::to_public_key_pem(&d3, Default::default()).unwrap()
//     }
// }
