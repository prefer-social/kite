-- Add migration script here
CREATE TABLE domain_block (
    uuid TEXT NOT NULL,
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
