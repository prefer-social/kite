use serde::{Deserialize, Serialize};

use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::uid::Uid as AccountUid;
use crate::mastodon::account::uri::Uri as AccountUri;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::table::follow::Follow as TFollow;
use anyhow::Result;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Follow {
    pub rowid: Option<i64>,
    pub uid: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub account_uid: Option<String>,
    pub target_account_uid: Option<i64>,
    pub show_rebloges: Option<bool>,
    pub uri: Option<String>,
    pub notify: Option<bool>,
    pub languages: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum FollowRelation {
    #[default]
    None,
    OnlyA,
    OnlyB,
    Both,
}

impl Follow {
    pub async fn new(
        uri: String,
        sub: AccountUid,
        obj: AccountUid,
    ) -> Result<()> {
        TFollow::new(uri.clone(), sub.clone(), obj.clone()).await
        //Self::get(uri, sub, obj).await
    }

    // pub async fn get(
    //     uri: String,
    //     sub: AccountUid,
    //     obj: AccountUid,
    // ) -> Result<Self> {
    // }

    pub async fn follower_count(account_uri: AccountUri) -> Result<u64> {
        let account_uid = account_uri.account_uid().await.unwrap().to_string();
        TFollow::follower_count(account_uid).await
    }

    pub async fn following_count(account_uri: AccountUri) -> Result<u64> {
        let a = account_uri.account_uid().await.unwrap().to_string();
        TFollow::following_count(a).await
    }

    //pub async fn relations(a: MAccount, b: MAccount) -> Result<Vec<Follow>> {}

    pub async fn relationship(a: MAccount, b: MAccount) -> FollowRelation {
        let c = a.uid.to_string();
        let d = b.uid.to_string();

        tracing::debug!("{c} - {d}");

        let follow_relationship =
            TFollow::relation(a.uid.to_string(), b.uid.to_string())
                .await
                .unwrap();

        let following: bool;
        let followed_by: bool;
        return match follow_relationship {
            0 => FollowRelation::None,
            1 => FollowRelation::OnlyA,
            2 => FollowRelation::OnlyB,
            3 => FollowRelation::Both,
            _ => FollowRelation::Both,
        };
    }
}
