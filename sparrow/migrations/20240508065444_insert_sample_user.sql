-- For testing, use this to generate password: https://argon2.online/
--
-- salt: 'v5A3gAHhQonCEcbr'
-- $argon2id$v=19$m=65536,t=2,p=2$djVBM2dBSGhRb25DRWNicg$NtluEYAwi/l8YlxZ1j3HR8UoP8aLYpiu8RRW11sS8Lc
-- 36d96e1180308bf97c625c59d63dc747c5283fc68b6298aef11456d75b12f0b7
-- plain text: 'test'



INSERT INTO user (
    uid, 
    account_id,
    email, 
    admin,
    created_at,
    updated_at, 
    encrypted_password,
    role_id
) 
VALUES (
    '019019a3-71a7-7e70-8fb2-27741ab413d3',
    '018f56d7-bac8-7f76-a2eb-b089696db637',
    'seungjin@duck.com',
    true,
    1718416828,
    1718416828,
    '$argon2id$v=19$m=65536,t=2,p=2$djVBM2dBSGhRb25DRWNicg$NtluEYAwi/l8YlxZ1j3HR8UoP8aLYpiu8RRW11sS8Lc',
    '01907d0c-7327-7674-8d83-9991bf5916bd'
);

 