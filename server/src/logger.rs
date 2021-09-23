use flexi_logger::{DeferredNow, Record};
use std::thread;

pub fn logger_format(
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
