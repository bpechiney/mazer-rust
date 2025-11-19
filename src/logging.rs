use std::fmt::{self};

use clap::ValueEnum;
use time::OffsetDateTime;
use time::macros::format_description;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum LogLevel {
    /// The most verbose logging, including very fine-grained details.
    Trace,
    /// Verbose debugging information, which is useful during development.
    Debug,
    /// Default application-level informational messages.
    Info,
    /// Warnings about unexpected situations that are not fatal.
    Warn,
    /// Errors indicating failures or conditions that should be investigated.
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };

        write!(f, "{}", s)
    }
}

struct TimeWithMillis;

impl FormatTime for TimeWithMillis {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> fmt::Result {
        let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

        // HH:MM:SS.uuuuuu (6 digits subsecond → microseconds)
        let format = format_description!("[hour]:[minute]:[second].[subsecond digits:6]");

        write!(w, "{}", now.format(&format).unwrap())
    }
}

pub fn init_logging(level: LogLevel) {
    let level_str = level.to_string().to_lowercase();
    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level_str));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_timer(TimeWithMillis);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
