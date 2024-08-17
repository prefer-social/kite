//! account table
//!

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_sdk::sqlite::{Connection, Value};
use spin_sqlx::sqlite::Connection as dbcon;
use std::any::type_name;
use std::time::{SystemTime, UNIX_EPOCH};
use struct_iterable::Iterable;
use url::Url;

use crate::activitystream::actor::person::Person as PersonActor;
use crate::mastodon::account::actor_url::ActorUrl;
use crate::mastodon::account::uri::Uri as AccountUri;
use crate::table::FieldType;

/// DB Account table struct
#[derive(
    Serialize,
    Deserialize,
    Default,
    Clone,
    Debug,
    PartialEq,
    sqlx::FromRow,
    Iterable,
)]
pub struct Account {
    /// rowid from sqlite, If rowid is negative < 0, It's not from table.
    pub rowid: i64,
    /// uid, uuid v7 format. Also when transformed to json, this filed becomes `id`
    #[serde(rename(serialize = "id", deserialize = "id"))]
    pub uid: String, // not null, primary key
    /// username, default value is "" and not null
    pub username: String,
    /// domain. Local user when None
    pub domain: Option<String>,
    /// private pem key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// public pem key, not null, default is ""
    pub public_key: String,
    /// when field was crated not null, Unix time
    pub created_at: i64,
    /// when field was updated, not null, Unix time
    pub updated_at: i64,
    /// Note filed, default "", not null
    pub note: String,
    /// display name, default "", not null
    pub display_name: String,
    /// actor(activity+json)'s URL, http//mastodon.com/users/userme   
    /// uri, default "", not null
    pub uri: String,
    /// profile(text/html)'s URL   
    pub url: Option<String>,
    pub avatar_file_name: Option<String>,
    pub avatar_content_type: Option<String>,
    pub avatar_file_size: Option<i64>,
    pub avatar_updated_at: Option<i64>,
    pub header_file_name: Option<String>,
    pub header_content_type: Option<String>,
    pub header_file_size: Option<i64>,
    pub header_updated_at: Option<i64>,
    pub avatar_remote_url: Option<String>,
    /// default(FALSE), not null
    pub locked: Option<i64>,
    /// default(""), not null
    pub header_remote_url: Option<String>,
    pub last_webfingered_at: Option<i64>,
    /// default(""), not null
    pub inbox_url: Option<String>,
    /// default(""), not null
    pub outbox_url: Option<String>,
    /// default(""), not null
    pub shared_inbox_url: Option<String>,
    /// default(""), not null
    pub following_url: Option<String>,
    /// default(""), not null
    pub followers_url: Option<String>,
    /// default("ostatus"), not null
    pub protocol: Option<i64>,
    /// default(FALSE), not null
    pub memorial: Option<i64>,
    pub moved_to_account_id: Option<i64>,
    pub featured_collection_url: Option<String>,
    pub fields: Option<String>,
    pub actor_type: Option<String>,
    pub discoverable: Option<i64>,
    /// is an Array
    pub also_known_as: Option<String>,
    pub silenced_at: Option<i64>,
    pub suspended_at: Option<i64>,
    pub hide_collections: Option<i64>,
    pub avatar_storage_schema_version: Option<i64>,
    pub header_storage_schema_version: Option<i64>,
    pub devices_url: Option<String>,
    pub suspension_origin: Option<i64>,
    pub sensitized_at: Option<i64>,
    pub trendable: Option<i64>,
    pub reviewed_at: Option<i64>,
    pub requested_review_at: Option<i64>,
    /// default(FALSE), not null
    pub indexable: Option<i64>,
}

type TAccount = Account;

