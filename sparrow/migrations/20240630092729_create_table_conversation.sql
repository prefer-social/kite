-- Add migration script here
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