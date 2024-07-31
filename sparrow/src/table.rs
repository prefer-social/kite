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
