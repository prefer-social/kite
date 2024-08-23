use std::thread::yield_now;

use anyhow::Result;
use chrono::{DateTime, Utc};
use hmac::{Mac, SimpleHmac};
use sha2::{Digest, Sha256};
use spin_sdk::{
    http::{Method, Request, Response},
    variables,
};

/*

curl --location --request PUT 'https://cnbbb4fp6bwv.compat.objectstorage.ap-seoul-1.oraclecloud.com/tmp/1x1black.png' \
--header 'Content-Type: image/png' \
--header 'X-Amz-Content-Sha256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855' \
--header 'X-Amz-Date: 20240304T065016Z' \
--header 'Authorization: AWS4-HMAC-SHA256 Credential=433f5f8a64597b8aaf3c3289963e08b45631503d/20240304/ap-seoul-1/s3/aws4_request, SignedHeaders=content-length;content-type;host;x-amz-content-sha256;x-amz-date, Signature=97f2b4fb213899602ac9d5f898d76675d2c717ce92ecd8b0e36afabd6dbc2683' \
--data '@/var/home/seungjin/Downloads/1x1black.png'

*/

// https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html#create-string-to-sign
// https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html

pub async fn send_to_s3(file_name: String, file: &[u8]) -> Result<String> {
    let (
        host,
        bucket,
        endpoint,
        region,
        access_key,
        secret_key,
        public_url_template,
    ) = s3_info().await.unwrap();
    let endpoint = format!("https://{endpoint}/{file_name}");
    let public_url = format!("{}{}", public_url_template, file_name);

    let content_length = file.len().to_string();
    tracing::debug!("--- content length : {}", content_length);

    let service = "s3".to_string();

    let (x_amz_date, ddate) = get_x_amz_date().await;
    let yyyymmdd = x_amz_date[..8].to_string();

    let x_amz_content_sha256 = sha256hash_hex_encoded(file).await.unwrap();
    let xxx = x_amz_content_sha256.clone();

    let http_method = "PUT";
    let url_encoded_file_name = urlencoding::encode(file_name.as_str());
    //let canonical_uri = format!("/{url_encoded_file_name}");
    let canonical_uri = format!("/{bucket}/{url_encoded_file_name}");
    let canonical_query_string = "";
    //let canonical_headers = format!("date:{ddate}\nhost:{bucket}.{host}:443\nx-amz-content-sha256:{xxx}\nx-amz-date:{x_amz_date}\n");
    let canonical_headers = format!(
        "date:{ddate}\nhost:{host}:443\nx-amz-content-sha256:{xxx}\nx-amz-date:{x_amz_date}\n"
    );
    let signed_headers = "date;host;x-amz-content-sha256;x-amz-date";
    let hashed_paylod = x_amz_content_sha256.clone();
    let canonical_request = format!(
        "{http_method}\n{canonical_uri}\n{canonical_query_string}\n{canonical_headers}\n{signed_headers}\n{hashed_paylod}"
    );

    tracing::debug!("\n\ncanonical_request:\n{}\n\n", canonical_request);

    let sha256_of_canonical_request =
        sha256hash_hex_encoded(canonical_request.as_bytes())
            .await
            .unwrap();
    tracing::debug!(
        "sha256_of_canonical_request: {}",
        sha256_of_canonical_request,
    );

    let date_key = hmac_sha256(
        format!("AWS4{secret_key}").into_bytes(),
        yyyymmdd.clone().into_bytes(),
    )
    .await
    .unwrap();
    let date_region_key = hmac_sha256(date_key, region.clone().into_bytes())
        .await
        .unwrap();
    let date_region_service_key =
        hmac_sha256(date_region_key, service.clone().into_bytes())
            .await
            .unwrap();
    let signing_key = hmac_sha256(
        date_region_service_key,
        "aws4_request".to_string().into_bytes(),
    )
    .await
    .unwrap();

    let string_to_sign = format!(
        "AWS4-HMAC-SHA256\n{x_amz_date}\n{yyyymmdd}/{region}/{service}/aws4_request\n{sha256_of_canonical_request}"
    );

    let signature =
        hmac_sha256(signing_key, string_to_sign.clone().into_bytes())
            .await
            .unwrap();

    let signature_string = hex_encoded(signature).await.unwrap();

    tracing::debug!("string_to_sign: {string_to_sign}");
    tracing::debug!("signature: {}", signature_string);
    let authorization = format!("AWS4-HMAC-SHA256 Credential={access_key}/{yyyymmdd}/{region}/s3/aws4_request,SignedHeaders=date;host;x-amz-content-sha256;x-amz-date,Signature={signature_string}");
    tracing::debug!(authorization);

    let request = Request::builder()
        .method(Method::Put)
        .uri(endpoint)
        .header("Authorization", authorization)
        .header("Content-Length", content_length)
        .header("X-Amz-Content-Sha256", x_amz_content_sha256)
        .header("X-Amz-Date", x_amz_date)
        .header("Date", ddate)
        .body(file.to_vec())
        .build();

    let response: Response = spin_sdk::http::send(request).await?;
    let r = response.status();
    let r1 = response.body();
    let r2 = std::str::from_utf8(r1).unwrap();

    let headers = response.headers();
    crate::utils::see_headers(headers).await;

    tracing::debug!("{} {}", r, r2);

    Ok(public_url)
}

