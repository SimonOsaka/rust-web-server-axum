use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, Logger, Naming, Record, WriteMode,
};
use once_cell::sync::Lazy;
use time::{format_description, UtcOffset};

use std::thread;

const TS_S: &str = "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6] \
                    [offset_hour sign:mandatory]:[offset_minute]";

static TS: Lazy<Vec<format_description::FormatItem<'static>>> =
    Lazy::new(|| format_description::parse(TS_S).unwrap());

fn logger_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} {} --- [{}-{:?}] {}:{} > {}",
        now.now()
            .to_offset(UtcOffset::from_hms(8, 0, 0).unwrap())
            .format(&TS)
            .unwrap_or_else(|_| "Timestamping failed".to_string()),
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
