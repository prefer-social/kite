// https://docs.joinmastodon.org/methods/apps/#create
// https://docs.joinmastodon.org/entities/Application/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::key_value::Store;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: Option<String>,
    pub name: String,
    pub website: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub vapid_key: Option<String>,
}

pub async fn new(app: Application) -> Result<Application> {
    // Store the client_id and client_secret in your cache, as these will be used to obtain OAuth tokens.
    let client_id = app.client_id.clone().unwrap();
    let client_secret = app.client_secret.clone().unwrap();

    let store = Store::open("mem").unwrap();
    store.set(client_id.as_str(), client_secret.as_bytes())?;

    let r =
        crate::table::oauth_apllication::OauthApplication::create(app)
            .await?;

    let ret = Application {
        id: Some(r.uid),
        name: r.name,
        website: Some(r.website),
        redirect_uri: Some("urn:ietf:wg:oauth:2.0:oob".to_string()),
        client_id: Some(client_id),
        client_secret: Some(client_secret),
        vapid_key: None,
    };

    Ok(ret)
}
