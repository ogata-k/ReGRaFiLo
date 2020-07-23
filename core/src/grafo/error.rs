use crate::grafo::core::graph_item::edge::EdgeItemError;
use crate::grafo::core::graph_item::group::GroupItemError;
use crate::grafo::core::graph_item::node::NodeItemError;
use crate::grafo::core::resolve::NameIdError;
use crate::util::kind::{GraphItemKind, LayoutItemKind};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GrafoError {
    GroupItemError(GroupItemError),
    NodeItemError(NodeItemError),
    EdgeItemError(EdgeItemError),
    ItemNameRefError(NameIdError<GraphItemKind>),
    LayoutNameRefError(NameIdError<LayoutItemKind>),
}

impl std::fmt::Display for GrafoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for GrafoError {}
