use crate::grafo::core::item::edge::EdgeItemError;
use crate::grafo::core::item::group::GroupItemError;
use crate::grafo::core::item::node::NodeItemError;
use std::error::Error;
use std::fmt::Formatter;
use crate::grafo::core::layout::error::AttributeWarning;

#[derive(Debug, Clone)]
pub enum GrafoError {
    GroupItemError(GroupItemError),
    NodeItemError(NodeItemError),
    EdgeItemError(EdgeItemError),
    AttributeWarning(AttributeWarning),
}

impl std::fmt::Display for GrafoError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for GrafoError {}
