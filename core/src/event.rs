//! event and the visitor for the event

use std::error::Error;

use crate::util::alias::ItemIndex;
use std::fmt::{Debug, Display};

/// Visitor pattern for event
pub trait Visitor {
    fn visit(&mut self, event: &Event);
}

//
// Item
//
/// event kind
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ItemEventKind {
    Group,
    Node,
    Edge,
}

pub trait DisplayWithDebug: Display + Debug {}
impl<T: Display + Debug> DisplayWithDebug for T {}

/// event information when do action<br/>
/// past verb is event after action. another is event before action.
#[derive(Debug)]
pub enum Event<'a> {
    InitializeStore(ItemEventKind),
    InitializeAttribute,
    /// Kind, GroupId, ItemId
    SucceededPushItem(ItemEventKind, ItemIndex, ItemIndex),
    /// Kind, GroupId, Err
    FailPushItem(ItemEventKind, ItemIndex, &'a dyn Error),
    PushValue(ItemEventKind, ItemIndex, &'a dyn DisplayWithDebug),
    OverrideValue(ItemEventKind, ItemIndex, &'a dyn DisplayWithDebug),
}

#[cfg(test)]
pub mod test {
    use crate::event::{Event, ItemEventKind};
    use crate::util::util_trait::KindBase;

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

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    pub enum Kind {
        Group,
        Node,
        Edge,
    }

    impl KindBase for Kind {}

    impl Into<ItemEventKind> for Kind {
        fn into(self) -> ItemEventKind {
            match self {
                Self::Group => ItemEventKind::Group,
                Self::Node => ItemEventKind::Node,
                Self::Edge => ItemEventKind::Edge,
            }
        }
    }

    pub fn check_list() -> Vec<Kind> {
        use Kind::*;
        vec![Group, Node, Edge]
    }
}
