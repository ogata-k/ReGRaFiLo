//! Module for error of graph without layout

use crate::graph::as_model::AsEdgeModel;
use crate::graph::model;
use crate::graph::model::EdgeModel;
use crate::graph::store::Edge;
use crate::util::{Identity, Weight};
use std::error::Error;
use std::fmt;
use std::fmt::Debug;

/// alias for Edge structure
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ErrorEdge<NodeId: Identity, EdgeId: Identity> {
    edge: Edge<NodeId, EdgeId>,
}

impl<NodeId: Identity, EdgeId: Identity> fmt::Display for ErrorEdge<NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.edge, f)
    }
}

impl<NodeId: Identity, EdgeId: Identity> From<Edge<NodeId, EdgeId>> for ErrorEdge<NodeId, EdgeId> {
    fn from(edge: Edge<NodeId, EdgeId>) -> Self {
        Self::create(edge)
    }
}

impl<NodeId: Identity, EdgeId: Identity> ErrorEdge<NodeId, EdgeId> {
    // ---
    // constructor
    // ---

    /// constructor
    fn create(edge: Edge<NodeId, EdgeId>) -> Self {
        ErrorEdge { edge }
    }

    // ---
    // getter
    // ---

    /// get weight.
    /// If weight is 1 or no weight, the edge's weight is 1.
    pub fn get_weight(&self) -> Weight {
        self.edge.as_model().get_weight()
    }

    /// create model as edge
    pub fn as_model<'a>(&'a self) -> model::Edge<'a, NodeId, EdgeId> {
        self.edge.as_model()
    }

    /// create model as undirected edge
    pub fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, NodeId, EdgeId>> {
        self.edge.as_undirected_model()
    }

    /// create model as directed edge
    pub fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, NodeId, EdgeId>> {
        self.edge.as_directed_model()
    }

    /// create model as mixed edge
    pub fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, NodeId, EdgeId>> {
        self.edge.as_mixed_model()
    }

    /// create model as undirected hyper edge
    pub fn as_undirected_hyper_model<'a>(
        &'a self,
    ) -> Option<model::UndirectedHyperEdge<'a, NodeId, EdgeId>> {
        self.edge.as_undirected_hyper_model()
    }

    /// create model as mixed hyper edge
    pub fn as_directed_hyper_model<'a>(
        &'a self,
    ) -> Option<model::DirectedHyperEdge<'a, NodeId, EdgeId>> {
        self.edge.as_directed_hyper_model()
    }

    /// create model as mixed hyper edge
    pub fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, NodeId, EdgeId>> {
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
pub enum GraphError<NodeId: Identity, EdgeId: Identity> {
    // Node
    /// already node exist at the id.
    ///
    /// arg: node_id
    AlreadyExistNodeAtId(NodeId),
    /// node not exist at the specified id.
    ///
    /// arg: node_id
    NotExistNodeAtId(NodeId),
    /// not support group error.
    ///
    /// arg: group_node_id
    NotSupportGroupNode(NodeId),
    /// cannot create vertex node.
    ///
    /// arg: (optional)parent_node_id, vertex_node_id
    CannotCreateVertex(Option<NodeId>, NodeId),
    /// cannot create group node.
    ///
    /// arg: (optional)parent_node_id, group_node_id, child_node_ids
    CannotCreateGroup(Option<NodeId>, NodeId, Vec<NodeId>),
    /// not exist group node at the id.
    ///
    /// arg: group_node_id
    NotExistGroup(NodeId),
    /// specified children as children for the group has illegal status.
    ///
    /// arg: group_node_id, child_node_ids
    SpecifiedIllegalChildren(NodeId, Vec<NodeId>),
    /// specified children as children for the group is not exist when not use the mode to create not exist vertex node.
    ///
    /// arg: group_node_id, child_node_ids
    NotExistChildrenCannotMakeAsGroupChild(NodeId, Vec<NodeId>),

    // Edge
    /// already edge exist at the id.
    ///
    /// arg: edge_id
    AlreadyExistEdgeAtId(EdgeId),
    /// edge not exist at the specified id.
    ///
    /// arg: edge_id
    NotExistEdgeAtId(EdgeId),
    /// not support edge error
    ///
    /// arg: edge_id, edge
    EdgeNotSupported(EdgeId, ErrorEdge<NodeId, EdgeId>),
    /// not available edge error
    ///
    /// arg: edge_id, edge
    IllegalEdge(EdgeId, ErrorEdge<NodeId, EdgeId>),
    /// exist same edge error
    ///
    /// arg: edge_id, edge, same_edge_ids
    ExistSameEdge(EdgeId, ErrorEdge<NodeId, EdgeId>, Vec<EdgeId>),
    /// specified node ids as incidence node for the edge has illegal status.
    ///
    /// arg: edge_id, edge, node_ids
    SpecifiedIllegalIncidenceNodeIds(EdgeId, ErrorEdge<NodeId, EdgeId>, Vec<NodeId>),
    /// specified node ids as incidence node for the edge are not exist when not use the mode to create not exist vertex node.
    ///
    /// arg: edge_id, edge, node_ids
    NotExistChildrenCannotMakeAsEdgeIncidence(EdgeId, ErrorEdge<NodeId, EdgeId>, Vec<NodeId>),
}

impl<NodeId: Identity, EdgeId: Identity> fmt::Display for GraphError<NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphError::*;

        match self {
            // Node
            AlreadyExistNodeAtId(node_id) => {
                write!(f, "Already node is exist at the id {:?}.", node_id)
            }
            NotExistNodeAtId(node_id) => {
                write!(
                    f,
                    "Specified target node is not exist at the id {:?}.",
                    node_id
                )
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
            NotExistChildrenCannotMakeAsGroupChild(group_node_id, child_node_ids) => {
                write!(f, "Specified the group {:?} children {{", group_node_id)?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}} cannot be made.")
            }

            // Edge
            AlreadyExistEdgeAtId(edge_id) => {
                write!(f, "Already edge is exist at the id {:?}.", edge_id)
            }
            NotExistEdgeAtId(edge_id) => {
                write!(
                    f,
                    "Specified target edge is not exist at the id {:?}.",
                    edge_id
                )
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
            NotExistChildrenCannotMakeAsEdgeIncidence(edge_id, edge, node_ids) => {
                write!(f, "Specified nodes {{")?;
                for (index, node_id) in node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", node_id)?;
                    } else {
                        write!(f, ", {:?}", node_id)?;
                    }
                }
                write!(
                    f,
                    "}} cannot be made as incidence node for the edge {} ath the id {:?}.",
                    edge, edge_id
                )
            }
        }
    }
}

impl<NodeId: Identity, EdgeId: Identity> Error for GraphError<NodeId, EdgeId> {}
