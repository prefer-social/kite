## how to migrate database/sqlite.
```
cargo sqlx database create --database-url=sqlite://.spin/sqlite_db.db
cargo sqlx migrate run --database-url=sqlite://.spin/sqlite_db.db
cargo sqlx database reset
```


## How to create your own dev pub/priv key 
```
openssl genrsa -out private.pem 2048
openssl rsa -in private.pem -outform PEM -pubout -out public.pem
```