//! event and the visitor for the event

use std::error::Error;

use crate::util::alias::ItemIndex;
use crate::util::item_kind::ItemKind;
use std::fmt::{Debug, Display};

/// Visitor pattern for event
pub trait Visitor {
    fn visit(&mut self, event: &Event);
}

//
// Item
//
/// Display and Debug trait
pub trait DisplayWithDebug: Display + Debug {}
impl<T: Display + Debug> DisplayWithDebug for T {}

/// event information when do action<br/>
/// past verb is event after action. another is event before action.
#[derive(Debug)]
pub enum Event<'a> {
    InitializeStore(ItemKind),
    InitializeAttribute,
    /// Kind, GroupId, ItemId
    SucceededPushItem(ItemKind, ItemIndex, ItemIndex),
    /// Kind, GroupId, Err
    FailPushItem(ItemKind, ItemIndex, &'a dyn Error),
    PushValue(ItemKind, ItemIndex, &'a dyn DisplayWithDebug),
    OverrideValue(ItemKind, ItemIndex, &'a dyn DisplayWithDebug),
}

#[cfg(test)]
pub mod test {
    use crate::event::Event;

    pub const ITERATE_COUNT: usize = 10;

    pub struct Visitor {}

    impl crate::event::Visitor for Visitor {
        fn visit(&mut self, _event: &Event<'_>) {}
    }

    impl Visitor {
        pub fn new() -> Self {
            Default::default()
        }
    }

    impl Default for Visitor {
        fn default() -> Self {
            Self {}
        }
    }
}
