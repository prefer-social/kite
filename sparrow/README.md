TODO: Write README.md


How to create db: 
cargo sqlx database create -D sqlite://../kite.db

how to migrate db tables:
cargo sqlx migrate run -D sqlite://../kite.db