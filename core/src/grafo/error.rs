use crate::grafo::graph_item::edge::EdgeItemError;
use crate::grafo::graph_item::group::GroupItemError;
use crate::grafo::graph_item::node::NodeItemError;
use crate::util::name_type::{NameType, StoredNameType};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GrafoError<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    FailBuildGrafo,
    GroupItemError(GroupItemError<Name, StoredName>),
    NodeItemError(NodeItemError<Name, StoredName>),
    EdgeItemError(EdgeItemError<Name, StoredName>),
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> std::fmt::Display
    for GrafoError<Name, StoredName>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Error
    for GrafoError<Name, StoredName>
{
}
