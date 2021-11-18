use once_cell::sync::Lazy;
use time::{format_description, OffsetDateTime, UtcOffset};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{subscribe::CollectExt, util::SubscriberInitExt, EnvFilter};

const TS_S: &str = "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6] \
                    [offset_hour sign:mandatory]:[offset_minute]";

static TS: Lazy<Vec<format_description::FormatItem<'static>>> =
    Lazy::new(|| format_description::parse(TS_S).unwrap());

// because time crate issue, localtime doesn't run, so make a custom timer to resolve it.
#[derive(Debug, Clone)]
struct UtcWithTz {
    hours: i8,
    minutes: i8,
    seconds: i8,
}

impl tracing_subscriber::fmt::time::FormatTime for UtcWithTz {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            OffsetDateTime::now_utc()
                .to_offset(UtcOffset::from_hms(self.hours, self.minutes, self.seconds).unwrap())
                .format(&TS)
                .expect("failed offset date")
        )
    }
}
pub fn log_create() -> (WorkerGuard, WorkerGuard) {
    let offset_tz = UtcWithTz {
        hours: 8,
        minutes: 0,
        seconds: 0,
    };

    let (stderr_writer, _guard_stderr) = tracing_appender::non_blocking(std::io::stderr());

    let fmt_stdout = tracing_subscriber::fmt::Subscriber::new()
        .with_timer(offset_tz.clone())
        .with_ansi(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_level(true)
        .with_target(true)
        .with_writer(stderr_writer);

    // 1. Local time required. log file append UTC time.
    // 2. no max file num
    // 3. file size
    let file_appender = tracing_appender::rolling::hourly("/apps/log/rust", "example.log");
    let (file_appender_writer, _guard_file) = tracing_appender::non_blocking(file_appender);

    let fmt_file = tracing_subscriber::fmt::Subscriber::new()
        .json()
        .with_timer(offset_tz)
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_level(true)
        .with_target(true)
        .with_writer(file_appender_writer);

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    // let subscriber =
    tracing_subscriber::registry()
        .with(env_filter)
        // .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt_stdout)
        // .with(tracing_subscriber::fmt::Subscriber::new().with_writer(std::io::stdout))
        .with(fmt_file)
        // .with(tracing_subscriber::fmt::Subscriber::new().with_writer(non_blocking))
        .init();

    // tracing::collect::set_global_default(subscriber).expect("Unable to set a global collector");

    // tracing_subscriber::fmt::init();

    tracing::debug!("debug is open");
    tracing::info!("info is open");
    tracing::warn!("warn is open");
    tracing::trace!("trace is open");
    tracing::error!("error is open");

    (_guard_file, _guard_stderr)
}
