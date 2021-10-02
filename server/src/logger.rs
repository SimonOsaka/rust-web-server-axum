use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming, Record, WriteMode,
};

use std::thread;

fn logger_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} {} --- [{}-{:?}] {}:{} > {}",
        now.now().format("%Y-%m-%d %H:%M:%S%.6f %:z"),
        record.level(),
        thread::current().name().unwrap_or("<unnamed>"),
        thread::current().id(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
    )
}

pub fn create() {
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
}
