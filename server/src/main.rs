#[macro_use]
extern crate log;

mod logger;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv::dotenv().ok();

    logger::create();

    if cfg!(feature = "database_lib") {
        repository::db::Repo::create().await;
    }

    if cfg!(feature = "redis_lib") {
        redis::connection::RedisConnection::create().await;
    }

    if cfg!(feature = "search_lib") {
        search::meilisearch::MeiliSearch::create().await;
    }

    if cfg!(feature = "api_warp_lib") {
        warp_api::start().await;
    } else {
        axum_api::start().await;
    }
}
