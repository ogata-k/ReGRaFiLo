//! Module for error of graph witout layout

use crate::graph::edge::Edge;
use crate::util::Identity;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

/// alias for Edge structure
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ErrorEdge<Id: Identity> {
    edge: Edge<Id>,
}

impl<Id: Identity> fmt::Display for ErrorEdge<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.edge, f)
    }
}

impl<Id: Identity> From<Edge<Id>> for ErrorEdge<Id> {
    fn from(edge: Edge<Id>) -> Self {
        Self::create(edge)
    }
}

impl<Id: Identity> ErrorEdge<Id> {
    // ---
    // constructor
    // ---

    /// constructor
    fn create(edge: Edge<Id>) -> Self {
        ErrorEdge { edge: edge }
    }

    // ---
    // getter
    // ---

    /// get weight.
    /// If weight is 1 or no weight, the edge's weight is 1.
    pub fn get_weight(&self) -> &i16 {
        self.edge.get_weight()
    }

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---

    /// check edge is undirected edge
    pub fn is_undirected(&self) -> bool {
        self.edge.is_undirected()
    }

    /// check edge is directed edge
    pub fn is_directed(&self) -> bool {
        self.edge.is_directed()
    }

    /// check edge is undirected or directed edge
    pub fn is_edge(&self) -> bool {
        self.edge.is_edge()
    }

    /// check edge is undirected hyper edge
    pub fn is_undirected_hyper(&self) -> bool {
        self.edge.is_undirected_hyper()
    }

    /// check edge is directed hyper edge
    pub fn is_directed_hyper(&self) -> bool {
        self.edge.is_directed_hyper()
    }

    /// check edge is undirected or directed hyper edge
    pub fn is_hyper_edge(&self) -> bool {
        self.edge.is_hyper_edge()
    }

    // ---
    // delete
    // ---
}

/// Error of graph without layout
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GraphError<Id: Identity> {
    EdgeNotSupported(Id, ErrorEdge<Id>),
    IllegalEdge(Id, ErrorEdge<Id>),
    ExistSameEdge(Id, ErrorEdge<Id>),
    NotSameNodeGroupHaveIntersect(Id, ErrorEdge<Id>),
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
            IllegalEdge(edge_id, edge) => {
                write!(f, "An edge {} has illegal parameter at the id {:?}.",edge,  edge_id)
            }
            ExistSameEdge(edge_id, edge) =>write!(
                f,
                "Cannot insert the edge {} at the id {:?} because of exist same edge.",
                edge, edge_id
            ),
            NotSameNodeGroupHaveIntersect(edge_id, edge) => write!(
                f,
                "Already node group has intersection to the edge {} at the id {:?} as node grouping.",
                edge, edge_id
            ),
        }
    }
}

impl<Id: Identity> Error for GraphError<Id> {}
