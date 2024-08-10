-- Add migration script here
CREATE TABLE user_role (
    uid TEXT NOT NULL,
    name TEXT DEFAULT '' NOT NULL,
    color TEXT DEFAULT '' NOT NULL,
    position INTEGER DEFAULT 0 NOT NULL,
    permissions INTEGER DEFAULT 0 NOT NULL,
    highlighted BOOLEAN DEFAULT false NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);