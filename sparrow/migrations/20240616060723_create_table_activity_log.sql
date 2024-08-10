-- Add migration script here
CREATE TABLE activity_log (
    uid TEXT NOT NULL,
    sig_header TEXT,
    headers TEXT, 
    hostname TEXT,
    method INTEGER, -- 0: get, 1: post
    body TEXT,
    status TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);





