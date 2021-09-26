//! Module for error of graph without layout

use crate::graph::edge::{model, Edge};
use crate::util::Identity;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use crate::graph::model::EdgeModel;

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
        ErrorEdge { edge }
    }

    // ---
    // getter
    // ---

    /// get weight.
    /// If weight is 1 or no weight, the edge's weight is 1.
    pub fn get_weight(&self) -> i16 {
        self.edge.as_model().get_weight()
    }

    /// create model as edge
    pub fn as_model<'a>(&'a self) -> model::Edge<'a, Id> {
        self.edge.as_model()
    }

    /// create model as undirected edge
    pub fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, Id>> {
        self.edge.as_undirected_model()
    }

    /// create model as directed edge
    pub fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, Id>> {
        self.edge.as_directed_model()
    }

    /// create model as mixed edge
    pub fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, Id>> {
        self.edge.as_mixed_model()
    }

    /// create model as undirected hyper edge
    pub fn as_undirected_hyper_model<'a>(&'a self) -> Option<model::UndirectedHyperEdge<'a, Id>> {
        self.edge.as_undirected_hyper_model()
    }

    /// create model as mixed hyper edge
    pub fn as_directed_hyper_model<'a>(&'a self) -> Option<model::DirectedHyperEdge<'a, Id>> {
        self.edge.as_directed_hyper_model()
    }

    /// create model as mixed hyper edge
    pub fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, Id>> {
        self.edge.as_mixed_hyper_model()
    }

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---
}

/// Error of graph without layout
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GraphError<Id: Identity> {
    // Node
    /// already node exist at the id.
    ///
    /// arg: node_id
    AlreadyExistNodeAtId(Id),
    /// node not exist at the specified id.
    ///
    /// arg: node_id
    NotExistNodeAtId(Id),
    /// not support group error.
    ///
    /// arg: group_node_id
    NotSupportGroupNode(Id),
    /// cannot create vertex node.
    ///
    /// arg: (optional)parent_node_id, vertex_node_id
    CannotCreateVertex(Option<Id>, Id),
    /// cannot create group node.
    ///
    /// arg: (optional)parent_node_id, group_node_id, child_node_ids
    CannotCreateGroup(Option<Id>, Id, Vec<Id>),
    /// not exist group node at the id.
    ///
    /// arg: group_node_id
    NotExistGroup(Id),
    /// specified children as children for the group has illegal status.
    ///
    /// arg: group_node_id, child_node_ids
    SpecifiedIllegalChildren(Id, Vec<Id>),
    /// specified children as children for the group is not exist when not use the mode to create not exist vertex node.
    ///
    /// arg: group_node_id, child_node_ids
    NotExistChildrenCannotMakeEdge(Id, Vec<Id>),

    // Edge
    /// already edge exist at the id.
    ///
    /// arg: edge_id
    AlreadyExistEdgeAtId(Id),
    /// edge not exist at the specified id.
    ///
    /// arg: edge_id
    NotExistEdgeAtId(Id),
    /// not support edge error
    ///
    /// arg: edge_id, edge
    EdgeNotSupported(Id, ErrorEdge<Id>),
    /// not available edge error
    ///
    /// arg: edge_id, edge
    IllegalEdge(Id, ErrorEdge<Id>),
    /// exist same edge error
    ///
    /// arg: edge_id, edge, same_edge_ids
    ExistSameEdge(Id, ErrorEdge<Id>, Vec<Id>),
    /// specified node ids as incidence node for the edge has illegal status.
    ///
    /// arg: edge_id, node_ids
    SpecifiedIllegalIncidenceNodeIds(Id, ErrorEdge<Id>, Vec<Id>),
}

impl<Id: Identity> fmt::Display for GraphError<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphError::*;

        match self {
            // Node
            AlreadyExistNodeAtId(node_id) => {
                write!(f, "Already node is exist at the id {:?}.", node_id)
            }
            NotExistNodeAtId(node_id) => {
            write!(f, "Specified target node is not exist at the id {:?}.", node_id)
        }
            NotSupportGroupNode(group_node_id) => {
                write!(f, "Not support group node at the id {:?}.", group_node_id)
            }
            CannotCreateVertex(Some(parent_node_id), vertex_node_id) => write!(
                f,
                "Cannot create vertex node at the id {:?} in the parent {:?}.",
                vertex_node_id, parent_node_id
            ),
            CannotCreateVertex(None, vertex_node_id) => write!(
                f,
                "Cannot create vertex node at the id {:?}.",
                vertex_node_id
            ),
            CannotCreateGroup(Some(parent_node_id), group_node_id, child_node_ids) => {
                write!(
                    f,
                    "Cannot create group node at the id {:?} with child {{",
                    group_node_id
                )?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}} in the parent {:?}.", parent_node_id)
            }
            CannotCreateGroup(None, group_node_id, child_node_ids) => {
                write!(
                    f,
                    "Cannot create group node at the id {:?} with child {{",
                    group_node_id
                )?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}}.")
            }
            NotExistGroup(group_node_id) => write!(
                f,
                "Group node is not exist or vertex node exist at the id {:?}.",
                group_node_id
            ),
            SpecifiedIllegalChildren(group_node_id, child_node_ids) => {
                write!(f, "Specified children {{")?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(
                    f,
                    "}} have illegal children for the group {:?}.",
                    group_node_id
                )
            }
            NotExistChildrenCannotMakeEdge(group_node_id, child_node_ids) => {
                write!(f, "Specified children {{")?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}} are not exist for the group {:?}.", group_node_id)
            }

            // Edge
            AlreadyExistEdgeAtId(edge_id) => {
                write!(f, "Already edge is exist at the id {:?}.", edge_id)
            }
            NotExistEdgeAtId(edge_id) => {
                write!(f, "Specified target edge is not exist at the id {:?}.", edge_id)
            }
            EdgeNotSupported(edge_id, edge) => write!(
                f,
                "Not support edge which is the edge {} at the id {:?}.",
                edge.as_model(),
                edge_id
            ),
            IllegalEdge(edge_id, edge) => {
                write!(
                    f,
                    "An edge {} has illegal parameter at the id {:?}.",
                    edge.as_model(),
                    edge_id
                )
            }
            ExistSameEdge(edge_id, edge, same_edge_ids) => {
                write!(
                    f,
                   "Cannot insert the edge {} at the id {:?} because exist same edges at the ids {{",
                   edge.as_model(),
                   edge_id
                )?;
                for (index, same_edge_id) in same_edge_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", same_edge_id)?;
                    } else {
                        write!(f, ", {:?}", same_edge_id)?;
                    }
                }
                write!(f, "}}.")
            }
            SpecifiedIllegalIncidenceNodeIds(edge_id, edge, incidence_node_ids) => {
                write!(
                    f,
                    "Cannot create the edge {} at the id {:?} because fail resolve node ids {{",
                    edge.as_model(),
                    edge_id
                )?;
                for (index, node_id) in incidence_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", node_id)?;
                    } else {
                        write!(f, ", {:?}", node_id)?;
                    }
                }
                write!(f, "}}.")
            }
        }
    }
}

impl<Id: Identity> Error for GraphError<Id> {}
