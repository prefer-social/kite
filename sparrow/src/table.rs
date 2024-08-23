//! Database Tables  
//!
//! Each database(sqlite) table has one corresponding struct.
//!
//! Rules:  
//! Allowed crates. If you need to use other than follow crates, do it in Mastodon section.  
//! use anyhow::Result;  
//! use async_trait::async_trait;  
//! use serde::{Deserialize, Serialize};  
//! use serde_json;  
//! use serde_json::Value;  
//! use std::time::{SystemTime, UNIX_EPOCH};  
//! use url::Url;  
//!

pub mod account;
pub mod activity_log;
pub mod actor_json;
pub mod conversation;
pub mod conversation_mute;
pub mod follow;
pub mod mute;
pub mod oauth_access_grant;
pub mod oauth_access_token;
pub mod oauth_application;
pub mod setting;
pub mod status;
pub mod user;
pub mod user_role;

use anyhow::{Error, Result};
use async_trait::async_trait;
use spin_sdk::sqlite::{Connection as SpinDbCon, Value};
use std::any::Any;
use struct_iterable::Iterable;

#[derive(Debug)]
pub enum FieldType {
    String,
    OptionString,
    I64,
    OptionI64,
    F64,
    OptionF64,
    Bool,
    OptionBool,
    NotDefined,
}

fn check_type(v: &dyn Any) -> FieldType {
    if v.is::<String>() {
        return FieldType::String;
    }
    if v.is::<Option<String>>() {
        return FieldType::OptionString;
    }
    if v.is::<i64>() {
        return FieldType::I64;
    }
    if v.is::<Option<i64>>() {
        return FieldType::OptionI64;
    }
    if v.is::<f64>() {
        return FieldType::F64;
    }
    if v.is::<Option<f64>>() {
        return FieldType::OptionF64;
    }
    if v.is::<bool>() {
        return FieldType::Bool;
    }
    if v.is::<Option<bool>>() {
        return FieldType::OptionBool;
    }
    FieldType::NotDefined
}

#[async_trait]
pub trait New {
    async fn new(&self) -> Result<()>
    where
        Self: Iterable,
    {
        let table_name =
            std::any::type_name::<Self>().split("::").last().unwrap();
        let mut query_template = format!("INSERT INTO {} (", table_name);
        let mut query_arg_template = "(".to_string();
        let mut query_params: Vec<Value> = Vec::new();

        for (field_name, field_value) in self.iter() {
            if vec!["rowid"].contains(&field_name) {
                continue;
            }

            query_template.push_str(&field_name);
            query_template.push_str(", ");
            query_arg_template.push_str("?, ");

            match check_type(field_value) {
                FieldType::String => {
                    let a = field_value
                        .downcast_ref::<String>()
                        .unwrap()
                        .to_owned();
                    let b = Value::Text(a);
                    query_params.push(b);
                }
                FieldType::OptionString => {
                    let a = field_value
                        .downcast_ref::<Option<String>>()
                        .unwrap()
                        .to_owned();
                    if a.is_none() {
                        query_params.push(Value::Null);
                    } else {
                        query_params.push(Value::Text(a.unwrap()));
                    }
                }
                FieldType::I64 => query_params.push(Value::Integer(
                    field_value.downcast_ref::<i64>().unwrap().to_owned(),
                )),
                FieldType::OptionI64 => {
                    let a = field_value
                        .downcast_ref::<Option<i64>>()
                        .unwrap()
                        .to_owned();
                    if a.is_none() {
                        query_params.push(Value::Null);
                    } else {
                        query_params.push(Value::Integer(a.unwrap()));
                    }
                }
                FieldType::F64 => query_params.push(Value::Real(
                    field_value.downcast_ref::<f64>().unwrap().to_owned(),
                )),
                FieldType::OptionF64 => {
                    let a = field_value
                        .downcast_ref::<Option<f64>>()
                        .unwrap()
                        .to_owned();
                    if a.is_none() {
                        query_params.push(Value::Null);
                    };
                    query_params.push(Value::Real(a.unwrap()));
                }
                FieldType::Bool => {
                    let a =
                        field_value.downcast_ref::<bool>().unwrap().to_owned();
                    let b = match a {
                        false => 0i64,
                        _ => 1i64,
                    };
                    query_params.push(Value::Integer(b))
                }
                FieldType::OptionBool => {
                    let a = field_value
                        .downcast_ref::<Option<bool>>()
                        .unwrap()
                        .to_owned();
                    if a.is_none() {
                        query_params.push(Value::Null);
                    } else {
                        let b = match a.unwrap() {
                            false => 0i64,
                            _ => 1i64,
                        };
                        query_params.push(Value::Integer(b));
                    }
                }
                _ => query_params.push(Value::Null),
            };
        }
        query_template.pop();
        query_template.pop();
        query_template.push_str(")");
        query_arg_template.pop();
        query_arg_template.pop();
        query_arg_template.push_str(")");

        let qt = format!("{} VALUES {}", query_template, query_arg_template);

        let connection = SpinDbCon::open_default()?;
        match connection.execute(&qt.as_str(), query_params.as_slice()) {
            Ok(_) => return Ok(()),
            Err(e) => {
                tracing::error!("{:?}", qt);
                tracing::error!("{:?}", query_params);
                return Err(Error::msg(e));
            }
        }
    }
}
