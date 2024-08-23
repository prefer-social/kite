//! Represents a short written work typically less than a single paragraph in length.
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note>

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

use crate::activitystream::activity::Activity;
use crate::activitystream::collection::Collection;
use crate::activitystream::object::ObjectType;
use crate::mastodon::status::Status as MStatus;

/*

{
  "@context": "https://www.w3.org/ns/activitystreams",
  "type": "Note",
  "name": "A Word of Warning",
  "content": "Looks like it is going to rain today. Bring an umbrella!"
}

"object":{
    "id":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598",
    "type":"Note",
    "summary":null,
    "inReplyTo":null,
    "published":"2024-07-31T13:23:08Z",
    "url":"https://mstd.seungjin.net/@wsj/112881315922274598",
    "attributedTo":"https://mstd.seungjin.net/users/wsj",
    "to":["https://www.w3.org/ns/activitystreams#Public"],
    "cc":["https://mstd.seungjin.net/users/wsj/followers"],
    "sensitive":false,
    "atomUri":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598",
    "inReplyToAtomUri":null,
    "conversation": "tag:mstd.seungjin.net,2024-07-31:objectId=2214270:objectType=Conversation",
    "content":"FooFoo",
    "contentMap":{"en":"FooFoo"},
    "attachment":[],
    "tag":[],
    "replies":{
      "id":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/replies",
      "type":"Collection",
      "first":{
        "type":"CollectionPage",
        "next":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/replies?only_other_accounts=true\u0026page=true",
        "partOf":"https://mstd.seungjin.net/users/wsj/statuses/112881315922274598/replies",
        "items":[]
      }
    }


  Object {
    "id": String("https://mas.to/users/seungjin/statuses/112884530431559850"),
    "type": String("Note"),
    "summary": Null,
    "inReplyTo": Null,
    "published": String("2024-08-01T03:00:38Z"),
    "url": String("https://mas.to/@seungjin/112884530431559850"),
    "attributedTo": String("https://mas.to/users/seungjin"),
    "to": Array [String("https://www.w3.org/ns/activitystreams#Public")],
    "cc": Array [String("https://mas.to/users/seungjin/followers")],
    "sensitive": Bool(false),
    "atomUri": String("https://mas.to/users/seungjin/statuses/112884530431559850"),
    "inReplyToAtomUri": Null,
    "conversation": String("tag:mas.to,2024-08-01:objectId=369230771:objectType=Conversation"),
    "content": String("<p>999</p>"),
    "contentMap": Object {"en": String("<p>999</p>")},
    "attachment": Array [],
    "tag": Array [],
    "replies": Object {
      "id": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies"),
      "type": String("Collection"),
      "first": Object {
        "type": String("CollectionPage"),
        "next": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies?only_other_accounts=true&page=true"),
        "partOf": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies"),
        "items": Array []
      }
    }
  }

*/

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    #[serde(rename = "type")]
    pub note_type: ObjectType,
    pub summary: Option<String>,
    pub in_reply_to: Option<String>,
    pub published: Option<String>,
    pub url: Option<String>,
    pub attributed_to: Option<String>,
    pub to: Option<Vec<String>>,
    pub bto: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub sensitivity: Option<bool>,
    pub atom_url: Option<String>,
    pub in_reply_to_atom_uri: Option<String>,
    pub conversation: Option<String>,
    pub content: Option<String>,
    pub content_map: Option<HashMap<String, String>>, // Object {"en": String("<p>999</p>")},
    pub attachment: Option<Vec<String>>,              // Array [],
    pub tag: Option<Vec<String>>,                     // Array [],
    // Todo: Value for now.
    pub replies: Option<Value>,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = serde_json::to_string(self).unwrap();
        write!(f, "{}", a)
    }
}

//impl Execute for Note {
//    async fn execute(&self, _s: Value) -> Result<()> {
//        MStatus::new(self.to_owned()).await
//    }
//}