impl Account {
    /// returns all Account rows
    pub async fn all() -> Result<Vec<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<Account> =
            sqlx::query_as("SELECT rowid, * FROM account")
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(accounts)
    }

    /// To get federation_id
    pub async fn federation_id(self: &Self) -> Result<String> {
        let username = self.username.clone();
        let domain = self.domain.clone();
        Ok(format!("{}@{}", username, domain.unwrap()))
    }

    /// Get Account struct by account(username and domain)
    pub async fn fr_username_domain(
        username: String,
        domain: Option<String>,
    ) -> Result<Option<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<Account> = match domain.is_none() {
            true => {
                sqlx::query_as("SELECT rowid, * FROM account WHERE username = ? AND domain IS NULL")
                    .bind(username)
                    .fetch_all(&sqlx_conn)
                    .await?
            }
            _ => {
                sqlx::query_as("SELECT rowid, * FROM account WHERE username = ? AND domain = ?")
                    .bind(username)
                    .bind(domain)
                    .fetch_all(&sqlx_conn)
                    .await?
            }
        };

        Ok(Some(accounts.last().unwrap().to_owned()))
    }

    /// Get Account struct from Account's Uri
    pub async fn fr_account_uri(
        account_uri: AccountUri,
    ) -> Result<Option<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<Account> = sqlx::query_as(
            "SELECT rowid, * FROM account WHERE username = ? AND domain = ?",
        )
        .bind(account_uri.username)
        .bind(account_uri.domain)
        .fetch_all(&sqlx_conn)
        .await?;
        Ok(Some(accounts.first().unwrap().to_owned()))
    }

    /// Get Accounf with oauth access token.  
    pub async fn fr_token(token: String) -> Result<Vec<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<Account> = sqlx::query_as(
            "SELECT account.rowid, account.* FROM oauth_access_token AS TOKEN INNER JOIN account ON TOKEN.resource_owner_id = account.uid WHERE TOKEN.token = ?",
        ).bind(token).fetch_all(&sqlx_conn).await?;
        Ok(accounts)
    }

    /// Get TAccount from actor url. actor's url is TAccount's uri.
    pub async fn fr_actor_url(url: String) -> Result<Vec<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<Account> =
            sqlx::query_as("SELECT account.rowid, account.* FROM account WHERE account.uri = ?")
                .bind(url)
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(accounts)
    }

    /// Check account already existed in datbase table.  
    pub async fn is_exist(username: String, domain: String) -> Result<bool> {
        let connection = Connection::open_default()?;
        let execute_params = [
            Value::Text(username.to_owned()),
            Value::Text(domain.to_owned()),
        ];
        let count = connection.execute(
            "SELECT count(*) AS cnt FROM account WHERE account.username = ? AND account.domain = ?",
            execute_params.as_slice(),
        )?;
        let cnt = count.rows().next().unwrap().get::<i64>("cnt").unwrap();
        Ok(cnt > 0)
    }
}

/// I am a trait Get<T>
#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<Account>>;
}

/// Very generic table Get function
/// Geting (key: String, val: String).
/// This goes `SELECT * FROM some_table WHERER key = val`
#[async_trait]
impl Get<(String, String)> for Account {
    async fn get((key, val): (String, String)) -> Result<Vec<Account>> {
        let query_template =
            format!("SELECT rowid, * FROM account WHERE {} = ?", key);

        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}

#[async_trait]
impl Get<AccountUri> for Account {
    async fn get(uri: AccountUri) -> Result<Vec<Account>> {
        let sqlx_conn = dbcon::open_default()?;
        let accounts: Vec<TAccount> = match uri.domain {
            Some(domain) => {
                let query_template =
                    format!("SELECT rowid, * FROM account WHERE username = ? AND domain = ?");
                sqlx::query_as(query_template.as_str())
                    .bind(uri.username)
                    .bind(domain)
                    .fetch_all(&sqlx_conn)
                    .await?
            }
            None => {
                let query_template =
                    format!("SELECT rowid, * FROM account WHERE username = ? AND domain IS NULL");
                sqlx::query_as(query_template.as_str())
                    .bind(uri.username)
                    .fetch_all(&sqlx_conn)
                    .await?
            }
        };

        Ok(accounts)
    }
}

/// Trait Put: Inserting into account table.  
#[async_trait(?Send)]
pub trait New<T> {
    /// Inserting into account table.  
    async fn new(arg: T) -> Result<()>;

