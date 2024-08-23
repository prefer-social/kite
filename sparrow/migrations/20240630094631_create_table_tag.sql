-- Add migration script here


CREATE TABLE tag (
    uid TEXT NOT NULL,
    name TEXT DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    usable BOOLEAN,
    trendable BOOLEAN,
    listable BOOLEAN,
    reviewed_at TIMESTAMP,
    requested_review_at TIMESTAMP,
    last_status_at TIMESTAMP,
    max_score REAL,
    max_score_at TIMESTAMP,
    display_name TEXT
);

CREATE TABLE tag_follow (
    uid TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);