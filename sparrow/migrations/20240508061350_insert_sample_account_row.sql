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
    replace('-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCbbKQY3Z1hqnl/\n65pINIViSZG9rWcCjJWVf31TAqR5ern7nKeJwaj0ZQeqiCH5zOzfn8+p2+i8Q+wI\ndCD57zVLGaoExmnZ2sMvDElv1RonFeU53zch1LcOWgUpfEpKrkFut47eKPbusVRs\nTNlZbF8zrQW9N2C1Lim9IgirpX/ykgCt+8223fQ2vf9Ggy93JJNc7L0hj940fYA5\nQJKsbte6nwnv7O2Z6QTp76L3ev/GfTnPgBAbgmHbfHdlURoFP0Yv6v37YP9y9+GL\n3Wa3mD2vBhsTvGeLbTtxRoyN7cvvGvN505TTH3qyGcRIhPO6WXUtXdfO6fkB8beD\ncFCBk2lNAgMBAAECggEAB70VjWCZCCnD44dYehuvWbD329D19FqjZtTfT98T6TG5\nb+c6/X3SF6hHuCLcejXmkQEOhtKtvseTYx2RHl13Ze/EF0YsmadhGSCyyVz71xIp\nt/alJcvHEr5cGNaY7MLKldfEejHhwnAiF90nSnxxLKWcewrcuriuy4pI+NVRNRBV\nfgfm+6qnYlldAs5G8SnDL5Z0K3Sv+JCpH3x9qZUyP1REeU9osoER/m3lBmNBcFd8\nEfmQmIyBZhKURUkn9Ytbo6nX4UomXGs801GI+0y5WxeOfAdt/1MQXBtdZrybBqaN\nJyo7o7ca86UVpWQttaU0JyJiMFsYR0oRVh1XJIV2kQKBgQDJNs/ZD088CBhNbKoB\n8pQWXV3MRtzptEIXl9UzUFly6LkTHWdC/LvOpHMG60iLPIfJRRfXMLOHt868w2L2\nMeIS/GpmWqknJO3Ic9XidDwa9LBTN+5Y3hmriI+UifFJfwM/EL66bChsaEcRvJc8\nfkbhkWyZNE1fRMGBMgkXQyZLOQKBgQDFvigKLzkvnmxhTkICxABLr8bBccN/rCPF\nkfSLM4jRwfyjB0h6+e0huYEDrXdz20gCraE4ghjASHGFU5O+LI5SGiXNlc+SsCcj\nSdGYjGAr797VBCz7v/xO1aYRw4NdhXUGOFDwn8j5oLT255NQdZfgUUibpWh7ROSI\nzgsXfRAKtQKBgFP1bRDzVGqGLi3QU7fwUl5qowLj5rYVoyZZEnsQ4eISvXb+Y+IN\ntwfokp6memSu4zHWhLYj+iV9SuR02sxD0EN/053V4pMU06s6ulp9ib7trs1qnyex\ntSBgIRkn3eh++QnNoOgEzXE+qbj66lgxkL36gkgDDO3dsKiKCkUMnePxAoGBAMLH\no5Mcnxzb1XAn8Pfeb1geZg5obCUXwAi9039GnQ/ZIGzKIMgZ5U4h9wZTWWXC6vTQ\naQ1LgOvur/ufrtlV9D7FSNVwKw4X/AVTkmTMaXL+DaYQPeshWnuhEbtPtddqPTKK\nSJgtHH9foMHQrBhGLPsltfqy/O7nlqF3LA/P/fw9AoGBALqyXM9AtOLRoakr9bNE\nik6N8nMWgWakkDyupGqLaSz89SS12bsPsDJ2ZwsEOZLfN1n/sz2BvpFMND8iHGzF\ncB26wvlkMO5C4L7HHKO/lDJS2jV4fOxhWwsVp3LpgEdGxqAtAF1N9AaMVtiavKJA\nVpgcbkQZsvcQwQcCKfrK5yO+\n-----END PRIVATE KEY-----','\n',char(10)),
    replace('-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAm2ykGN2dYap5f+uaSDSF\nYkmRva1nAoyVlX99UwKkeXq5+5ynicGo9GUHqogh+czs35/PqdvovEPsCHQg+e81\nSxmqBMZp2drDLwxJb9UaJxXlOd83IdS3DloFKXxKSq5BbreO3ij27rFUbEzZWWxf\nM60FvTdgtS4pvSIIq6V/8pIArfvNtt30Nr3/RoMvdySTXOy9IY/eNH2AOUCSrG7X\nup8J7+ztmekE6e+i93r/xn05z4AQG4Jh23x3ZVEaBT9GL+r9+2D/cvfhi91mt5g9\nrwYbE7xni207cUaMje3L7xrzedOU0x96shnESITzull1LV3Xzun5AfG3g3BQgZNp\nTQIDAQAB\n-----END PUBLIC KEY-----','\n',char(10)),
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