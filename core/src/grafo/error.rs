use crate::grafo::core::item::edge::EdgeItemError;
use crate::grafo::core::item::group::GroupItemError;
use crate::grafo::core::item::node::NodeItemError;
use crate::grafo::core::refindex::NameRefWarning;
use crate::util::item_kind::ItemKind;
use crate::util::layout_kind::LayoutKind;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum GrafoError {
    GroupItemError(GroupItemError),
    NodeItemError(NodeItemError),
    EdgeItemError(EdgeItemError),
    ItemNameRefWarning(NameRefWarning<ItemKind>),
    AttributeNameRefWarning(NameRefWarning<LayoutKind>),
}

impl std::fmt::Display for GrafoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for GrafoError {}
