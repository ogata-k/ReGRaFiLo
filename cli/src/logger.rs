//! ReGRaFiLo's log module
//! usual message for item's log is "item of <item kind> (with <option>)+ ..."

use std::io::Write;

use env_logger::Builder;
use log::LevelFilter;

use regrafilo_core::event::{Event, ItemEventKind, Visitor};

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

fn item_kind_to_str(item_kind: &ItemEventKind) -> &str {
    match item_kind {
        ItemEventKind::Group => "Group",
        ItemEventKind::Node => "Node",
        ItemEventKind::Edge => "Edge",
    }
}

impl Visitor for Logger {
    fn visit(&mut self, event: &Event<'_>) {
        match event {
            Event::InitializeStore(item_kind) => {
                debug!("initialize {} item store", item_kind_to_str(item_kind));
            }
            Event::SucceededPushItem(item_kind, group_id, item_id) => {
                trace!(
                    "push {} item with the id {} at group {}",
                    item_kind_to_str(item_kind),
                    item_id,
                    group_id,
                );
            }
            Event::FailPushItem(item_kind, group_id, err) => {
                warn!(
                    "fail push {} item at group {} with error: {}",
                    item_kind_to_str(item_kind),
                    group_id,
                    err,
                );
            }
            Event::InitializeAttribute => {
                debug!("initialize attribute reference indexes");
            }
            Event::PushValue(item_kind, item_id, value) => {
                trace!(
                    "push {} item with the id {} for the value {}",
                    item_kind_to_str(item_kind),
                    item_id,
                    value,
                );
            }
            Event::OverrideValue(item_kind, item_id, value) => {
                warn!(
                    "override {} item with the id {} for the value {}",
                    item_kind_to_str(item_kind),
                    item_id,
                    value
                );
            }
        }
    }
}

impl Logger {
    /// initializer with logger
    pub fn new(verbose: bool) -> Self {
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

        Self {}
    }
}
