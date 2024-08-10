--
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

