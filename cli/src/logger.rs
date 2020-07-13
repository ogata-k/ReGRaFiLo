//! ReGRaFiLo's log module
//! usual message for item's log is "item of <item kind> (with <option>)+ ..."

use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

/// ReGRaFiLo's logger
pub struct Logger {}

/// for Logger
#[allow(unused_macros)]
macro_rules! trace {
    ($($arg:tt)+) => (
        log::trace!($($arg)+);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)+) => (
        log::debug!($($arg)+);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)+) => (
        log::info!($($arg)+);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! warn {
    ($($arg:tt)+) => (
        log::warn!($($arg)+);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! error {
    ($($arg:tt)+) => (
        log::error!($($arg)+);
        panic!("occurred an unforeseen error");
    )
}

impl Logger {
    /// initializer with logger
    pub fn init(verbose: bool) {
        let mut builder = Builder::new();
        builder
            .format_timestamp_secs()
            .format_level(true)
            .format_module_path(true)
            .format_indent(Some(4))
            .format(|buf, record| {
                let ts = buf.timestamp();
                writeln!(buf, "{}  [{}]\t{}", ts, record.level(), record.args())
            });

        if cfg!(debug_assertions) || cfg!(test) {
            if verbose {
                builder.filter_level(LevelFilter::Trace);
            } else {
                builder.filter_level(LevelFilter::Debug);
            };
        } else if verbose {
            builder.filter_level(LevelFilter::Info);
        } else {
            builder.filter_level(LevelFilter::Warn);
        }

        builder.is_test(true);

        if let Err(e) = builder.try_init() {
            log::error!("fail init for ReGRaFiLo: {}", e);
        }
    }
}
