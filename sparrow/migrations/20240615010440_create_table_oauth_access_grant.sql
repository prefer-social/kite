-- Add migration script here
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