# rust-web-server-example
  learn rust web server and crates.

## Structure
- api: web request & response
    - warp
    - axum
- domain: business service & impl
- db: wrap database operation & sql
    - postgresql
    - sqlx
- types: type for all module
- server: boot & logger
    - flexi_logger
- redis: nosql db
    - redis
- search: search engine
    - meilisearch
- auth: jwt token auth
    - jwt web token

## Prepare
- Install Rust 1.56+
- Install PostgreSQL 13+
- Install MeiliSearch
- Install Redis, optional


## How to use sqlx migrations
```shell
# install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres
# create database from .env file of current workspace
# add correct DATABASE_URL=..., no need create in database, sqlx will create it
sqlx database create
# create <timestamp>_my_example.up.sql and <timestamp>_my_example.down.sql
sqlx migrate add -r my_example
# run sql from <timestamp>.up.sql
# multi sql file, it's like going into the stack, all at once
sqlx migrate run
# revert sql from <timestamp>.down.sql
# only the most recent can be revert at a time, it's like going out of the stack
sqlx migrate revert
```
#### sql raw
```sql
-- add column
ALTER TABLE table_name ADD COLUMN IF NOT EXISTS column_name column_type NOT NULL DEFAULT default_value;
-- add column comment
COMMENT ON COLUMN table_name.column_name IS column_comment;
-- drop column
ALTER TABLE table_name DROP COLUMN column_name;
-- modify column type
ALTER TABLE table_name ALTER COLUMN column_name TYPE column_type;
```

## How to run
- Open `.env` file
- Config `DATABASE_URL`
    - run shell command `sqlx database create`
    - run shell command `sqlx migrate run`
- Config `MEILISEARCH_URL` and `MEILISEARCH_KEY`
- Add some data into database
- Sync data from database to meilisearch
    - sync, see [meilisearch-readme](search/README.md) `Sync from db to meilisearch` section.
    - extra search conditions, see [meilisearch-readme](search/README.md) `settings` section.
- `cargo build --release`
- `cd target/release`
- `./app`

## Docker
```shell
docker build -t rust-web-server-example:0.1.0 .

docker run -d -p 3030:3030 \
  -v /data/app/log:/apps/log/rust \
  --name app rust-web-server-example:0.1.0

# or add custom env, replace .env file
# docker run -d -p 3030:3030 \
#   -v /data/app/log:/apps/log/rust \
#   -e DATABASE_URL=<type your db url> \
#   -e BIND_ADDRESS=<type your server address> \
#   -e REDIS_URL=<type your redis url> \
#   -e MEILISEARCH_URL=<type your search url> \
#   -e MEILISEARCH_KEY=<type your search key> \
#   --name app rust-web-server-example:0.1.0
```
#### LICENSE
MIT