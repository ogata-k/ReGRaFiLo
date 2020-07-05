//! ReGRaFiLo's log module

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
        } else {
            if verbose {
                builder.filter_level(LevelFilter::Info);
            } else {
                builder.filter_level(LevelFilter::Warn);
            };
        }

        builder.is_test(true);

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

    //
    // 各種表示用ラッパ
    //

    /// log when create builder
    pub fn builder_start_log(kind: &str) {
        debug!("start {} builder", kind);
    }

    /// log when push item
    pub fn push_log(kind: &str, index: usize) {
        trace!("push {} item with id {}", kind, index);
    }

    /// log when push item with name
    pub fn with_name_push_log(kind: &str, name: &str, index: usize) {
        trace!(
            "push {} item with id {} with name \"{}\"",
            kind,
            index,
            name
        );
    }

    /// log when builder have done building action
    pub fn builder_finish_log(kind: &str) {
        debug!("build {} builder", kind);
    }
}
