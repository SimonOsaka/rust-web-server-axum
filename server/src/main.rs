use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode};

#[macro_use]
extern crate log;

mod logger;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenv::dotenv().ok();

    Logger::try_with_env()
        .unwrap()
        .log_to_file(FileSpec::default().directory("/apps/log/rust"))
        .write_mode(WriteMode::BufferAndFlush)
        .print_message()
        .duplicate_to_stdout(Duplicate::All)
        .duplicate_to_stderr(Duplicate::Warn)
        .format(crate::logger::logger_format)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7),
        )
        .append()
        .start()
        .unwrap();

    debug!("log file path: /apps/log/rust");

    api::start().await;
}
