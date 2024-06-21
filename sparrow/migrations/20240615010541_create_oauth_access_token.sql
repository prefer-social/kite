-- Add migration script here
CREATE TABLE oauth_access_token (
    id TEXT NOT NULL,
    token TEXT NOT NULL,
    refresh_token TEXT,
    expires_in TIMESTAMP,
    revoked_at INTEGER,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    scopes TEXT,
    application_id INTEGER,
    resource_owner_id INTEGER,
    last_used_at INTEGER,
    last_used_ip TEXT
);