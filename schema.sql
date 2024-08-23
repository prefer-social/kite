CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
CREATE TABLE account (
    uid TEXT NOT NULL,
    username TEXT DEFAULT "" NOT NULL,
    domain TEXT,
    private_key TEXT,
    public_key TEXT DEFAULT "" NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at INTEGER DEFAULT (unixepoch()) NOT NULL,
    note TEXT DEFAULT "" NOT NULL,
    display_name TEXT DEFAULT "" NOT NULL,
    uri TEXT DEFAULT "" NOT NULL,
    url TEXT,
    avatar_file_name TEXT,
    avatar_content_type TEXT,
    avatar_file_size INTEGER,
    avatar_updated_at INTEGER,
    header_file_name TEXT,
    header_content_type TEXT,
    header_file_size INTEGER,
    header_updated_at INTEGER,
    avatar_remote_url TEXT,
    locked BOOLEAN DEFAULT FALSE NOT NULL,
    header_remote_url TEXT,
    last_webfingered_at INTEGER,
    inbox_url TEXT DEFAULT "" NOT NULL,
    outbox_url TEXT DEFAULT "" NOT NULL,
    shared_inbox_url TEXT DEFAULT "" NOT NULL,
    followers_url TEXT DEFAULT "" NOT NULL,
    following_url TEXT DEFAULT "" NOT NULL,
    protocol INTEGER DEFAULT 0 NOT NULL,
    memorial BOOLEAN DEFAULT FALSE NOT NULL,
    moved_to_account_id bigint,
    featured_collection_url TEXT,
    fields jsonb,
    actor_type TEXT,
    discoverable BOOLEAN,
    also_known_as TEXT,
    silenced_at INTEGER,
    suspended_at INTEGER,
    hide_collections BOOLEAN,
    avatar_storage_schema_version INTEGER,
    header_storage_schema_version INTEGER,
    devices_url TEXT,
    suspension_origin INTEGER,
    sensitized_at INTEGER,
    trendable BOOLEAN,
    reviewed_at INTEGER,
    requested_review_at INTEGER,
    indexable BOOLEAN DEFAULT FALSE NOT NULL
);
CREATE TABLE user (
    uid TEXT NOT NULL,
    email TEXT DEFAULT "" NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    encrypted_password TEXT DEFAULT "" NOT NULL,
    reset_password_token TEXT,
    reset_password_sent_at INTEGER,
    sign_in_count INTEGER DEFAULT 0 NOT NULL,
    current_sign_in_at INTEGER,
    last_sign_in_at INTEGER,
    admin BOOLEAN DEFAULT FALSE NOT NULL,
    confirmation_token TEXT,
    confirmed_at INTEGER,
    confirmation_sent_at INTEGER,
    unconfirmed_email TEXT,
    locale TEXT,
    encrypted_otp_secret TEXT,
    encrypted_otp_secret_iv TEXT,
    encrypted_otp_secret_salt TEXT,
    consumed_timestep INTEGER,
    otp_required_for_login BOOLEAN DEFAULT FALSE NOT NULL,
    last_emailed_at INTEGER,
    otp_backup_codes TEXT,
    account_id TEXT NOT NULL,
    disabled BOOLEAN DEFAULT FALSE NOT NULL,
    moderator BOOLEAN DEFAULT FALSE NOT NULL,
    invite_id INTEGER,
    chosen_languages TEXT,
    created_by_application_id INTEGER,
    approved BOOLEAN DEFAULT true NOT NULL,
    sign_in_token TEXT,
    sign_in_token_sent_at INTEGER,
    webauthn_id TEXT,
    sign_up_ip TEXT,
    skip_sign_in_token BOOLEAN,
    role_id TEXT,
    settings TEXT,
    time_zone TEXT
);
CREATE TABLE domain_allow (
    uuid TEXT NOT NULL,
    domain TEXT DEFAULT "" NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
CREATE TABLE domain_block (
    uid TEXT NOT NULL,
    domain TEXT DEFAULT "" NOT NULL,
    created_at INTEGER DEFAULT (unixepoch()) NOT NULL,
    updated_at INTEGER DEFAULT (unixepoch()) NOT NULL,
    severity INTEGER DEFAULT 0,
    reject_media BOOLEAN DEFAULT FALSE NOT NULL,
    reject_reports BOOLEAN DEFAULT FALSE NOT NULL,
    private_comment TEXT,
    public_comment TEXT,
    obfuscate BOOLEAN DEFAULT FALSE NOT NULL
);
CREATE TABLE oauth_access_grant (
    uid INTEGER NOT NULL,
    token TEXT NOT NULL,
    expires_in INTEGER NOT NULL,
    redirect_uri TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    revoked_at INTEGER,
    scopes TEXT,
    application_id INTEGER NOT NULL,
    resource_owner_id INTEGER NOT NULL
);
CREATE TABLE oauth_access_token (
    uid TEXT NOT NULL,
    token TEXT NOT NULL,
    refresh_token TEXT,
    expires_in TIMESTAMP,
    revoked_at INTEGER,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    scopes TEXT,
    application_id TEXT,
    resource_owner_id TEXT,
    last_used_at INTEGER,
    last_used_ip TEXT
);
CREATE TABLE oauth_application (
    uid TEXT NOT NULL,
    name TEXT NOT NULL,
    secret TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    scopes TEXT DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    superapp BOOLEAN DEFAULT false NOT NULL,
    website TEXT,
    owner_type TEXT,
    owner_id TEXT,
    confidential boolean DEFAULT true NOT NULL
);
CREATE TABLE one_time_key (
    id INTEGER NOT NULL,
    device_id INTEGER,
    key_id TEXT DEFAULT '' NOT NULL,
    key TEXT DEFAULT '' NOT NULL,
    signature TEXS DEFAULT ''NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS "activity_log" (
    uid TEXT NOT NULL,
    sig_header TEXT,
    hostname TEXT, 
    body TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
, headers TEXT, status TEXT, method integer);
CREATE TABLE setting (
    var TEXT NOT NULL,
    value TEXT,
    thing_type TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    thing_id INTEGER
);
CREATE TABLE status (
    uid TEXT NOT NULL,
    uri TEXT,
    text TEXT DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    in_reply_to_id TEXT,
    reblog_of_id TEXT,
    url TEXT,
    sensitive BOOLEAN DEFAULT false NOT NULL,
    visibility INTEGER DEFAULT 0 NOT NULL,
    spoiler_text TEXT DEFAULT '' NOT NULL,
    reply BOOLEAN DEFAULT false NOT NULL,
    language TEXT,
    conversation_id TEXT,
    local BOOLEAN,
    account_id TEXT NOT NULL,
    application_id TEXT,
    in_reply_to_account_id TEXT,
    poll_id TEXT,
    deleted_at TIMESTAMP,
    edited_at TIMESTAMP,
    trendable BOOLEAN,
    ordered_media_attachment_ids TEXT
);
CREATE TABLE notification (
    uid TEXT NOT NULL,
    activity_id TEXT NOT NULL,
    activity_type TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP,
    account_id TEXT NOT NULL,
    from_account_id TEXT NOT NULL,
    type TEXT
);
CREATE TABLE conversation (
    uid TEXT NOT NULL,
    uri TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);
CREATE TABLE conversation_mute (
    uid TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    account_id TEXT NOT NULL
);
CREATE TABLE tag (
    uid TEXT NOT NULL,
    name TEXT DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    usable BOOLEAN,
    trendable BOOLEAN,
    listable BOOLEAN,
    reviewed_at TIMESTAMP,
    requested_review_at TIMESTAMP,
    last_status_at TIMESTAMP,
    max_score REAL,
    max_score_at TIMESTAMP,
    display_name TEXT
);
CREATE TABLE tag_follow (
    uid TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);
CREATE TABLE actor_json (
    uid TEXT NOT NULL,
    actor_json TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);
CREATE TABLE user_role (
    uid TEXT NOT NULL,
    name TEXT DEFAULT '' NOT NULL,
    color TEXT DEFAULT '' NOT NULL,
    position INTEGER DEFAULT 0 NOT NULL,
    permissions INTEGER DEFAULT 0 NOT NULL,
    highlighted BOOLEAN DEFAULT false NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);
CREATE TABLE mute (
    id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    hide_notifications BOOLEAN DEFAULT true NOT NULL,
    account_id TEXT NOT NULL,
    target_account_id TEXT NOT NULL,
    expires_at TIMESTAMP
);
CREATE TABLE media_attachments (
    uid TEXT NOT NULL,
    status_id TEXT,
    file_file_name TEXT,
    file_content_type TEXT,
    file_file_size i64,
    file_updated_at TIMESTAMP,
    remote_url TEXT DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    shortcode TEXT,
    type INTEGER DEFAULT 0 NOT NULL,
    file_meta TEXT,
    account_id TEXT,
    description TEXT,
    scheduled_status_id TEXT,
    blurhash TEXT,
    processing INTEGER,
    file_storage_schema_version INTEGER,
    thumbnail_file_name TEXT,
    thumbnail_content_type TEXT,
    thumbnail_file_size INTEGER,
    thumbnail_updated_at TIMESTAMP,
    thumbnail_remote_url TEXT
);
CREATE TABLE follow (
    uid TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP,
    account_uid TEXT NOT NULL,
    target_account_uid TEXT NOT NULL,
    show_reblogs BOOLEAN DEFAULT true NOT NULL,
    uri TEXT,
    notify BOOLEAN DEFAULT false NOT NULL,
    languages TEXT,
    UNIQUE(account_uid,target_account_uid)
);
CREATE VIEW instance AS
 WITH domain_counts(domain, account_count) AS (
         SELECT account.domain,
            count(*) AS account_count
           FROM account
          WHERE (account.domain IS NOT NULL)
          GROUP BY account.domain
        )
SELECT domain_counts.domain,domain_counts.account_count
FROM domain_counts
UNION
 SELECT domain_block.domain,
    COALESCE(domain_counts.account_count, 0) AS account_count
   FROM (domain_block
     LEFT JOIN domain_counts ON ((domain_counts.domain = domain_block.domain)))
UNION
 SELECT domain_allow.domain,
    COALESCE(domain_counts.account_count, 0) AS account_count
   FROM (domain_allow
     LEFT JOIN domain_counts ON ((domain_counts.domain = domain_allow.domain)))
/* instance(domain,account_count) */;
