// https://docs.joinmastodon.org/methods/apps/#create
// https://docs.joinmastodon.org/entities/Application/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Application {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vapid_key: Option<String>,
    #[serde(skip_serializing)]
    pub owner_id: Option<String>,
}

impl From<crate::table::oauth_application::OauthApplication> for Application {
    fn from(oa: crate::table::oauth_application::OauthApplication) -> Self {
        Application {
            uid: Some(oa.uid.clone()),
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
                uid: Some(oap.uid.clone()),
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
            uid: Some(oa.uid.clone()),
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

    pub async fn cancel_reserve(uid: String) -> Result<()> {
        Ok(())
    }
}
