-- -- Add migration script here
-- -- This pub/priv key are dev/test only. DO NOT USE IT FOR REAL.

INSERT INTO account (
    uid,
    username,
    private_key,
    public_key,
    created_at,
    updated_at,
    note, 
    display_name,
    uri,
    url,
    avatar_remote_url,
    header_remote_url,
    inbox_url,
    outbox_url,
    shared_inbox_url,
    followers_url,
    following_url,
    actor_type
) VALUES (
    '018f56d7-bac8-7f76-a2eb-b089696db637',
    'seungjin',
    '-----BEGIN PRIVATE KEY----- ??? -----END PRIVATE KEY-----',
    '-----BEGIN PUBLIC KEY----- ??? -----END PUBLIC KEY-----',
    (unixepoch()),
    (unixepoch()),
    'note here',
    'seungjin_display_name',
    'https://dev.prefer.social/self',
    'https://dev.prefer.social',
    'https://dev.prefer.social/assets/icon.jpg',
    'https://dev.prefer.social/assets/background.jpg',
    'https://dev.prefer.social/inbox',
    'https://dev.prefer.social/outbox',
    'https://dev.prefer.social/inbox',
    'https://dev.prefer.social/followers',
    'https://dev.prefer.social/following',
    "Person"
);
