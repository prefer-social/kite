-- -- Add migration script here
-- -- This pub/priv key are dev/test only. DO NOT USE IT FOR REAL.

INSERT INTO account (
    uuid,
    username,
    domain,
    private_key,
    public_key,
    created_at,
    updated_at,
    note, 
    display_name,
    uri,
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
    replace('-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCmUXrAJs39LUCQ\nrQSw+8hEvUr5V/MoqgH2oTILxdUaT8zv3Lo0wgRS7uITu78O9JTH3V6E8FMm0SqS\n/AwEbQrYZS9L8UUDuzf1bqZ33ldUo0ViRxmQc+NONoNTAaXYwWb1OTBbliGr+H29\nST7PpNieSjJVDncLNyWm/SNkpR34LYN7aANLHbG/H8Z4dAfSMfcMoer5lmSDPzuj\nhXKyxOPXfDupp+cJrOmx9GhbkcSC5WVoxQNJpKfP94iR2Uc2zwsDTzdlZzP96/fi\npoJtFooT8UXfzZQocSnimDaUYEEj7Ihq0KxpHf/bQ+ZTBv5804dXsxVMZ6uLkbXX\naPOK9D+jAgMBAAECggEAA0Mm4r0AHl60XQIfzO7rYznJTjHE8b+/8RwBVMzJHM6C\n6RONzXoexper9JIiscLnZjI35+ik4OaKbL6n57BR3aPCdT1iyqtx8gz9E+3XDbB0\nLHTTkgtL4p1cHxUxsRo+EEJSagF8YJirsrpMkpsbqqMu8I8rk5s90FQ3zQIWH+eQ\ne4ny3Lsw2I93W9y7Fmh2fKXVffuke32GSyV0vCZ8zpKk8f+xtqeiJM0DhTEDb+mu\nn0LiIjHrFBQMK4fF0EeRM/vKqWGkH5HIFWU1xSKGBXe52w7pHRYeeAY/NB6KHEip\nm0JCSc0ghvdqwHa+njRhHzyorghohdhQheK14nyRGQKBgQDp7arMDMIpS/GYJXqy\nER23RQC4lgZNyltTEwOOxY4T32IruXVK7PjEvCULawDr0d9oSoWlYoG5Su/vhIxG\n1Oytw0RSO9G5G5R4q8imU0QABMUnjpi26ipCeQtPDOvtkmmVz6kseLMtoZCGwj7g\nC2snycDXNKuhHdX+dg16+/MgKQKBgQC2AsAGBOTT4CZUsYy2NkP3EfI4f9CE0pQY\nObr4BN4zoPxi2naTR7/VnUJOEAYd6KkUumAloQb/P5pRWDDBC0e7b3ChBUeDmsSy\ndXBlpBHWCNmbuLVgrdnch8cCVzaLwyTGNmoBK/pwOk8pBhQTgAubRkxFuLmj0aLy\n+/JE12cq6wKBgQC++V+JjBTq3+wRWO+s9Zo752JqT1hAI6ag0JID+bY+9QxRQ+vE\nymdhPXREuQucec8YdlZbKWtcWRg+gwh+PxAl35ckR1+07hYs2ZlKaAnIthw6agi0\ns8DiWa1o0NS/M9ttCYZEHxmwdAjoayYnhErrxaLgTkiblR8ByWDB+XSH2QKBgGgg\nWKTY5eSExskLLloL34QYoz9JeAyUwB6/4EQF3O0ie2E+ZDnMAWZ+6jDon+c6dIik\nO18rqWobg/hngLTf9DC6B3BHKeuTflYuM5gibgXJ2FxrJnDh5NJyZxritBTvFjnD\nZY+4DjVoVmM80/138qvpYrc8gA/FXfiuxKI3rRDzAoGBALnSbJ+Ywnt5K2eAy37a\nYWkCWkaF3WYiSsFHbxN3c0cDMwkLoUSSdQYPiX8lDzqUR9GrCkOLPQCH0TUwQk1V\nkSKzHyTXbZZLSdp/F7ufJ2YxnHJY4q7RpMxGy3BM2aiAsdWk7W2t3OLMJfDedMMJ\nmWwYuCQknDBMdidvb7M6Mh7k\n-----END PRIVATE KEY-----','\n',char(10)),
    replace('-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAplF6wCbN/S1AkK0EsPvI\nRL1K+VfzKKoB9qEyC8XVGk/M79y6NMIEUu7iE7u/DvSUx91ehPBTJtEqkvwMBG0K\n2GUvS/FFA7s39W6md95XVKNFYkcZkHPjTjaDUwGl2MFm9TkwW5Yhq/h9vUk+z6TY\nnkoyVQ53Czclpv0jZKUd+C2De2gDSx2xvx/GeHQH0jH3DKHq+ZZkgz87o4VyssTj\n13w7qafnCazpsfRoW5HEguVlaMUDSaSnz/eIkdlHNs8LA083ZWcz/ev34qaCbRaK\nE/FF382UKHEp4pg2lGBBI+yIatCsaR3/20PmUwb+fNOHV7MVTGeri5G112jzivQ/\nowIDAQAB\n-----END PUBLIC KEY-----','\n',char(10)),
    (unixepoch()),
    (unixepoch()),
    'note here',
    'seungjin_display_name',
    'https://dev.prefer.social',
    'https://media-mstd.seungjin.net/accounts/avatars/109/737/937/659/013/254/original/626c9187e341632b.jpg',
    'https://media-mstd.seungjin.net/accounts/headers/109/737/937/659/013/254/original/9a714d77de20ae26.jpg',
    'https://dev.prefer.social/inbox',
    'https://dev.prefer.social/outbox',
    'https://dev.prefer.social/inbox',
    'https://dev.prefer.social/followers',
    'https://dev.prefer.social/following',
    "Person"
);