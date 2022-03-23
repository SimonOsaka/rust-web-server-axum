# rust-web-server-example
  learn rust web server and crates.

## Structure
- api: web request & response
    - use axum

- crates: third part crates, some crates are not latest, so clone and include it
    - lettre
    - tracing

- domain: business service & impl

- extra:
    - authorization: rbac auth
        - use casbin
    - email: send email
        - use lettre
    - logger: log for everything
        - use tracing
    - meilisearch: search engine
        - use meilisearch
    - redis: memory and disk store
        - use redis

- repository: wrap database operation & sql
    - use postgresql
    - use sqlx

- search: business service for search

- server_app: main entry

- server_lib: boot

- util:
    - date: date format and parse
        - use time
    - excel: export data to excel
        - use xlsxwriter
    - i18n: multi-languages
    - jwt: resource auth
        - use jsonwebtoken

- vars: variables for all module

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
-- add column constraint
ALTER TABLE table_name ADD CONSTRAINT constraint_name UNIQUE(column_name);
-- drop column constraint
ALTER TABLE table_name DROP CONSTRAINT constraint_name;
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

## tracing
1. init to see source [logger/src/tracing_logger.rs](logger/src/tracing_logger.rs)
    - environment variable `RUST_ENV`
    - stderr, console output log in `development`
    - file, path to log in `production`, `development`
2. tracing log marco
    - `tracing::debug!`
    - `tracing::info!`
    - `tracing::warn!`
    - `tracing::error!`
3. tracing method 
    - `#[tracing::instrument]` in library or async method
    - `#[tracing::instrument(skip(transaction))]` ignore `transaction` param in method
    - `#[tracing::instrument(skip(transaction), fields(name=%u.username))]` output `name=xxxx`
    - `#[tracing::instrument(skip(transaction), fields(name=%u.username), err)]` err print

## tokio-console
Web: [github/tokio-console](https://github.com/tokio-rs/console)
1. add dependency
```toml
console-subscriber = "0.1"
# features['tracing'] required
tokio = { version = "1", features = ["tracing"] }
```
2. add init code to main function
```rust
#[tokio::main]
async fn main() {
    console_subscriber::init();
}
```
3. cargo run command with tokio-console
```shell
RUSTFLAGS="--cfg tokio_unstable" cargo run
```
4. install tokio-console
```shell
cargo install tokio-console
```
5. run tokio-console
```shell
tokio-console
# 'command/ctrl + click' 'console_subscriber::init();' to see ENV variables.
```
*For further to see docs*.

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