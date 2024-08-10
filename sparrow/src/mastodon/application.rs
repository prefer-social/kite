//! Represents an application that interfaces with the REST API to access accounts or post statuses.    
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Application/>
//! Mastodon doc: <https://docs.joinmastodon.org/methods/apps/#create>

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

/// Represents an application that interfaces with the REST API to access accounts or post statuses.  
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Application/>  
/// Mastodon doc: <https://docs.joinmastodon.org/methods/apps/#create>  
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Application {
    /// UID<uuid v7> of your application.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String,
    /// The name of your application.
    pub name: String,
    /// The website associated with your application.
    /// Nullable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// Client ID key, to be used for obtaining OAuth tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Client secret key, to be used for obtaining OAuth tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// redirect_uri passed from request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// DEPRECATED
    /// Used for Push Streaming API. Returned with POST /api/v1/apps. Equivalent to WebPushSubscription#server_key and Instance#vapid_public_key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vapid_key: Option<String>,
    /// UID(user table's id) of owner of this application.
    #[serde(skip_serializing)]
    pub owner_id: Option<String>,
}

impl From<crate::table::oauth_application::OauthApplication> for Application {
    fn from(oa: crate::table::oauth_application::OauthApplication) -> Self {
        Application {
            uid: oa.uid.clone(),
            name: oa.name.clone(),
            website: Some(oa.website.clone()),
            redirect_uri: Some(oa.redirect_uri.clone()),
            client_id: None,
            client_secret: Some(oa.secret.clone()),
            vapid_key: None,
            owner_id: oa.owner_id.clone(),
        }
    }
}

impl Into<String> for Application {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Into<Value> for Application {
    fn into(self) -> Value {
        serde_json::to_value(&self).unwrap()
    }
}

impl Application {
    pub async fn add(app: String, user_id: Option<String>) -> Result<()> {
        let application: Application =
            serde_json::from_str(app.as_str()).unwrap();
        crate::table::oauth_application::OauthApplication::add(
            application,
            user_id,
        )
        .await?;
        tracing::debug!("App created");
        Ok(())
    }

    pub async fn all() -> Result<Vec<Application>> {
        let oauth_applications =
            crate::table::oauth_application::OauthApplication::all().await?;

        let mut r: Vec<Application> = Vec::new();

        for oap in oauth_applications.into_iter() {
            r.push(Application {
                uid: oap.uid.clone(),
                name: oap.name.clone(),
                website: Some(oap.website.clone()),
                redirect_uri: Some(oap.redirect_uri.clone()),
                client_id: None,
                client_secret: Some(oap.secret.clone()),
                vapid_key: None,
                owner_id: oap.owner_id.clone(),
            })
        }
        Ok(r)
    }

    pub async fn get_by_app_id(app_id: String) -> Result<Application> {
        let oa =
            crate::table::oauth_application::OauthApplication::get_by_app_id(
                app_id,
            )
            .await?;

        let a = Application {
            uid: oa.uid.clone(),
            name: oa.name.clone(),
            website: Some(oa.website.clone()),
            redirect_uri: Some(oa.redirect_uri.clone()),
            client_id: None,
            client_secret: Some(oa.secret.clone()),
            vapid_key: None,
            owner_id: oa.owner_id.clone(),
        };

        Ok(a)
    }

    pub async fn cancel_reserve(_uid: String) -> Result<()> {
        Ok(())
    }
}
