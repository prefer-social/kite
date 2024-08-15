//! Memory Storage
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
pub fn set(s: &Store, key: &str, val: &[u8]) -> Result<()> {
    s.set(key, val)?;
    Ok(())
}

/// Get cached value with key
pub fn get(s: &Store, key: &str) -> Result<Option<Vec<u8>>> {
    Ok(s.get(key)?)
}

/// Delete cached key/value
pub fn delete(s: &Store, key: &str) -> Result<()> {
    let exp_key = format!("_exp_{}", key);
    s.delete(key)?;
    if s.exists(exp_key.as_str())? {
        s.delete(exp_key.as_str())?;
    }
    Ok(())
}

/// Set key and value, value is json/serdo_json::value
pub fn set_val(s: &Store, key: &str, val: &Value) -> Result<()> {
    s.set_json(key, val)?;
    Ok(())
}

/// Get json(serdo_json::Value) with key
pub async fn get_val(s: &Store, key: &str) -> Result<Option<Value>> {
    Ok(s.get_json(key).unwrap())
}

/// Set key/val fair's expiry time
pub async fn set_expiry(
    s: &Store,
    key: &str,
    exp: DateTime<Utc>,
) -> Result<()> {
    let val = s.get(key)?;
    if val.is_some() {
        let exp_key = format!("_exp_{}", key.to_string());
        s.set(exp_key.as_str(), &exp.timestamp().to_le_bytes())?;
    }
    Ok(())
}

/// flush key-val's lifetime
pub fn flush() -> Result<()> {
    let store = Store::open("mem").unwrap();
    let keys = store.get_keys().unwrap();
    let exp_keys = keys
        .into_iter()
        .filter(|w| w.to_string().starts_with("_exp_"));

    for exp_key in exp_keys {
        match get(&store, exp_key.as_str())? {
            Some(k) => {
                let a = String::from_utf8(k.clone()).unwrap();
                let b = a.as_str();
                let c = str::parse::<i64>(b).unwrap();

                let now = chrono::offset::Utc::now().timestamp();

                if c < now {
                    delete(&store, String::from_utf8(k).unwrap().as_str())?;
                    delete(&store, exp_key.as_str())?;
                }
            }
            None => {
                delete(&store, exp_key.as_str())?;
            }
        }
    }

    Ok(())
}

/// Set key-val with exp time
pub fn set_with_exp(
    s: &Store,
    key: &str,
    val: &[u8],
    exp: DateTime<Utc>,
) -> Result<()> {
    tracing::debug!(
        "<----- Cache set: {} : {} ---->",
        key,
        std::str::from_utf8(val).unwrap()
    );
    s.set(key, val)?;
    let exp_key = format!("_exp_{}", key);
    s.set(exp_key.as_str(), &exp.timestamp().to_le_bytes())?;
    Ok(())
}

// Retrive Value from KeyVal storage
// pub fn get_val(s: &Store, key: &str) -> Result<Option<Value>> {
//     match s.get(key)? {
//         None => Ok(None),
//         Some(v) => Ok(Some(serde_json::from_slice(&v)?)),
//     }
// }

// Store Json Value type into KeyVal storage
// pub fn set_val(s: &Store, key: &str, value: &Value) -> Result<()> {
//     let json_bytes: Vec<u8> = serde_json::to_vec(&value)?;
//     Ok(s.set(key, &json_bytes)?)
// }
