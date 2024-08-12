//! Utilies

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use passwords::PasswordGenerator;
use serde_json::Value;
use spin_sdk::http::{HeaderValue, Method, Request, Response};
use std::str;
use url::Url;

pub async fn create_token() -> String {
    let pg = PasswordGenerator {
        length: 64,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    pg.generate_one().unwrap()
}

/// RFC 1123: Sun, 21 Oct 2018 12:16:24 GMT
pub fn get_current_time_in_rfc_1123() -> String {
    use chrono::{DateTime, Utc};
    let current_time: DateTime<Utc> = Utc::now();
    current_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

/// Iso8601: 2024-02-27T06:17:54Z
/// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
pub fn get_current_time_in_iso_8601() -> String {
    use chrono::{DateTime, Utc};
    let current_time: DateTime<Utc> = Utc::now();
    current_time.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn get_current_epoch() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn convert_epoch_to_iso_8601(epoch: i64) -> String {
    let naive = DateTime::from_timestamp(epoch, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive.naive_utc(), Utc);
    datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

// debug tool
pub fn see_headers<'a>(headers: impl Iterator<Item = (&'a str, &'a HeaderValue)>) {
    for header in headers {
        tracing::debug!("{header:?}");
    }
}

pub async fn clean_last_slash_from_url(c: Url) -> String {
    let a = match c.path() {
        "/" => {
            format!("{:?}", &c.to_string()[..c.to_string().len() - 1])
        }
        _ => c.to_string(),
    };
    println!("{}", a);
    a
}

pub async fn random_string(length: u8) -> String {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    let mut rng = StdRng::from_entropy();
    let random_string: String = (0..length)
        .map(|_| match rng.gen_range(0..=2) {
            0 => char::from(rng.gen_range(b'0'..=b'9') as char),
            1 => char::from(rng.gen_range(b'A'..=b'Z') as char),
            _ => char::from(rng.gen_range(b'a'..=b'z') as char),
        })
        .collect();
    random_string
}
