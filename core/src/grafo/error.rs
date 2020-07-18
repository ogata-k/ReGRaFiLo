use crate::grafo::core::graph_item::edge::EdgeItemError;
use crate::grafo::core::graph_item::group::GroupItemError;
use crate::grafo::core::graph_item::node::NodeItemError;
use crate::grafo::core::name_refindex::NameRefWarning;
use crate::util::kind::{GraphItemKind, LayoutItemKind};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum GrafoError {
    GroupItemError(GroupItemError),
    NodeItemError(NodeItemError),
    EdgeItemError(EdgeItemError),
    ItemNameRefWarning(NameRefWarning<GraphItemKind>),
    AttributeNameRefWarning(NameRefWarning<LayoutItemKind>),
}

impl std::fmt::Display for GrafoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for GrafoError {}
