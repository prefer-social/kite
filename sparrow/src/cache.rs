//! Cache library  
//!
//! Cache library based on <https://docs.rs/spin-sdk/latest/spin_sdk/key_value/index.html>  
//! Backend: sqlite in memory  
//! References:
//! * <https://developer.fermyon.com/spin/v2/key-value-store-tutorial>  
//! * <https://developer.fermyon.com/spin/v2/dynamic-configuration#key-value-store-runtime-configuration>  

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use spin_sdk::key_value::Store;

/// Set key and value into Cache
pub async fn set(key: &str, val: &[u8]) -> Result<()> {
    let store = Store::open("mem").unwrap();
    store.set(key, val)?;
    Ok(())
}

/// Get cached value with key
pub async fn get(key: &str) -> Result<Option<Vec<u8>>> {
    let store = Store::open("mem").unwrap();
    Ok(store.get(key)?)
}

/// Delete cached key/value
pub async fn delete(key: &str) -> Result<()> {
    let store = Store::open("mem").unwrap();
    let exp_key = format!("_exp_{}", key);
    store.delete(key)?;
    if store.exists(exp_key.as_str())? {
        store.delete(exp_key.as_str())?;
    }
    Ok(())
}

/// Set key and value, value is json/serdo_json::value
pub async fn set_json(key: &str, val: &Value) -> Result<()> {
    let store = Store::open("mem").unwrap();
    store.set_json(key, val)?;
    Ok(())
}

/// Get json(serdo_json::Value) with key
pub async fn get_json(key: &str) -> Result<Option<Value>> {
    let store = Store::open("mem").unwrap();
    Ok(store.get_json(key).unwrap())
}

/// Set key/val fair's expiry time
pub async fn set_expiry(key: &str, exp: DateTime<Utc>) -> Result<()> {
    let store = Store::open("mem").unwrap();
    let val = store.get(key)?;
    if val.is_some() {
        let exp_key = format!("_exp_{}", key.to_string());
        store.set(exp_key.as_str(), &exp.timestamp().to_le_bytes())?;
    }
    Ok(())
}

/// flush key-val's lifetime
pub async fn flush() -> Result<()> {
    let store = Store::open("mem").unwrap();
    let keys = store.get_keys().unwrap();
    let exp_keys = keys
        .into_iter()
        .filter(|w| w.to_string().starts_with("_exp_"));

    for exp_key in exp_keys {
        match get(exp_key.as_str()).await? {
            Some(k) => {
                let a = String::from_utf8(k.clone()).unwrap();
                let b = a.as_str();
                let c = str::parse::<i64>(b).unwrap();

                let now = chrono::offset::Utc::now().timestamp();

                if c < now {
                    delete(String::from_utf8(k).unwrap().as_str()).await?;
                    delete(exp_key.as_str()).await?;
                }
            }
            None => {
                delete(exp_key.as_str()).await?;
            }
        }
    }

    Ok(())
}

/// Set key-val with exp time
pub async fn set_with_exp(
    key: &str,
    val: &[u8],
    exp: DateTime<Utc>,
) -> Result<()> {
    tracing::debug!(
        "<----- Cache set: {} : {} ---->",
        key,
        std::str::from_utf8(val).unwrap()
    );

    let store = Store::open("mem").unwrap();
    store.set(key, val)?;
    let exp_key = format!("_exp_{}", key);
    store.set(exp_key.as_str(), &exp.timestamp().to_le_bytes())?;
    Ok(())
}
