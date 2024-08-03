/*
Object {
      "id": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies"),
      "type": String("Collection"),
      "first": Object {
        "type": String("CollectionPage"),
        "next": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies?only_other_accounts=true&page=true"),
        "partOf": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies"),
        "items": Array []
      } */

pub struct Replies {
      id: String, 
      type: Collection,
      "first": Object {
        "type": String("CollectionPage"),
        "next": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies?only_other_accounts=true&page=true"),
        "partOf": String("https://mas.to/users/seungjin/statuses/112884530431559850/replies"),
        "items": Array []
      }

}
