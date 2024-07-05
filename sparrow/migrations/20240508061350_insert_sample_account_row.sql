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
    '-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC7pENJmciS+eUf
VV6hbbzo0xq98HMvl1vClLv2YdkN1nSB0ER+XxwjQLWOvGBzLI10psjQIMAUPrug
q+awuL987giPzV2anuTm6QFdxqNEpkeSmcZxVBL6ITpw30lEIE89f5b3/t9MkBl+
Qvl1u9C4gCM4WJ+KiVIBpfGInHJ2aQgVNwj8EBefOWBALZgmUNUJZjN6EpNGTtTv
J2X5tF6ffqB4EnvTyqIZJbzAHguSDp6mk8hC02ucScQW+mdTnGCtZfLvmvkyEigo
ivD3p/gbFFVcwTTvu4loIqFdAfRI+TWniL7gtyYk73AO95GGz47i+L2q9mKSmf/q
lv2WozpJAgMBAAECggEACRf8P1VFvlLATwkP/56N/tIZTA56xJoPI0JaTCXgky47
If3uDdpN5qr4iZD5Uyeb+OTq20RVDr4eEXq3QMYJWLSAA3kaUdekxAiTjYAGklmF
IzaS4pPmogrpdkYp+rRepr432q+2UnNy2tbD2Hf4VPK30j+gRsJiMCyCxMJBPVoD
Ip/fN2/vOYWjetsZ4U9AjTlOgZdhz3WWUInwPYLFilEG9vY/cVEOS1nvebWaWuD0
DVSNudvqV35N2F2VgBDlkx1wwdwdzWfL1xYxYJSlDw08oz556QUzBEK7Iswf58Gl
7ANeFfdG/tKRv4noZ++KYfhobnzoiyEBDhsuVPuJ0QKBgQDzlcCRSxCP2F8XOcSj
qTl4v6ksRX029WNV6tqRRVfn412Tje2vxd+pYCrEm4ZHAJl+LppfvmDlivg0LBra
QPo+8pY8oROhrH62XNc1CbzKKFRyAU7uXfMai5ziscgBed0o9Ddv7fI8e+TSELVP
xtXp9WVq9VcrxRlwMMmCIG64WQKBgQDFNJLNOY4PksuTSiD1gNvY+vOwA1W+a7U9
S9+25W0iU/yYkUN70hJLzMJxAMoZJAGZteXWdvST9WYMRY40cxda58aQaPS5mg+X
Uzh1DHRphuvhleSybzyGuP9hblI0Bm2S1p4p9KTln0iS4bQKSeNu3SEovqRcJf0T
YRTzUBdTcQKBgQCpPCkviSS3nA/tFhC92Iv6jcbe4GSvs2vRvLsgZTo+/SP9qYUz
M1pC1nLAaIGxgQQjvAM7dAzkTlpF/UKm0rBK3cZt10fJcwwSbYyD6AEHv190Wd5a
FGIb1lYo1nXsOJvkoVCG7p33UejX3g3qoG3s/7b3yf9HhMg4v1yxkRD7+QKBgHTC
ywSNxkn8PWKI6njRgdm3xB776oGjXhzb1qqArpw7o/xlTdJ7L/AHXTzxHbfC9Nug
smPH7MnJ61DqwebWGgbt4khQjE10JTUQxgWviSGODA115YGskOanlDWIz00UXT4q
qlznB9cY9yxYTHQnCWzzqpVoQwJ4T55wxanGSvlRAoGBAIZh5Fy0eJv+y8irt0BU
Um23c77YcYYxdRCLRnXMzOWezcGBmRmCdpLhzc+v6FFzfEoq4xdF0UvmohKsufkr
0MyTyxBhMKN6lOJHjEE6dgMEKhdhB77+Omahu8OBDtIfkiD3UYcar8SuyXETpBvn
MWhMjXp45K2seJpthP09hOsV
-----END PRIVATE KEY-----',
    '-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAu6RDSZnIkvnlH1VeoW28
6NMavfBzL5dbwpS79mHZDdZ0gdBEfl8cI0C1jrxgcyyNdKbI0CDAFD67oKvmsLi/
fO4Ij81dmp7k5ukBXcajRKZHkpnGcVQS+iE6cN9JRCBPPX+W9/7fTJAZfkL5dbvQ
uIAjOFifiolSAaXxiJxydmkIFTcI/BAXnzlgQC2YJlDVCWYzehKTRk7U7ydl+bRe
n36geBJ708qiGSW8wB4Lkg6eppPIQtNrnEnEFvpnU5xgrWXy75r5MhIoKIrw96f4
GxRVXME077uJaCKhXQH0SPk1p4i+4LcmJO9wDveRhs+O4vi9qvZikpn/6pb9lqM6
SQIDAQAB
-----END PUBLIC KEY-----',
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
