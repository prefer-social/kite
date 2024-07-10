//! Engine for Prefer.social/kite, Mastodon implementation(somewhat minimum) for Spin framework  

#![warn(missing_docs)]

pub mod activitypub;

//pub mod auth;

//pub mod db;
//pub mod keys;

pub mod cache;

// pub mod follow_request;
pub mod http_response;

pub mod mastodon;

//pub mod postbox;
//pub mod send;

//pub mod storage;
// Database(sqlite) tables
pub mod table;
/// My utils
pub mod utils;

pub mod webfinger;
