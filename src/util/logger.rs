//! ReGRaFiLo's logger module

use std::io::Write;
use std::process::exit;

use env_logger::Builder;
use log::{Level, LevelFilter};

/// ReGRaFiLo's logger
pub struct Logger {}

/// for Logger
#[allow(unused_macros)]
macro_rules! trace {
    ($($arg:tt)+) => (
        log::trace!($($arg)+);
        Logger::conclusion_for(Level::Trace);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:tt)+) => (
        log::debug!($($arg)+);
        Logger::conclusion_for(Level::Debug);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)+) => (
        log::info!($($arg)+);
        Logger::conclusion_for(Level::Info);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! warn {
    ($($arg:tt)+) => (
        log::warn!($($arg)+);
        Logger::conclusion_for(Level::Warn);
    )
}

/// for Logger
#[allow(unused_macros)]
macro_rules! error {
    ($($arg:tt)+) => (
        log::error!($($arg)+);
        Logger::conclusion_for(Level::Error);
    )
}

impl Logger {
    /// initialize for this Logger
    pub fn init(verbose: bool, code_trace: bool) {
        let mut builder = Builder::new();
        builder
            .format_timestamp_secs()
            .format_level(true)
            .format_module_path(true)
            .format_indent(Some(4));
        if cfg!(debug_assertions) || cfg!(test) {
            builder
                .format(|buf, record| {
                    let ts = buf.timestamp();
                    writeln!(buf,
                             "{}  [{}]\t{} at {}:{}:{}",
                             ts,
                             record.level(),
                             record.args(),
                             record.module_path().unwrap_or("<Unknown>"),
                             record.file().unwrap_or("<Unknown>"),
                             record.line().unwrap_or(0)
                    )
                });
            match (verbose, code_trace) {
                (true, true) => builder.filter_level(LevelFilter::Trace),
                (true, false) => builder.filter_level(LevelFilter::Debug),
                (false, _) => builder.filter_level(LevelFilter::Info),
            };
        } else {
            builder
                .format(|buf, record| {
                    let ts = buf.timestamp();
                    writeln!(buf,
                             "{}  [{}]\t{}",
                             ts,
                             record.level(),
                             record.args()
                    )
                });
            match (verbose, code_trace) {
                (true, _) => builder.filter_level(LevelFilter::Info),
                (false, _) => builder.filter_level(LevelFilter::Warn),
            };
        }

        builder.is_test(false);

        if let Err(e) = builder.try_init() {
            log::error!("fail init for ReGRaFiLo: {}", e);
        }
    }

    /// do action after show each message when use this Logger.
    /// usually use in each log macro.
    #[allow(dead_code)]
    fn conclusion_for(level: Level) {
        if level == Level::Error {
            exit(1);
        }
    }

    // TODO 各種表示用ラッパ
    // Self::XXXという関数でログを書かせるイメージ
}
