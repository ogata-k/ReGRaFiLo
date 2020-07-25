use crate::grafo::graph_item::edge::EdgeItemError;
use crate::grafo::graph_item::group::GroupItemError;
use crate::grafo::graph_item::node::NodeItemError;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GrafoError {
    GroupItemError(GroupItemError),
    NodeItemError(NodeItemError),
    EdgeItemError(EdgeItemError),
}

impl std::fmt::Display for GrafoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for GrafoError {}
