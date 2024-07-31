-- Add migration script here

CREATE TABLE setting (
    var TEXT NOT NULL,
    value TEXT,
    thing_type TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    thing_id INTEGER
);
