//! ReGRaFiLo's log module
//! usual message for item's log is "item of <item kind> (with <option>)+ ..."

use std::fmt::Debug;
use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

/// ReGRaFiLo's logger
pub struct Logger {}

/// Base of Kind
pub trait KindBase: Ord + Eq + Copy + GroupKind4Logger + KeyKind4Logger {}

/// get the kind name of the type for Logger
pub trait GroupKind4Logger {
    fn group_kind_string() -> &'static str;
}

/// get the kind name of the instance for Logger
pub trait KeyKind4Logger {
    fn key_kind_string(&self) -> &'static str;
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

    //
    // 各種表示用ラッパ
    //

    /// log when create builder
    pub fn initializer_log(store_group_kind: &str, item_group_kind: Option<&str>) {
        if let Some(item_kind_str) = item_group_kind {
            debug!(
                "initialize {} store for {} item",
                store_group_kind, item_kind_str
            );
        } else {
            debug!("initialize {} store", store_group_kind);
        }
    }

    /// log when push item
    pub fn push_log(store_group_kind: &str, item_key_kind: &str, index: usize) {
        trace!(
            "push {} item into {} store with the id {}",
            item_key_kind,
            store_group_kind,
            index
        );
    }

    /// log when push item with name
    pub fn with_name_push_log(
        store_group_kind: &str,
        item_key_kind: &str,
        name: &str,
        index: usize,
    ) {
        trace!(
            "push {} item into {} store with the id {} with the name \"{}\"",
            item_key_kind,
            store_group_kind,
            index,
            name
        );
    }

    /// log when push item override
    pub fn override_value_log<S: ToString>(store_group_kind: &str, item_key_kind: &str, value: S) {
        warn!(
            "{}'s item in {} store override a value to {}",
            item_key_kind,
            store_group_kind,
            value.to_string()
        );
    }

    /// log when occurred inconsistent action
    pub fn inconsistent<D: Debug>(store_group_kind: &str, item_key_kind: &str, value: D) {
        error!(
            "{}'s item of {} store is inconsistent: {:?}",
            item_key_kind, store_group_kind, value
        );
    }
}
