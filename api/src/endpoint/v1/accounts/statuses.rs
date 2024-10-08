// /api/v1/accounts/1/statuses?only_media=false&exclude_replies=true&exclude_reblogs=true
// https://docs.joinmastodon.org/methods/accounts/#statuses

use anyhow::Result;
use sparrow::http_response::HttpResponse;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::collections::HashMap;
use tracing::debug;
use url::Url;

pub async fn request(req: Request, params: Params) -> Result<Response> {
    match req.method() {
        Method::Get => get(req, params).await,
        _ => HttpResponse::not_found().await,
    }
}

/*
[
  {
    "id": "108880211901672326",
    "created_at": "2022-08-24T22:29:46.493Z",
    "in_reply_to_id": "108880209317577809",
    "in_reply_to_account_id": "103641",
    "sensitive": false,
    // ...
  },
  // ...
]

*/

// Returns: Array of Status (https://docs.joinmastodon.org/entities/Status/)
//
pub async fn get(req: Request, params: Params) -> Result<Response> {
    debug!("{params:?}");

    let foo = r#"[

    {
      "id": "103270115826048975",
      "created_at": "2019-12-08T03:48:33.901Z",
      "in_reply_to_id": null,
      "in_reply_to_account_id": null,
      "sensitive": false,
      "spoiler_text": "",
      "visibility": "public",
      "language": "en",
      "uri": "https://mastodon.social/users/Gargron/statuses/103270115826048975",
      "url": "https://mastodon.social/@Gargron/103270115826048975",
      "replies_count": 5,
      "reblogs_count": 6,
      "favourites_count": 11,
      "favourited": false,
      "reblogged": false,
      "muted": false,
      "bookmarked": false,
      "content": "<p>&quot;I lost my inheritance with one wrong digit on my sort code&quot;</p><p><a href=\"https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code\" rel=\"nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://www.</span><span class=\"ellipsis\">theguardian.com/money/2019/dec</span><span class=\"invisible\">/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code</span}</p>",
      "reblog": null,
      "application": {
        "name": "Web",
        "website": null
      },
      "account": {
        "id": "1",
        "username": "Gargron",
        "acct": "Gargron",
        "display_name": "Eugen",
        "locked": false,
        "bot": false,
        "discoverable": true,
        "group": false,
        "created_at": "2016-03-16T14:34:26.392Z",
        "note": "<p>Developer of Mastodon and administrator of mastodon.social. I post service announcements, development updates, and personal stuff.</p>",
        "url": "https://mastodon.social/@Gargron",
        "avatar": "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg",
        "avatar_static": "https://files.mastodon.social/accounts/avatars/000/000/001/original/d96d39a0abb45b92.jpg",
        "header": "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png",
        "header_static": "https://files.mastodon.social/accounts/headers/000/000/001/original/c91b871f294ea63e.png",
        "followers_count": 322930,
        "following_count": 459,
        "statuses_count": 61323,
        "last_status_at": "2019-12-10T08:14:44.811Z",
        "emojis": [],
        "fields": [
          {
            "name": "Patreon",
            "value": "<a href=\"https://www.patreon.com/mastodon\" rel=\"me nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://www.</span><span class=\"\">patreon.com/mastodon</span><span class=\"invisible\"></span}",
            "verified_at": null
          },
          {
            "name": "Homepage",
            "value": "<a href=\"https://zeonfederated.com\" rel=\"me nofollow noopener noreferrer\" target=\"_blank\"><span class=\"invisible\">https://</span><span class=\"\">zeonfederated.com</span><span class=\"invisible\"></span}",
            "verified_at": "2019-07-15T18:29:57.191+00:00"
          }
        ]
      },
      "media_attachments": [],
      "mentions": [],
      "tags": [],
      "emojis": [],
      "card": {
        "url": "https://www.theguardian.com/money/2019/dec/07/i-lost-my-193000-inheritance-with-one-wrong-digit-on-my-sort-code",
        "title": "‘I lost my £193,000 inheritance – with one wrong digit on my sort code’",
        "description": "When Peter Teich’s money went to another Barclays customer, the bank offered £25 as a token gesture",
        "type": "link",
        "author_name": "",
        "author_url": "",
        "provider_name": "",
        "provider_url": "",
        "html": "",
        "width": 0,
        "height": 0,
        "image": null,
        "embed_url": ""
      },
      "poll": null
    }
    
    ]"#;

    let json_val: serde_json::Value = serde_json::from_str(foo).unwrap();

    Ok(Response::builder()
        .status(200)
        .header("Context-Type", "application/activity+json")
        .body(json_val.to_string())
        .build())
}

pub async fn public_statuses() {}

pub async fn statuses() {}
