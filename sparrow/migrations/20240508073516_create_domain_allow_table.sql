-- Add migration script here
CREATE TABLE domain_allow (
    uuid TEXT NOT NULL,
    domain TEXT DEFAULT "" NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
