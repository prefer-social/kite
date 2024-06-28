-- Add migration script here
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