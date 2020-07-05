//! ReGRaFiLo's log module
//! usual message for item's log is "item of <item kind> (with <option>)+ ..."

use std::fmt::Debug;
use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

/// ReGRaFiLo's logger
pub struct Logger {}

/// get kind name of the type for Logger
pub trait KindGroup4Logger {
    fn kind_group() -> &'static str;
}

/// get kind name of the instance for Logger
pub trait KindKey4Logger {
    fn get_kind_string(&self) -> String;
}

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

    //
    // 各種表示用ラッパ
    //

    /// log when create builder
    pub fn builder_start_log(kind: &str) {
        debug!("start {} builder", kind);
    }

    /// log when push item
    pub fn push_log(kind: &str, index: usize) {
        trace!("push item of {} with the id {}", kind, index);
    }

    /// log when push item with name
    pub fn with_name_push_log(kind: &str, name: &str, index: usize) {
        trace!(
            "push item of {} with the id {} with the name \"{}\"",
            kind,
            index,
            name
        );
    }

    /// log when builder have done building action
    pub fn builder_finish_log(kind: &str) {
        debug!("build {} builder", kind);
    }

    /// log when push item override
    pub fn override_log<S: ToString>(kind: &str, item: S) {
        warn!("item of {} override from {}", kind, item.to_string());
    }

    pub fn inconsistent<D: Debug>(kind: &str, value: D) {
        error!("item of {} is inconsistent: {:?}", kind, value);
    }
}
