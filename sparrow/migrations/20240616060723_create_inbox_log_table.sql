-- Add migration script here
CREATE TABLE inbox_log (
    uuid TEXT NOT NULL,
    valid_sig TEXT,
    sig_header TEXT,
    hostname TEXT, 
    body TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);





