pub async fn start() {
    dotenv::dotenv().ok();

    // add return value here, log can output to console and file, don't change.
    // issue: https://github.com/tokio-rs/tracing/issues/971
    let (_guard_file, _guard_stderr) = extra::init().await;

    #[cfg(feature = "tokio_console")]
    console_subscriber::init();

    #[cfg(feature = "database_lib")]
    repository::db::Repo::create().await;

    #[cfg(feature = "api_lib")]
    api::start().await;
}
