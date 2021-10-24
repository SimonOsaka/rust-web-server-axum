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