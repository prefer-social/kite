//! ActivityPub Person Actor   
//!
//! Resource: <https://www.w3.org/TR/activitypub/#actor-objects>  

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::table::account::Account;
use crate::table::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonActor {
    #[serde(rename = "@context")]
    pub context: Value,
    pub id: String,
    #[serde(rename = "type")]
    pub actor_type: String,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    pub featured: String,
    pub featured_tags: String,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memorial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<String>,
    pub public_key: PublicKey,
    pub tag: Vec<Value>,
    pub attachment: Vec<Value>,
    pub endpoints: Endpoints,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    pub kind: String,
    pub media_type: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    pub shared_inbox: String,
}

impl PersonActor {
    pub async fn build(u: User, a: Account) -> Result<Self> {
        let pk = PublicKey {
            id: format!("{}#main-key", a.uri.clone().clone()),
            owner: a.clone().uri.clone(),
            public_key_pem: a.public_key,
        };

        let icon = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: a.avatar_remote_url.unwrap(),
        };

        let image = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: a.header_remote_url.unwrap(),
        };

        let username = a.username;

        let ct = vec![
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string(),
        ];
        let ct_val = serde_json::to_value(&ct).unwrap();

        let pa = PersonActor {
            context: ct_val,
            id: a.uri,
            actor_type: "Person".to_string(),
            following: a.following_url.unwrap().clone(),
            followers: a.followers_url.unwrap().clone(),
            inbox: a.inbox_url.unwrap().clone(),
            outbox: a.outbox_url.unwrap().clone(),
            featured: "".to_string(),      // Todo:
            featured_tags: "".to_string(), // Todo:
            preferred_username: username.clone(), //a.display_name.unwrap().clone(),
            name: username,
            summary: a.note,
            url: a.url.unwrap(),
            manually_approves_followers: false, // Todo:
            discoverable: a.discoverable.unwrap_or_default(),
            indexable: a.indexable.unwrap_or_default(),
            published: crate::utils::convert_epoch_to_iso_8601(
                u.created_at.unwrap(),
            )
            .await,
            memorial: Some(false),
            devices: None,
            public_key: pk,
            tag: vec![Value::Null],
            attachment: vec![Value::Null],
            icon: Some(icon),
            image: Some(image),
            ..Default::default()
        };
        Ok(pa)
    }

    // async fn get_public_key(u: User) -> PublicKey {
    //     let account_id = u.account_id;
    //     let account =
    //         crate::table::account::Account::get_with_userid(account_id)
    //             .await
    //             .unwrap()
    //             .unwrap();
    //     let my_account = account.get(0).unwrap();
    //     let federation_id = my_account.clone().federation_id().await.unwrap();
    //     let pubkey = my_account.clone().public_key;
    //     PublicKey {
    //         id: format!("{}#main-key", federation_id),
    //         owner: federation_id,
    //         public_key_pem: pubkey,
    //     }
    // }

    // pub async fn create(acct: Account) -> Result<PersonActor> {
    //     let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    //     let pub_date =
    //         acct.created_at.format_with_items(fmt.clone()).to_string();

    //     let federation_id = acct.federation_id().await.unwrap();

    //     let public_key_pem = acct.public_key;
    //     let pk = PublicKey {
    //         id: format!("{}#main-key", federation_id.clone()),
    //         owner: federation_id.clone(),
    //         public_key_pem: public_key_pem,
    //     };

    //     let avatar_remote_url = acct.avatar_remote_url.unwrap();
    //     let icon = Image {
    //         kind: "Image".to_string(),
    //         media_type: "image/jpeg".to_string(),
    //         url: avatar_remote_url,
    //     };

    //     let image = Image {
    //         kind: "Image".to_string(),
    //         media_type: "image/jpeg".to_string(),
    //         url: acct.header_remote_url.unwrap(),
    //     };

    //     let pa = PersonActor {
    //         context: vec![
    //             "https://www.w3.org/ns/activitystreams".to_string(),
    //             "https://w3id.org/security/v1".to_string(),
    //         ],
    //         id: federation_id,
    //         kind: "Person".to_string(),
    //         following: todo!(),
    //         followers: acct.followers_url,
    //         inbox: acct.inbox_url,
    //         outbox: acct.outbox_url,
    //         featured: "".to_string(),      // Todo:
    //         featured_tags: "".to_string(), // Todo:
    //         preferred_username: acct.username,
    //         name: acct.username,
    //         summary: acct.note,
    //         url: acct.url,
    //         manually_approves_followers: false, // Todo:
    //         discoverable: acct.discoverable.unwrap(),
    //         indexable: acct.indexable.unwrap(),
    //         published: pub_date,
    //         memorial: Some(false),
    //         devices: None,
    //         public_key: pk,
    //         tags: vec![Value::Null],
    //         attachment: vec![Value::Null],
    //         icon: icon,
    //         image: image,
    //     };

    //     Ok(pa)
    // }
}

impl TryFrom<serde_json::Value> for PersonActor {
    type Error = ();
    fn try_from(actor_value: serde_json::Value) -> Result<Self, Self::Error> {
        let actor =
            serde_json::from_value::<PersonActor>(actor_value).unwrap();

        tracing::debug!("--=-=-=-=-=-=-=-=-=-=");
        tracing::debug!("--=-=-=-=-=-=-=-=-=-=");
        tracing::debug!("--=-=-=-=-=-=-=-=-=-=");
        tracing::debug!("{:?}", actor);

        let b = PersonActor {
            ..Default::default()
        };

        Ok(b)
    }
}