    /// Update account table
    async fn update(arg: T) -> Result<()>;
}

#[async_trait(?Send)]
impl New<PersonActor> for Account {
    async fn new(actor: PersonActor) -> Result<()> {
        // Todo: Convert PersonActor -> TAccount -> Insert TAccount
        // try_from -> put

        let account = Self::try_from(actor).unwrap();
        Self::new(account).await?;

        Ok(())
    }

    async fn update(_actor: PersonActor) -> Result<()> {
        // Todo: Here
        Ok(())
    }
}

#[async_trait(?Send)]
impl New<Account> for Account {
    async fn new(tacct: Account) -> Result<()> {
        // If account already in table, do update

        let username = tacct.username.clone();
        let domain = tacct.domain.clone().unwrap();

        if Self::is_exist(username, domain).await? {
            // do update instead of insert
            return Self::update(tacct).await;
        }

        let mut fields = "(".to_string();
        let mut value_mark = "(".to_string();
        let mut values = Vec::new();
        let mut execute_params = Vec::new();
        for (k, v) in tacct.iter() {
            // Ignore rowid
            if k == "rowid" {
                continue;
            }

            let value = match super::check_type(v) {
                FieldType::String => {
                    v.downcast_ref::<String>().unwrap().to_owned()
                }
                FieldType::OptionString => {
                    let a = &v.downcast_ref::<Option<String>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap()
                }
                FieldType::I64 => v.downcast_ref::<i64>().unwrap().to_string(),
                FieldType::OptionI64 => {
                    let a = &v.downcast_ref::<Option<i64>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap().to_string()
                }
                FieldType::I64 => v.downcast_ref::<i64>().unwrap().to_string(),
                FieldType::OptionI64 => {
                    let a = &v.downcast_ref::<Option<i64>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap().to_string()
                }
                FieldType::F64 => v.downcast_ref::<f64>().unwrap().to_string(),
                FieldType::OptionF64 => {
                    let a = &v.downcast_ref::<Option<f64>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap().to_string()
                }
                _ => "".to_string(),
            };

            fields.push_str(k);
            fields.push_str(", ");
            value_mark.push_str("?, ");

            values.push(value.clone());
            execute_params.push(Value::Text(value));
        }

        fields.pop();
        fields.pop();
        fields.push_str(")");
        value_mark.pop();
        value_mark.pop();
        value_mark.push_str(")");

        let sql_insert =
            format!("INSERT INTO account {} VALUES {}", fields, value_mark);

        let connection = Connection::open_default()?;
        connection.execute(sql_insert.as_str(), execute_params.as_slice())?;

        Ok(())
    }

    async fn update(tacct: Account) -> Result<()> {
        // SQL UPDATE
        // UPDATE table_name
        // SET column1 = value1, column2 = value2, ...
        // WHERE condition;

        let mut sets = "".to_string();
        let mut execute_params = Vec::new();
        for (k, v) in tacct.iter() {
            // Todo: Passing Account struct should not include skip_fields fileds at creation time. Check MAccount
            let skip_fileds = vec!["rowid", "uid", "created_at"];
            if skip_fileds.contains(&k) {
                continue;
            }

            let value = match super::check_type(v) {
                FieldType::String => {
                    v.downcast_ref::<String>().unwrap().to_owned()
                }
                FieldType::OptionString => {
                    let a = &v.downcast_ref::<Option<String>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap()
                }
                FieldType::I64 => v.downcast_ref::<i64>().unwrap().to_string(),
                FieldType::OptionI64 => {
                    let a = &v.downcast_ref::<Option<i64>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap().to_string()
                }
                FieldType::F64 => v.downcast_ref::<f64>().unwrap().to_string(),
                FieldType::OptionF64 => {
                    let a = &v.downcast_ref::<Option<f64>>();
                    if a.unwrap().is_none() {
                        continue;
                    };
                    a.unwrap().to_owned().unwrap().to_string()
                }
                _ => "".to_string(),
            };

            sets.push_str(format!("{} = ?, ", k).as_str());

            execute_params.push(Value::Text(value));
        }
        let current_unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        sets.push_str("updated_at = ?");
        execute_params.push(Value::Integer(current_unix_time));

        let username = tacct.username.clone();
        let domain = tacct.domain.clone().unwrap();

        execute_params.push(Value::Text(username));
        execute_params.push(Value::Text(domain));

        let sql_stmt = format!(
            "UPDATE account SET {} WHERE account.username = ? AND account.domain = ?",
            sets
        );

        let connection = Connection::open_default()?;
        connection.execute(sql_stmt.as_str(), execute_params.as_slice())?;

        Ok(())
    }
}

impl TryFrom<PersonActor> for Account {
    type Error = anyhow::Error;

