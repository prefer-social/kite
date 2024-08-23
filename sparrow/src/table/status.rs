//! Mastodon status
//!
//! <https://docs.joinmastodon.org/entities/Status/>

use anyhow::Result;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::sqlite::{Connection, Value};
use spin_sqlx::sqlite::Connection as dbcon;
use std::time::{SystemTime, UNIX_EPOCH};
use struct_iterable::Iterable;
use uuid::Uuid;

use crate::activitystream::object::note::Note as NoteObject;
use crate::mastodon::status::Status as MStatus;
use crate::table::account::Account as TAccount;
use crate::table::FieldType;
use crate::table::New;

/// Represents a status posted by an account.  
///
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Status/>
#[derive(
    Clone,
    Debug,
    Deserialize,
    Serialize,
    PartialEq,
    Default,
    sqlx::FromRow,
    Iterable,
)]
pub struct Status {
    pub rowid: Option<String>,
    pub uid: String,
    pub uri: Option<String>,
    pub text: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub in_reply_to_id: Option<String>,
    pub reblog_of_id: Option<String>,
    pub url: Option<String>,
    pub sensitive: i64,
    pub visibility: i64,
    pub spoiler_text: String,
    pub reply: bool,
    pub language: Option<String>,
    pub conversation_id: Option<String>,
    pub local: Option<bool>,
    /// table account's uid
    pub account_id: String,
    pub application_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub poll_id: Option<String>,
    pub deleted_at: Option<i64>,
    pub edited_at: Option<i64>,
    pub trendable: Option<bool>,
    pub ordered_media_attachment_ids: Option<String>,
}

impl Status {
    /// Count status.
    /// Todo: Visibility apply.  
    pub async fn count(taccount: TAccount) -> Result<u32> {
        let sqlx_conn = dbcon::open_default()?;
        let (cnt,): (i64,) = sqlx::query_as(
            "SELECT count(rowid) AS CNT FROM status WHERE account_id = ?",
        )
        .bind(taccount.uid)
        .fetch_one(&sqlx_conn)
        .await?;
        Ok(cnt as u32)
    }
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Status>>;
}

impl TryFrom<MStatus> for Status {
    type Error = anyhow::Error;

    /// (Person)Actor to Account
    fn try_from(mstatus: MStatus) -> Result<Self, Self::Error> {
        let current_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let status = Status {
            rowid: None,
            uid: Uuid::now_v7().to_string(),
            uri: mstatus.uri,
            text: mstatus.text,
            created_at: mstatus.created_at.timestamp(),
            updated_at: current_epoch,
            in_reply_to_id: mstatus.in_reply_to_id,
            reblog_of_id: None,
            url: mstatus.url,
            sensitive: (|x: bool| -> i64 {
                match x {
                    false => 0,
                    true => 1,
                }
            })(mstatus.sensitive),
            visibility: (|x: &str| -> i64 {
                match x {
                    "public" => 0,
                    "unlisted" => 1,
                    "private" => 2,
                    "direct" => 3,
                    _ => 0,
                }
            })(mstatus.visibility.as_str()),
            spoiler_text: mstatus.spoiler_text,
            ..Default::default()
        };
        Ok(status)
    }
}

#[async_trait]
impl Get<(String, String)> for Status {
    async fn get((key, val): (String, String)) -> Result<Vec<Status>> {
        let query_template =
            format!("SELECT rowid, * FROM status WHERE {} = ?", key);
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}

impl New for Status {}

// #[async_trait]
// impl New<Self> for Status {
//     async fn new(status: Status) -> Result<()> {
//         let table_name = std::any::type_name::<Option<Status>>();
//         let mut query_template = format!("INSERT INTO {} (", table_name);
//         let mut query_params: Vec<Value> = Vec::new();

//         for (field_name, field_value) in status.iter() {
//             query_template.push_str(&field_name);
//             query_template.push_str(", ");

//             match super::check_type(field_value) {
//                 FieldType::String => {
//                     query_params.push(Value::Text(
//                         field_value
//                             .downcast_ref::<String>()
//                             .unwrap()
//                             .to_owned(),
//                     ));
//                 }
//                 FieldType::OptionString => {
//                     let a = &field_value.downcast_ref::<Option<String>>();
//                     if a.unwrap().is_none() {
//                         query_params.push(Value::Null);
//                     };
//                     query_params
//                         .push(Value::Text(a.unwrap().to_owned().unwrap()));
//                 }
//                 FieldType::I64 => query_params.push(Value::Integer(
//                     field_value.downcast_ref::<i64>().unwrap().to_owned(),
//                 )),
//                 FieldType::OptionI64 => {
//                     let a = &field_value.downcast_ref::<Option<i64>>();
//                     if a.unwrap().is_none() {
//                         query_params.push(Value::Null);
//                     };
//                     query_params
//                         .push(Value::Integer(a.unwrap().to_owned().unwrap()));
//                 }
//                 FieldType::F64 => query_params.push(Value::Real(
//                     field_value.downcast_ref::<f64>().unwrap().to_owned(),
//                 )),
//                 FieldType::OptionF64 => {
//                     let a = &field_value.downcast_ref::<Option<f64>>();
//                     if a.unwrap().is_none() {
//                         query_params.push(Value::Null);
//                     };
//                     query_params.push(Value::Real(
//                         a.unwrap().to_owned().unwrap().to_owned(),
//                     ));
//                 }
//                 _ => query_params.push(Value::Null),
//             };
//         }
//         query_template.push_str(") VALUES (");

//         tracing::debug!("{}", query_template);
//         tracing::debug!("{:?}", query_params);

//         // let connection = Connection::open_default()?;
//         // connection.execute(&query_template.as_str(), execute_params.as_slice())?;

//         Ok(())
//     }
// }

// #[async_trait]
// impl New<NoteObject> for Status {
//     async fn new(note: NoteObject) -> Result<()> {
//         let query_template = format!(
//             "INSERT INTO status(
//                 uid,
//                 uri,
//                 text,
//                 created_at,
//                 updated_at,
//                 in_reply_to_id,
//                 reblog_of_id,
//                 url,
//                 sensitive,
//                 visibility,
//                 spoiler_text,
//                 reply,
//                 language,
//                 conversation_id,
//                 local,
//                 account_id,
//                 application_id,
//                 in_reply_to_account_id,
//                 poll_id,
//                 deleted_at,
//                 edited_at,
//                 trendable,
//                 ordered_media_attachment_ids,
//             ) VALUES ()"
//         );
//         let sqlx_conn = dbcon::open_default()?;
//         sqlx::query(query_template.as_str())
//             //.bind(val)
//             .execute(&sqlx_conn)
//             .await?;
//         Ok(())
//     }
// }

// #[async_trait]
// impl New<MStatus> for Status {
//     async fn new(status: MStatus) -> Result<()> {
//         Ok(())
//     }
// }
