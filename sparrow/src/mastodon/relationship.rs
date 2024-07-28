//! Mastodon api's Relationship object
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Relationship/>

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::mastodon::account::uid::Uid as AccountUid;
use crate::mastodon::account::Account as MAccount;
use crate::mastodon::follow::Follow;
use crate::mastodon::follow::FollowRelation;

/// Represents the relationship between accounts, such as following / blocking / muting / etc.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct Relationship {
    /// The account ID. Uuid.v7.
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: AccountUid,
    /// Are you following this user?
    pub following: bool,
    /// Are you receiving this user’s boosts in your home timeline?
    pub showing_reblogs: bool,
    /// Have you enabled notifications for this user?
    pub notifying: bool,
    /// Which languages are you following from this user?
    pub languages: Vec<String>,
    /// Are you followed by this user?
    pub followed_by: bool,
    /// Are you blocking this user?
    pub blocking: bool,
    /// Is this user blocking you?
    pub blocked_by: bool,
    /// Are you muting this user?
    pub muting: bool,
    /// Are you muting notifications from this user?
    pub muting_notifications: bool,
    /// Do you have a pending follow request for this user?
    pub requested: bool,
    /// Has this user requested to follow you?
    pub requested_by: bool,
    /// Are you blocking this user’s domain?
    pub domain_blocking: bool,
    /// Are you featuring this user on your profile
    pub endorsed: bool,
    /// This user’s profile bio
    pub note: String,
}

impl Relationship {
    pub async fn new(a: &MAccount, b: &MAccount) -> Result<Self> {
        //let follow_relationship =
        //    TFollow::relation(a.uid.to_string(), b.uid.to_string()).await?;
        let follow_relationship =
            Follow::relationship(a.to_owned(), b.to_owned()).await;

        let following: bool;
        let followed_by: bool;
        match follow_relationship {
            FollowRelation::None => {
                following = false;
                followed_by = false;
            }
            FollowRelation::OnlyA => {
                following = true;
                followed_by = false;
            }
            FollowRelation::OnlyB => {
                following = false;
                followed_by = true;
            }
            FollowRelation::Both => {
                following = true;
                followed_by = true;
            }
        }

        // Todo:
        // `follow` table has information about Relationship.
        Ok(Relationship {
            uid: b.uid.to_owned(),
            following,
            showing_reblogs: true,
            notifying: true,
            languages: Vec::new(),
            followed_by,
            blocking: false,
            blocked_by: false,
            muting: false,
            muting_notifications: false,
            requested: false,
            requested_by: false,
            domain_blocking: false,
            endorsed: false,
            note: b.to_owned().note,
        })
    }
}
