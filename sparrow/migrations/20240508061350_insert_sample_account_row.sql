-- -- Add migration script here
-- -- This pub/priv key are dev/test only. DO NOT USE IT FOR REAL.

INSERT INTO account (
    uid,
    username,
    domain,
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
    'dev.prefer.social',
    replace('-----BEGIN PRIVATE KEY-----\nFOO\nBAR\n-----END PRIVATE KEY-----','\n',char(10)),
    replace('-----BEGIN PUBLIC KEY-----\nFOO\nBAR\n-----END PUBLIC KEY-----','\n',char(10)),
    (unixepoch()),
    (unixepoch()),
    'note here',
    'seungjin_display_name',
    'https://dev.prefer.social',
    'https://dev.prefer.social/@seungjin',
    'https://media-mstd.seungjin.net/accounts/avatars/109/737/937/659/013/254/original/626c9187e341632b.jpg',
    'https://media-mstd.seungjin.net/accounts/headers/109/737/937/659/013/254/original/9a714d77de20ae26.jpg',
    'https://dev.prefer.social/inbox',
    'https://dev.prefer.social/outbox',
    'https://dev.prefer.social/inbox',
    'https://dev.prefer.social/followers',
    'https://dev.prefer.social/following',
    "Person"
);
