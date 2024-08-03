use chrono::{DateTime, Utc};

pub struct MediaAttachement {
    pub uid: String,
    pub status_id: String,
    pub file_file_name: String,
    pub file_content_type: String,
    pub file_file_size: i64,
    pub file_updated_at: DateTime<Utc>,
    pub remote_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub shortcode: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub media_type: i64,
    pub file_meta: String,
    pub account_id: String,
    pub description: String,
    pub scheduled_status_id: String,
    pub blurhash: String,
    pub processing: i64,
    pub file_storage_schema_version: i64,
    pub thumbnail_file_name: String,
    pub thumbnail_content_type: String,
    pub thumbnail_file_size: i64,
    pub thumbnail_updated_at: DateTime<Utc>,
    pub thumbnail_remote_url: String,
}

#[async_trait]
pub trait Get<T> {
    async fn get(arg: T) -> Result<Vec<MediaAttachement>>;
}

/// Very generic table Get function
/// Geting (key: String, val: String).
/// This goes `SELECT * FROM some_table WHERER key = val`
#[async_trait]
impl Get<(String, String)> for MediaAttachement {
    async fn get(
        (key, val): (String, String),
    ) -> Result<Vec<MediaAttachement>> {
        let query_template = format!(
            "SELECT rowid, * FROM media_attachement WHERE {} = ?",
            key
        );
        let sqlx_conn = dbcon::open_default()?;
        let accounts = sqlx::query_as(query_template.as_str())
            .bind(val)
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(accounts)
    }
}
