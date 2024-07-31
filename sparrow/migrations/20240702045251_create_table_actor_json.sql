-- Add migration script here
CREATE TABLE actor_json (
    uid TEXT NOT NULL,
    actor_json TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);