//! Module for Convert to Edge model

use crate::graph::model;
use crate::graph::store::Edge;
use crate::util::Identity;

/// convert to Edge model
pub(in crate::graph) trait AsEdgeModel<Id: Identity> {
    /// create model as edge
    fn as_model<'a>(&'a self) -> model::Edge<'a, Id>;

    /// create model as undirected edge
    fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, Id>>;

    /// create model as directed edge
    fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, Id>>;

    /// create model as mixed edge
    fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, Id>>;

    /// create model as undirected hyper edge
    fn as_undirected_hyper_model<'a>(&'a self) -> Option<model::UndirectedHyperEdge<'a, Id>>;

    /// create model as mixed hyper edge
    fn as_directed_hyper_model<'a>(&'a self) -> Option<model::DirectedHyperEdge<'a, Id>>;

    /// create model as mixed hyper edge
    fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, Id>>;
}

impl<Id: Identity> AsEdgeModel<Id> for Edge<Id> {
    /// create model as edge
    #[inline]
    fn as_model<'a>(&'a self) -> model::Edge<'a, Id> {
        match self {
            Edge::Undirected { weight, ids } => model::Edge::Undirected(model::UndirectedEdge {
                weight,
                incidence: ids,
            }),
            Edge::Directed {
                weight,
                source_id,
                target_id,
            } => model::Edge::Directed(model::DirectedEdge {
                weight,
                incidence: (source_id, target_id),
            }),
            Edge::UndirectedHyper { weight, ids } => {
                model::Edge::UndirectedHyper(model::UndirectedHyperEdge {
                    weight,
                    incidence: ids,
                })
            }
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => model::Edge::DirectedHyper(model::DirectedHyperEdge {
                weight,
                incidence: (source_ids, target_ids),
            }),
        }
    }

    /// create model as undirected edge
    #[inline]
    fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, Id>> {
        match self {
            Edge::Undirected { weight, ids } => Some(model::UndirectedEdge {
                weight,
                incidence: ids,
            }),
            _ => None,
        }
    }

    /// create model as directed edge
    #[inline]
    fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, Id>> {
        match self {
            Edge::Directed {
                weight,
                source_id,
                target_id,
            } => Some(model::DirectedEdge {
                weight,
                incidence: (source_id, target_id),
            }),
            _ => None,
        }
    }

    /// create model as mixed edge
    #[inline]
    fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, Id>> {
        match self {
            Edge::Undirected { weight, ids } => {
                Some(model::MixedEdge::Undirected(model::UndirectedEdge {
                    weight,
                    incidence: ids,
                }))
            }
            Edge::Directed {
                weight,
                source_id,
                target_id,
            } => Some(model::MixedEdge::Directed(model::DirectedEdge {
                weight,
                incidence: (source_id, target_id),
            })),
            Edge::UndirectedHyper { .. } | Edge::DirectedHyper { .. } => None,
        }
    }

    /// create model as undirected hyper edge
    #[inline]
    fn as_undirected_hyper_model<'a>(&'a self) -> Option<model::UndirectedHyperEdge<'a, Id>> {
        match self {
            Edge::UndirectedHyper { weight, ids } => Some(model::UndirectedHyperEdge {
                weight,
                incidence: ids,
            }),
            _ => None,
        }
    }

    /// create model as mixed hyper edge
    #[inline]
    fn as_directed_hyper_model<'a>(&'a self) -> Option<model::DirectedHyperEdge<'a, Id>> {
        match self {
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => Some(model::DirectedHyperEdge {
                weight,
                incidence: (source_ids, target_ids),
            }),
            _ => None,
        }
    }

    /// create model as mixed hyper edge
    #[inline]
    fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, Id>> {
        match self {
            Edge::Undirected { .. } | Edge::Directed { .. } => None,
            Edge::UndirectedHyper { weight, ids } => Some(model::MixedHyperEdge::Undirected(
                model::UndirectedHyperEdge {
                    weight,
                    incidence: ids,
                },
            )),
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => Some(model::MixedHyperEdge::Directed(model::DirectedHyperEdge {
                weight,
                incidence: (source_ids, target_ids),
            })),
        }
    }
}