pub async fn get_x_amz_date() -> (String, String) {
    let current_time: DateTime<Utc> = Utc::now();
    (
        current_time.format("%Y%m%dT%H%M%SZ").to_string(),
        // Fri, 24 May 2013 00:00:00 GMT
        current_time.format("%a, %d %b %Y %T GMT").to_string(),
    )
}

// https://www.devglan.com/online-tools/hmac-sha256-online
// HMAC() returns a byte[] and not a hex string.
// HMAC(key, data) represents an HMAC-SHA256 function that returns output in binary format.
// The result of each hash function becomes input for the next one.
// https://stackoverflow.com/questions/67656612/how-to-compute-hmac-sha-256-for-aws-authentication
pub async fn hmac_sha256(key: Vec<u8>, text: Vec<u8>) -> Result<Vec<u8>> {
    type HmacSha256 = SimpleHmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(&key).expect("Error from hmac_sha256");
    mac.update(&text);
    let f = mac.finalize();
    let f2 = f.into_bytes().to_vec();
    Ok(f2)
}

pub async fn sha256hash_hex_encoded(data: &[u8]) -> Result<String> {
    let mut hasher = Sha256::new();
    //hasher.update(data);
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

pub async fn hex_encoded(a: Vec<u8>) -> Result<String> {
    Ok(hex::encode(a))
}

pub enum S3Provider {
    Oci,
    Aws,
}

impl S3Provider {
    fn from_str(s: &str) -> Result<Option<S3Provider>> {
        match s.to_lowercase().as_str() {
            "aws" => Ok(Some(S3Provider::Aws)),
            "oci" => Ok(Some(S3Provider::Oci)),
            _ => Ok(None),
        }
    }
}

async fn s3_info(
) -> Result<(String, String, String, String, String, String, String)> {
    let s3_provider = variables::get("s3_provider").unwrap().to_string();
    let s = S3Provider::from_str(&s3_provider).unwrap().unwrap();

    // TODO: Change s3_provider to Enum.
    match s {
        S3Provider::Aws => {
            let bucket = variables::get("s3_bucket").unwrap();
            let host = format!(
                "{}.{}",
                bucket,
                variables::get("s3_endpoint").unwrap()
            );
            let endpoint = format!("{}.{}", bucket, host); // AWS is virtual domain style s3 endpoint.
            let region = variables::get("s3_region").unwrap();
            let access_key = variables::get("s3_access_key").unwrap();
            let secret_key = variables::get("s3_secret_key").unwrap();
            let public_url_template = "".to_string();
            return Ok((
                host,
                bucket,
                endpoint,
                region,
                access_key,
                secret_key,
                public_url_template,
            ));
        }
        S3Provider::Oci => {
            let host = variables::get("s3_endpoint").unwrap();
            let bucket = variables::get("s3_bucket").unwrap();
            let endpoint = format!("{}/{}", host, bucket);
            let region = variables::get("s3_region").unwrap();
            let access_key = variables::get("s3_access_key").unwrap();
            let secret_key = variables::get("s3_secret_key").unwrap();

            let bucket_namespace = endpoint.split(".").collect::<Vec<&str>>();
            let public_url_template = format!(
                "https://{}.objectstorage.{}.oci.customer-oci.com/n/{}/b/{}/o/",
                bucket_namespace[0], region, bucket_namespace[0], bucket
            );

            return Ok((
                host,
                bucket,
                endpoint,
                region,
                access_key,
                secret_key,
                public_url_template,
            ));
        }
    }
}
