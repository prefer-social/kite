use crate::mastodon::account::uri::Uri as AccountUri;
use crate::table::follow::Follow as TFollow;
use anyhow::Result;

pub struct Follow;

impl Follow {
    pub async fn follower_count(account_uri: AccountUri) -> Result<u64> {
        let account_uid = account_uri.account_uid().await.unwrap().to_string();
        TFollow::follower_count(account_uid).await
    }

    pub async fn following_count(account_uri: AccountUri) -> Result<u64> {
        let a = account_uri.account_uid().await.unwrap().to_string();
        TFollow::following_count(a).await
    }
}
