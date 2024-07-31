-- Add migration script here

CREATE TABLE one_time_key (
    id INTEGER NOT NULL,
    device_id INTEGER,
    key_id TEXT DEFAULT '' NOT NULL,
    key TEXT DEFAULT '' NOT NULL,
    signature TEXS DEFAULT ''NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

