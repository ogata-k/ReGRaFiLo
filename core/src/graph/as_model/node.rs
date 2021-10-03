//! Module for Convert to model

use crate::graph::model;
use crate::graph::store::{Incidence, Node};
use crate::util::Identity;
use std::marker::PhantomData;

/// convert to Node's Incidence model
pub(in crate::graph) trait AsNodeIncidenceModel<NodeId: Identity, EdgeId: Identity> {
    /// create model as node's incidence edge
    fn as_model<'a>(&'a self) -> model::Incidence<'a, NodeId, EdgeId>;
}

impl<NodeId: Identity, EdgeId: Identity> AsNodeIncidenceModel<NodeId, EdgeId>
    for Incidence<NodeId, EdgeId>
{
    fn as_model<'a>(&'a self) -> model::Incidence<'a, NodeId, EdgeId> {
        match self {
            Incidence::Undirected { edge_id, .. } => model::Incidence::Undirected {
                edge_id: &edge_id,
                _node_id: PhantomData,
            },
            Incidence::DirectedSource { edge_id, .. } => model::Incidence::DirectedSource {
                edge_id: &edge_id,
                _node_id: PhantomData,
            },
            Incidence::DirectedTarget { edge_id, .. } => model::Incidence::DirectedTarget {
                edge_id: &edge_id,
                _node_id: PhantomData,
            },
            Incidence::UndirectedHyper { edge_id, .. } => model::Incidence::UndirectedHyper {
                edge_id: &edge_id,
                _node_id: PhantomData,
            },
            Incidence::DirectedHyperSource { edge_id, .. } => {
                model::Incidence::DirectedHyperSource {
                    edge_id: &edge_id,
                    _node_id: PhantomData,
                }
            }
            Incidence::DirectedHyperTarget { edge_id, .. } => {
                model::Incidence::DirectedHyperTarget {
                    edge_id: &edge_id,
                    _node_id: PhantomData,
                }
            }
        }
    }
}

/// convert to Node model
pub(in crate::graph) trait AsNodeModel<NodeId: Identity, EdgeId: Identity> {
    /// create model as node
    fn as_model<'a>(&'a self) -> model::Node<'a, NodeId, EdgeId>;

    /// create model as node point
    fn as_vertex_model<'a>(&'a self) -> Option<model::VertexNode<'a, NodeId, EdgeId>>;

    /// create model as node group
    fn as_group_model<'a>(&'a self) -> Option<model::GroupNode<'a, NodeId, EdgeId>>;
}

impl<NodeId: Identity, EdgeId: Identity> AsNodeModel<NodeId, EdgeId> for Node<NodeId, EdgeId> {
    /// create model as node
    #[inline]
    fn as_model<'a>(&'a self) -> model::Node<'a, NodeId, EdgeId> {
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
    fn as_vertex_model<'a>(&'a self) -> Option<model::VertexNode<'a, NodeId, EdgeId>> {
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
    fn as_group_model<'a>(&'a self) -> Option<model::GroupNode<'a, NodeId, EdgeId>> {
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
