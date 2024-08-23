-- Add migration script here
CREATE TABLE activity_log (
    uid TEXT NOT NULL,
    sig_header TEXT,
    headers TEXT, 
<<<<<<< HEAD
    hostname TEXT, 
=======
    hostname TEXT,
    method INTEGER, -- 0: get, 1: post
>>>>>>> 20adcdf955a016e90b8884496fc561f717b516ac
    body TEXT,
    status TEXT,
    created_at TIMESTAMP DEFAULT (unixepoch()) NOT NULL
);





