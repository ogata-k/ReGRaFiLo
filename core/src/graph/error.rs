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
    // Node
    /// already node exist at the id.
    ///
    /// arg: node_id
    AlreadyExistNodeAtId(Id),
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
    NotExistChildren(Id, Vec<Id>),

    // Edge
    /// already edge exist at the id.
    ///
    /// arg: edge_id
    AlreadyExistEdgeAtId(Id),
    /// not support edge error
    /// arg: edge_id, edge
    EdgeNotSupported(Id, ErrorEdge<Id>),
    /// not available edge error
    /// arg: edge_id, edge
    IllegalEdge(Id, ErrorEdge<Id>),
    /// exist same edge error
    /// arg: edge_id, edge
    ExistSameEdge(Id, ErrorEdge<Id>),
    NotSameNodeGroupHaveIntersect(Id, ErrorEdge<Id>),
}

impl<Id: Identity> fmt::Display for GraphError<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphError::*;

        match self {
            // Node
            AlreadyExistNodeAtId(node_id)=>write!(
                f,
                "Already node exist at the id {:?}.",
                node_id
            ),
            NotSupportGroupNode(group_node_id) => write!(
                f,
                "Not support group node at the id {:?}.",
                group_node_id
            ),
            CannotCreateVertex(Some(parent_node_id), vertex_node_id)=>write!(
                f,
                "Cannot create vertex node at the id {:?} in the parent {:?}.",
                vertex_node_id, parent_node_id
            ),
            CannotCreateVertex(None, vertex_node_id)=>write!(
                f,
                "Cannot create vertex node at the id {:?}.",
                vertex_node_id
            ),
            CannotCreateGroup(Some(parent_node_id), group_node_id, child_node_ids) => {
                write!(f, "Cannot create group node at the id {:?} with child {{", group_node_id)?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}} in the parent {:?}.", parent_node_id)
            },
            CannotCreateGroup(None, group_node_id, child_node_ids) => {
                write!(f, "Cannot create group node at the id {:?} with child {{", group_node_id)?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}}.")
            },
            NotExistGroup(group_node_id)=>write!(
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
                write!(f, "}} have illegal children for the group {:?}.", group_node_id)
            },
            NotExistChildren(group_node_id, child_node_ids) => {
                write!(f, "Specified children {{")?;
                for (index, child_node_id) in child_node_ids.iter().enumerate() {
                    if index == 0 {
                        write!(f, "{:?}", child_node_id)?;
                    } else {
                        write!(f, ", {:?}", child_node_id)?;
                    }
                }
                write!(f, "}} are not exist for the group {:?}.", group_node_id)
            },

            // Edge
            AlreadyExistEdgeAtId(edge_id)=>write!(
                f,
                "Already edge exist at the id {:?}.",
                edge_id
            ),
            EdgeNotSupported(edge_id, edge) => write!(
                f,
                "Not support edge which is the edge {} at the id {:?}.",
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
