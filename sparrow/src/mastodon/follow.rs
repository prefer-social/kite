use serde::{Deserialize, Serialize};
use sha2::digest::MacError;

use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::uid::Uid as AccountUid;
use crate::mastodon::account::uri::Uri as AccountUri;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::account::Get as _;
use crate::table::follow::Follow as TFollow;
use crate::table::follow::Get as _;
use anyhow::Result;

/// Follow
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Follow {
    pub rowid: Option<i64>,
    pub uid: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub account_uid: Option<String>,
    pub target_account_uid: Option<i64>,
    pub show_reblogs: Option<bool>,
    pub uri: Option<String>,
    pub notify: Option<bool>,
    pub languages: Option<String>,
}

/// Relation status between two accounts.  
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum FollowRelation {
    #[default]
    None,
    OnlyA,
    OnlyB,
    Both,
}

impl Follow {
    pub async fn new(uri: String, sub: AccountUid, obj: AccountUid) -> Result<()> {
        match Self::is_exist(sub.to_owned(), obj.to_owned()).await? {
            true => TFollow::update(uri, sub, obj).await,
            false => TFollow::new(uri, sub, obj).await,
        }
    }

    pub async fn follower_count(account_uri: AccountUri) -> Result<u64> {
        let account_uid = account_uri.account_uid().await.unwrap().to_string();
        TFollow::follower_count(account_uid).await
    }

    pub async fn following_count(account_uri: AccountUri) -> Result<u64> {
        let a = account_uri.account_uid().await.unwrap().to_string();
        TFollow::following_count(a).await
    }

    //pub async fn relations(a: MAccount, b: MAccount) -> Result<Vec<Follow>> {}

    /// shows FollowRelation between two accounts.  
    pub async fn relationship(a: MAccount, b: MAccount) -> FollowRelation {
        let c = a.uid.to_string();
        let d = b.uid.to_string();

        tracing::debug!("{c} - {d}");

        let follow_relationship = TFollow::relation(a.uid.to_string(), b.uid.to_string())
            .await
            .unwrap();

        return match follow_relationship {
            0 => FollowRelation::None,
            1 => FollowRelation::OnlyA,
            2 => FollowRelation::OnlyB,
            3 => FollowRelation::Both,
            _ => FollowRelation::Both,
        };
    }

    pub async fn undo(uri: String) -> Result<()> {
        TFollow::unfollow(uri).await
    }

    pub async fn is_exist(sub: AccountUid, obj: AccountUid) -> Result<bool> {
        let s = MAccount::get(sub).await?;
        let o = MAccount::get(obj).await?;

        let relation = TFollow::relation(s.uid.to_string(), o.uid.to_string()).await?;

        tracing::debug!("-----------------------------> {}", relation);
        Ok(relation == 1usize || relation == 3usize)
    }

    pub async fn get_follows(id: String) -> Result<Vec<MAccount>> {
        let mut maccounts: Vec<MAccount> = Vec::new();

        let followers = TFollow::get(("target_account_uid".to_string(), id)).await?;
        for f in followers.iter() {
            let account_id = f.to_owned().account_uid.unwrap();
            let account_uid = AccountUid(account_id);
            let account = MAccount::get(account_uid).await?;
            maccounts.push(account);
        }
        Ok(maccounts)
    }

    pub async fn get_following(id: String) -> Result<Vec<MAccount>> {
        let mut maccounts: Vec<MAccount> = Vec::new();

        let following = TFollow::get(("account_uid".to_string(), id)).await?;
        for f in following.iter() {
            let target_account_id = f.to_owned().target_account_uid.unwrap();
            let target_account_uid = AccountUid(target_account_id);
            let account = MAccount::get(target_account_uid).await?;
            maccounts.push(account);
        }
        Ok(maccounts)
    }
}