    /// (Person)Actor to Account
    fn try_from(actor: PersonActor) -> Result<Self, Self::Error> {
        let current_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let avatar_remote_url = match &actor.icon {
            Some(i) => Some(i.to_owned().url),
            None => None,
        };

        //let avatar_content_type = match &actor.icon {
        //    Some(i) => Some(i.to_owned().media_type),
        //    None => None,
        //};
        let avatar_content_type = actor
            .icon
            .to_owned()
            .map(|x| x.to_owned().media_type)
            .unwrap_or_default();

        let header_remote_url = match &actor.image {
            Some(i) => Some(i.to_owned().url),
            None => None,
        };

        //let header_content_type = match &actor.image {
        //    Some(i) => Some(i.to_owned().media_type),
        //    None => None,
        //};
        let header_content_type = actor
            .icon
            .to_owned()
            .map(|x| x.to_owned().media_type)
            .unwrap_or_default();

        let account = Account {
            uid: uuid::Uuid::now_v7().to_string(),
            username: actor.preferred_username.to_lowercase(),
            domain: Some(
                Url::parse(actor.id.to_lowercase().as_str())
                    .unwrap()
                    .domain()
                    .unwrap()
                    .to_string(),
            ),
            public_key: actor.public_key.public_key_pem,
            created_at: current_epoch, // not null
            updated_at: current_epoch, // not null
            note: actor.summary.unwrap_or("".to_string()), // default(""), not null
            display_name: actor.name, // default(""), not null
            uri: actor.id,            // default(""), not null
            url: Some(actor.url),
            avatar_content_type: avatar_content_type,
            header_content_type: header_content_type,
            avatar_remote_url: avatar_remote_url,
            header_remote_url: header_remote_url,
            last_webfingered_at: Some(current_epoch),
            inbox_url: Some(actor.inbox),
            outbox_url: Some(actor.outbox),
            shared_inbox_url: Some(actor.endpoints.shared_inbox), // default(""), not null
            following_url: Some(actor.following),
            followers_url: Some(actor.followers), // default(""), not null
            memorial: actor.memorial.map(|x| match x {
                false => 0,
                true => 1,
            }),
            featured_collection_url: actor.featured,
            actor_type: Some(actor.actor_type.to_string()),
            discoverable: actor.discoverable.map(|x| match x {
                false => 0,
                true => 1,
            }),
            devices_url: actor.devices,
            indexable: actor.indexable.map(|x| match x {
                false => 0,
                true => 1,
            }),
            ..Default::default() // default(FALSE), not null
        };
        Ok(account)
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[async_trait]
pub trait Remove<T> {
    async fn remove(arg: T) -> Result<()>;
}

#[async_trait]
impl Remove<ActorUrl> for Account {
    async fn remove(actor_url: ActorUrl) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;
        let query_template = format!("DELETE FROM account WHERE uri = ?");
        sqlx::query(query_template.as_str())
            .bind(actor_url.to_string())
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Remove<AccountUri> for Account {
    async fn remove(account_uri: AccountUri) -> Result<()> {
        let sqlx_conn = dbcon::open_default()?;
        let query_template =
            format!("DELETE FROM account WHERE account.username = ? AND account.domain = ?");
        sqlx::query(query_template.as_str())
            .bind(account_uri.username)
            .bind(account_uri.domain.unwrap())
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(())
    }
}
