#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv::dotenv().ok();

    // add return value here, log can output to console and file, don't change.
    // issue: https://github.com/tokio-rs/tracing/issues/971
    let (_guard_file, _guard_stderr) = logger::log_create();

    #[cfg(any(feature="tokio_console"))]
    console_subscriber::init();

    #[cfg(any(feature = "auth_lib"))]
    auth::init().await;

    #[cfg(any(feature = "extra_lib"))]
    extra::init().await;

    #[cfg(any(feature = "database_lib"))]
    repository::db::Repo::create().await;

    #[cfg(any(feature = "redis_lib"))]
    redis::connection::RedisConnection::create().await;

    #[cfg(any(feature = "search_lib"))]
    search::meilisearch::MeiliSearch::create().await;

    #[cfg(any(feature = "api_warp_lib"))]
    warp_api::start().await;

    #[cfg(any(feature = "api_axum_lib"))]
    axum_api::start().await;
}
