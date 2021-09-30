//! Module for Convert to model

use crate::graph::model;
use crate::graph::store::{Incidence, Node};
use crate::util::Identity;

/// convert to Node's Incidence model
pub(in crate::graph) trait AsNodeIncidenceModel<Id: Identity> {
    /// create model as node's incidence edge
    fn as_model<'a>(&'a self) -> model::Incidence<'a, Id>;
}

impl<Id: Identity> AsNodeIncidenceModel<Id> for Incidence<Id> {
    fn as_model<'a>(&'a self) -> model::Incidence<'a, Id> {
        match self {
            Incidence::Undirected { edge_id } => model::Incidence::Undirected { edge_id: &edge_id },
            Incidence::DirectedSource { edge_id } => {
                model::Incidence::DirectedSource { edge_id: &edge_id }
            }
            Incidence::DirectedTarget { edge_id } => {
                model::Incidence::DirectedTarget { edge_id: &edge_id }
            }
            Incidence::UndirectedHyper { edge_id } => {
                model::Incidence::UndirectedHyper { edge_id: &edge_id }
            }
            Incidence::DirectedHyperSource { edge_id } => {
                model::Incidence::DirectedHyperSource { edge_id: &edge_id }
            }
            Incidence::DirectedHyperTarget { edge_id } => {
                model::Incidence::DirectedHyperTarget { edge_id: &edge_id }
            }
        }
    }
}

/// convert to Node model
pub(in crate::graph) trait AsNodeModel<Id: Identity> {
    /// create model as node
    fn as_model<'a>(&'a self) -> model::Node<'a, Id>;

    /// create model as node point
    fn as_vertex_model<'a>(&'a self) -> Option<model::VertexNode<'a, Id>>;

    /// create model as node group
    fn as_group_model<'a>(&'a self) -> Option<model::GroupNode<'a, Id>>;
}

impl<Id: Identity> AsNodeModel<Id> for Node<Id> {
    /// create model as node
    #[inline]
    fn as_model<'a>(&'a self) -> model::Node<'a, Id> {
        match self {
            Node::Vertex {
                weight,
                parent,
                incidences,
            } => model::Node::Vertex(model::VertexNode {
                weight,
                parent,
                incidences: incidences
                    .iter()
                    .map(|incidence| incidence.as_model())
                    .collect(),
            }),
            Node::Group {
                weight,
                parent,
                children,
                incidences,
            } => model::Node::Group(model::GroupNode {
                weight,
                parent,
                children,
                incidences: incidences
                    .iter()
                    .map(|incidence| incidence.as_model())
                    .collect(),
            }),
        }
    }

    /// create model as node point
    #[inline]
    fn as_vertex_model<'a>(&'a self) -> Option<model::VertexNode<'a, Id>> {
        match &self {
            Node::Vertex {
                weight,
                parent,
                incidences,
            } => Some(model::VertexNode {
                weight,
                parent,
                incidences: incidences
                    .iter()
                    .map(|incidence| incidence.as_model())
                    .collect(),
            }),
            _ => None,
        }
    }

    /// create model as node group
    #[inline]
    fn as_group_model<'a>(&'a self) -> Option<model::GroupNode<'a, Id>> {
        match &self {
            Node::Group {
                weight,
                parent,
                children,
                incidences,
            } => Some(model::GroupNode {
                weight,
                parent,
                children,
                incidences: incidences
                    .iter()
                    .map(|incidence| incidence.as_model())
                    .collect(),
            }),
            _ => None,
        }
    }
}
