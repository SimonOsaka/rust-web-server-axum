use tracing_appender::non_blocking::WorkerGuard;

#[cfg(feature = "lib_authorization")]
pub mod authorization;
#[cfg(feature = "lib_email")]
pub mod email;
pub mod logger;
#[cfg(feature = "lib_meilisearch")]
pub mod meilisearch;
#[cfg(feature = "lib_redis")]
pub mod redis;

pub async fn init() -> (WorkerGuard, WorkerGuard) {
    let (_guard_file, _guard_stderr) = logger::log_create();

    #[cfg(feature = "lib_email")]
    email::email_init().await;

    #[cfg(feature = "lib_redis")]
    redis::connection::RedisConnection::create().await;

    #[cfg(feature = "lib_meilisearch")]
    meilisearch::connection::MeiliSearch::create().await;

    #[cfg(feature = "lib_authorization")]
    authorization::init_authorization().await;

    (_guard_file, _guard_stderr)
}
