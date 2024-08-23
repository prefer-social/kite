//! Represents a custom user role that grants permissions.  
//!
//! Mastodon doc: <https://docs.joinmastodon.org/entities/Role/>

use anyhow::Result;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::mastodon::user::User;
use crate::table::user_role::Get;

/// Represents a custom user role that grants permissions.  
///
/// Mastodon doc: <https://docs.joinmastodon.org/entities/Role/>
///
/// Permission flags <https://docs.joinmastodon.org/entities/Role/#permission-flags>
/// To determine the permissions available to a certain role, convert the permissions attribute to binary and compare from the least significant bit upwards. For convenience (and to prevent the terms from growing too long), permissions will be presented below using hexadecimal values.
///
/// 0x1     Administrator. Users with this permission bypass all permissions.
/// 0x2     Devops. Allows users to access Sidekiq and PgHero dashboards.
/// 0x4     View Audit Log. Allows users to see history of admin actions.
/// 0x8     View Dashboard. Allows users to access the dashboard and various metrics.
/// 0x10    Manage Reports. Allows users to review reports and perform moderation actions against them.
/// 0x20    Manage Federation. Allows users to block or allow federation with other domains, and control deliverability.
/// 0x40    Manage Settings. Allows users to change site settings.
/// 0x80    Manage Blocks. Allows users to block e-mail providers and IP addresses.
/// 0x100   Manage Taxonomies. Allows users to review trending content and update hashtag settings.
/// 0x200   Manage Appeals. Allows users to review appeals against moderation actions.
/// 0x400   Manage Users. Allows users to view other users’ details and perform moderation actions against them.
/// 0x800   Manage Invites. Allows users to browse and deactivate invite links.
/// 0x1000  Manage Rules. Allows users to change server rules.
/// 0x2000  Manage Announcements. Allows users to manage announcements on the server.
/// 0x4000  Manage Custom Emojis. Allows users to manage custom emojis on the server.
/// 0x8000  Manage Webhooks. Allows users to set up webhooks for administrative events.
/// 0x10000 Invite Users. Allows users to invite new people to the server.
/// 0x20000 Manage Roles. Allows users to manage and assign roles below theirs.
/// 0x40000 Manage User Access. Allows users to disable other users’ two-factor authentication, change their e-mail address, and reset their password.
/// 0x80000 Delete User Data. Allows users to delete other users’ data without delay.
#[derive(
    Default,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    Encode,
    Decode,
)]
pub struct UserRole {
    /// The ID(uuid v7) of the Role in the database.
    pub uid: String,
    /// The name of the role.
    pub name: String,
    /// The hex code assigned to this role. If no hex code is assigned, the string will be empty.
    pub permissions: i64,
    /// A bitmask that represents the sum of all permissions granted to the role.
    pub color: String,
    /// Whether the role is publicly visible as a badge on user profiles.
    pub highlighted: bool,
}

impl UserRole {
    pub async fn get(user: User) -> Result<Option<UserRole>> {
        let role_id = user.role_id.unwrap();

        let role_tbl = crate::table::user_role::UserRole::get((
            "uid".to_string(),
            role_id,
        ))
        .await?;

        if role_tbl == None {
            return Ok(None);
        };

        let a = role_tbl.unwrap();
        let user_role = UserRole {
            uid: a.uid,
            name: a.name,
            permissions: a.permissions,
            color: a.color,
            highlighted: a.highlighted,
        };

        Ok(Some(user_role))
    }
}
