-- Add migration script here
CREATE TABLE mute (
    id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    hide_notifications BOOLEAN DEFAULT true NOT NULL,
    account_id TEXT NOT NULL,
    target_account_id TEXT NOT NULL,
    expires_at TIMESTAMP
);