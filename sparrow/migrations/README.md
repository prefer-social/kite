how to migrate database/sqlite.

```
cargo sqlx database create --database-url=sqlite://.spin/sqlite_db.db
cargo sqlx migrate run --database-url=sqlite://.spin/sqlite_db.db
```