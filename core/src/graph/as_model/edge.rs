//! Module for Convert to Edge model

use crate::graph::model;
use crate::graph::store::Edge;
use crate::util::Identity;
use std::marker::PhantomData;

/// convert to Edge model
pub(in crate::graph) trait AsEdgeModel<NodeId: Identity, EdgeId: Identity> {
    /// create model as edge
    fn as_model<'a>(&'a self) -> model::Edge<'a, NodeId, EdgeId>;

    /// create model as undirected edge
    fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, NodeId, EdgeId>>;

    /// create model as directed edge
    fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, NodeId, EdgeId>>;

    /// create model as mixed edge
    fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, NodeId, EdgeId>>;

    /// create model as undirected hyper edge
    fn as_undirected_hyper_model<'a>(
        &'a self,
    ) -> Option<model::UndirectedHyperEdge<'a, NodeId, EdgeId>>;

    /// create model as mixed hyper edge
    fn as_directed_hyper_model<'a>(
        &'a self,
    ) -> Option<model::DirectedHyperEdge<'a, NodeId, EdgeId>>;

    /// create model as mixed hyper edge
    fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, NodeId, EdgeId>>;
}

impl<NodeId: Identity, EdgeId: Identity> AsEdgeModel<NodeId, EdgeId> for Edge<NodeId, EdgeId> {
    /// create model as edge
    #[inline]
    fn as_model<'a>(&'a self) -> model::Edge<'a, NodeId, EdgeId> {
        match self {
            Edge::Undirected { weight, ids, .. } => {
                model::Edge::Undirected(model::UndirectedEdge {
                    weight,
                    incidence: ids,
                    _edge_id: PhantomData,
                })
            }
            Edge::Directed {
                weight,
                source_id,
                target_id,
                ..
            } => model::Edge::Directed(model::DirectedEdge {
                weight,
                incidence: (source_id, target_id),
                _edge_id: PhantomData,
            }),
            Edge::UndirectedHyper { weight, ids, .. } => {
                model::Edge::UndirectedHyper(model::UndirectedHyperEdge {
                    weight,
                    incidence: ids,
                    _edge_id: PhantomData,
                })
            }
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
                ..
            } => model::Edge::DirectedHyper(model::DirectedHyperEdge {
                weight,
                incidence: (source_ids, target_ids),
                _edge_id: PhantomData,
            }),
        }
    }

    /// create model as undirected edge
    #[inline]
    fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, NodeId, EdgeId>> {
        match self {
            Edge::Undirected { weight, ids, .. } => Some(model::UndirectedEdge {
                weight,
                incidence: ids,
                _edge_id: PhantomData,
            }),
            _ => None,
        }
    }

    /// create model as directed edge
    #[inline]
    fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, NodeId, EdgeId>> {
        match self {
            Edge::Directed {
                weight,
                source_id,
                target_id,
                ..
            } => Some(model::DirectedEdge {
                weight,
                incidence: (source_id, target_id),
                _edge_id: PhantomData,
            }),
            _ => None,
        }
    }

    /// create model as mixed edge
    #[inline]
    fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, NodeId, EdgeId>> {
        match self {
            Edge::Undirected { weight, ids, .. } => {
                Some(model::MixedEdge::Undirected(model::UndirectedEdge {
                    weight,
                    incidence: ids,
                    _edge_id: PhantomData,
                }))
            }
            Edge::Directed {
                weight,
                source_id,
                target_id,
                ..
            } => Some(model::MixedEdge::Directed(model::DirectedEdge {
                weight,
                incidence: (source_id, target_id),
                _edge_id: PhantomData,
            })),
            Edge::UndirectedHyper { .. } | Edge::DirectedHyper { .. } => None,
        }
    }

    /// create model as undirected hyper edge
    #[inline]
    fn as_undirected_hyper_model<'a>(
        &'a self,
    ) -> Option<model::UndirectedHyperEdge<'a, NodeId, EdgeId>> {
        match self {
            Edge::UndirectedHyper { weight, ids, .. } => Some(model::UndirectedHyperEdge {
                weight,
                incidence: ids,
                _edge_id: PhantomData,
            }),
            _ => None,
        }
    }

    /// create model as mixed hyper edge
    #[inline]
    fn as_directed_hyper_model<'a>(
        &'a self,
    ) -> Option<model::DirectedHyperEdge<'a, NodeId, EdgeId>> {
        match self {
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
                ..
            } => Some(model::DirectedHyperEdge {
                weight,
                incidence: (source_ids, target_ids),
                _edge_id: PhantomData,
            }),
            _ => None,
        }
    }

    /// create model as mixed hyper edge
    #[inline]
    fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, NodeId, EdgeId>> {
        match self {
            Edge::Undirected { .. } | Edge::Directed { .. } => None,
            Edge::UndirectedHyper { weight, ids, .. } => Some(model::MixedHyperEdge::Undirected(
                model::UndirectedHyperEdge {
                    weight,
                    incidence: ids,
                    _edge_id: PhantomData,
                },
            )),
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
                ..
            } => Some(model::MixedHyperEdge::Directed(model::DirectedHyperEdge {
                weight,
                incidence: (source_ids, target_ids),
                _edge_id: PhantomData,
            })),
        }
    }
}
