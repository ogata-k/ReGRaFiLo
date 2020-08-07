use crate::grafo::graph_item::edge::EdgeItemError;
use crate::grafo::graph_item::group::GroupItemError;
use crate::grafo::graph_item::node::NodeItemError;
use crate::util::name_type::NameType;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GrafoError<Name: NameType> {
    FailBuildGrafo,
    GroupItemError(GroupItemError<Name>),
    NodeItemError(NodeItemError<Name>),
    EdgeItemError(EdgeItemError<Name>),
}

impl<Name: NameType> std::fmt::Display for GrafoError<Name> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType> Error for GrafoError<Name> {}
