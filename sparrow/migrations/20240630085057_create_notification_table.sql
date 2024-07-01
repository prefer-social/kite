-- Add migration script here

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
