//! Module for error of graph witout layout

use crate::graph::edge::Edge;
use crate::util::Identity;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

/// Error of graph without layout
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GraphError<Id: Identity> {
    EdgeNotSupported(Id, Edge<Id>),
    IllegalEdge(Id),
    EdgeAlreadyExist(Id),
}

impl<Id: Identity> fmt::Display for GraphError<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphError::*;

        match self {
            EdgeNotSupported(edge_id, edge) => write!(
                f,
                "Not support undirected edge which is the edge {} at the id {:?}.",
                edge, edge_id
            ),
            IllegalEdge(edge_id) => {
                write!(f, "An edge has illegal parameter at the id {:?}", edge_id)
            }
            EdgeAlreadyExist(edge_id) => {
                write!(f, "An edge is already exist at the id {:?}", edge_id)
            }
        }
    }
}

impl<Id: Identity> Error for GraphError<Id> {}
