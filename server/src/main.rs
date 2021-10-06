#[macro_use]
extern crate log;

mod logger;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv::dotenv().ok();

    logger::create();

    repository::db::Repo::create().await;

    redis::connection::RedisConnection::create().await;

    search::connection::MeiliSearch::create().await;

    api::start().await;
}
