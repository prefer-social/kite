//! Represents a short written work typically less than a single paragraph in length.
//!
//! <https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note>

use crate::activitystream::collection::Collection;
use crate::activitystream::object::ObjectType;

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


*/

pub struct Note {
    id: String,
    note_type: ObjectType,
    name: Option<String>,
    attachment: Option<String>,
    attributedTo: Option<String>,
    audience: Option<String>,
    content: Option<String>,
    context: Option<String>,
    endTime: Option<String>,
    generator: Option<String>,
    icon: Option<String>,
    image: Option<String>,
    inReplyTo: Option<String>,
    location: Option<String>,
    preview: Option<String>,
    published: Option<String>,
    replies: Collection,
    startTime: Option<String>,
    summary: Option<String>,
    tag: Option<String>,
    updated: Option<String>,
    url: Option<String>,
    to: Option<String>,
    bto: Option<String>,
    cc: Option<String>,
    bcc: Option<String>,
    mediaType: Option<String>,
    duration: Option<String>,
}
