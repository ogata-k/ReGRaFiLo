use std::error::Error;
use std::fmt::Formatter;

use crate::grafo::graph_item::edge::EdgeItemError;
use crate::grafo::graph_item::group::GroupItemError;
use crate::grafo::graph_item::node::NodeItemError;
use crate::grafo::ResolverError;
use crate::util::name_type::NameType;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GrafoError<Name: NameType> {
    FailBuildGrafo,
    ResolverError(ResolverError),
    GroupItemError(GroupItemError<Name>),
    NodeItemError(NodeItemError<Name>),
    EdgeItemError(EdgeItemError<Name>),
}

impl<Name: NameType> std::fmt::Display for GrafoError<Name> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GrafoError::FailBuildGrafo => write!(f, "fail build Grafo"),
            GrafoError::ResolverError(e) => write!(f, "{}", e),
            GrafoError::GroupItemError(e) => write!(f, "{}", e),
            GrafoError::NodeItemError(e) => write!(f, "{}", e),
            GrafoError::EdgeItemError(e) => write!(f, "{}", e),
        }
    }
}

impl<Name: NameType> Error for GrafoError<Name> {}

impl<Name: NameType> From<ResolverError> for GrafoError<Name> {
    fn from(e: ResolverError) -> Self {
        Self::ResolverError(e)
    }
}

impl<Name: NameType> From<GroupItemError<Name>> for GrafoError<Name> {
    fn from(e: GroupItemError<Name>) -> Self {
        Self::GroupItemError(e)
    }
}

impl<Name: NameType> From<NodeItemError<Name>> for GrafoError<Name> {
    fn from(e: NodeItemError<Name>) -> Self {
        Self::NodeItemError(e)
    }
}

impl<Name: NameType> From<EdgeItemError<Name>> for GrafoError<Name> {
    fn from(e: EdgeItemError<Name>) -> Self {
        Self::EdgeItemError(e)
    }
}
