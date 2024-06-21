-- Add migration script here


--     CREATE TABLE public.follows (
--     id bigint NOT NULL,
--     created_at timestamp without time zone NOT NULL,
--     updated_at timestamp without time zone NOT NULL,
--     account_id bigint NOT NULL,
--     target_account_id bigint NOT NULL,
--     show_reblogs boolean DEFAULT true NOT NULL,
--     uri character varying,
--     notify boolean DEFAULT false NOT NULL,
--     languages character varying[]
-- );


CREATE TABLE follow (
    uuid TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL,
    updated_at TIMESTAMP,
    account_id TEXT NOT NULL,
    target_account_id TEXT NOT NULL,
    show_reblogs BOOLEAN DEFAULT true NOT NULL,
    uri TEXT,
    notify BOOLEAN DEFAULT false NOT NULL,
    languages TEXT
);