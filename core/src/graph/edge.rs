//! Module for edge and it's store

use crate::graph::GraphConfig;
use crate::util::Identity;
use std::collections::BTreeMap;
use std::fmt;

/// helper for sort id and for sort with distinct.
/// If sort for undirected or directed edge, must not use distinct option because of exist self loop.
fn _sort_ids<T: Ord>(vec: &mut Vec<T>, distinct: bool) {
    vec.sort();
    if distinct {
        vec.dedup();
    }
}

/// Edge status for graph edge.
/// If edge's node-ids is vector or array, the ids is always sorted (it sort when onstruct).
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Edge<Id: Identity> {
    /// undirected edge
    Undirected { weight: i16, ids: [Id; 2] },

    /// Directed edge
    Directed {
        weight: i16,
        source_id: Id,
        target_id: Id,
    },

    /// undirected Hyper edge
    UndirectedHyper { weight: i16, ids: Vec<Id> },

    /// Directed Hyper edge
    DirectedHyper {
        weight: i16,
        source_ids: Vec<Id>,
        target_ids: Vec<Id>,
    },
}

impl<Id: Identity> Edge<Id> {
    // ---
    // constructor
    // ---

    /// constructor for undirected edge
    pub fn undirected(id1: Id, id2: Id) -> Self {
        Self::undirected_with_weight(id1, id2, 1_i16)
    }

    /// constructor for directed edge
    pub fn directed(source_id: Id, target_id: Id) -> Self {
        Self::directed_with_weight(source_id, target_id, 1_i16)
    }

    /// constructor for undirected hyper edge
    pub fn undirected_hyper(mut ids: Vec<Id>) -> Self {
        Self::undirected_hyper_with_weight(ids, 1_i16)
    }

    /// constructor for directed hyper edge
    pub fn directed_hyper(mut source_ids: Vec<Id>, mut target_ids: Vec<Id>) -> Self {
        Self::directed_hyper_with_weight(source_ids, target_ids, 1_i16)
    }

    /// constructor for undirected edge with edge
    pub fn undirected_with_weight(id1: Id, id2: Id, weight: i16) -> Self {
        if &id1 <= &id2 {
            Self::Undirected {
                weight: weight,
                ids: [id1, id2],
            }
        } else {
            Self::Undirected {
                weight: weight,
                ids: [id2, id1],
            }
        }
    }

    /// constructor for directed edge with weight
    pub fn directed_with_weight(source_id: Id, target_id: Id, weight: i16) -> Self {
        Self::Directed {
            weight: weight,
            source_id: source_id,
            target_id: target_id,
        }
    }

    /// constructor for undirected hyper edge with weight
    pub fn undirected_hyper_with_weight(mut ids: Vec<Id>, weight: i16) -> Self {
        _sort_ids(&mut ids, true);

        Self::UndirectedHyper {
            weight: weight,
            ids: ids,
        }
    }

    /// constructor for directed hyper edge with weight
    pub fn directed_hyper_with_weight(
        mut source_ids: Vec<Id>,
        mut target_ids: Vec<Id>,
        weight: i16,
    ) -> Self {
        _sort_ids(&mut source_ids, true);
        _sort_ids(&mut target_ids, true);

        Self::DirectedHyper {
            weight: weight,
            source_ids: source_ids,
            target_ids: target_ids,
        }
    }

    // ---
    // getter
    // ---

    /// get weight.
    /// If weight is 1 or no weight, the edge's weight is 1.
    pub fn get_weight(&self) -> &i16 {
        use Edge::*;

        match self {
            Undirected { weight, .. }
            | Directed { weight, .. }
            | UndirectedHyper { weight, .. }
            | DirectedHyper { weight, .. } => weight,
        }
    }

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---

    /// check edge is undirected edge
    pub fn is_undirected(&self) -> bool {
        if let Self::Undirected { .. } = self {
            true
        } else {
            false
        }
    }

    /// check edge is directed edge
    pub fn is_directed(&self) -> bool {
        if let Self::Directed { .. } = self {
            true
        } else {
            false
        }
    }

    /// check edge is undirected hyper edge
    pub fn is_undirected_hyper(&self) -> bool {
        if let Self::UndirectedHyper { .. } = self {
            true
        } else {
            false
        }
    }

    /// check edge is directed hyper edge
    pub fn is_directed_hyper(&self) -> bool {
        if let Self::DirectedHyper { .. } = self {
            true
        } else {
            false
        }
    }

    /// check configure support this edge type.
    pub fn is_support(&self, config: &GraphConfig) -> bool {
        use Edge::*;

        match self {
            Undirected { .. } => config.is_undirected_graph() || config.is_mixed_graph(),
            Directed { .. } => config.is_directed_graph() || config.is_mixed_graph(),
            UndirectedHyper { .. } => {
                config.is_hyper_graph() || config.is_mixed_hyper_graph() || config.has_group()
            }
            DirectedHyper { .. } => config.is_hyper_graph() || config.is_mixed_hyper_graph(),
        }
    }

    // ---
    // delete
    // ---
}

/// Store structure for edge.
#[derive(Eq, PartialEq, Clone)]
pub struct EdgeStore<Id: Identity> {
    inner: BTreeMap<Id, Edge<Id>>,
}

impl<Id: Identity> Default for EdgeStore<Id> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<Id: Identity + fmt::Debug> fmt::Debug for EdgeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

impl<Id: Identity> EdgeStore<Id> {
    // TODO 必要な実装を必要な時に

    // ---
    // constructor
    // ---

    // ---
    // getter
    // ---

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---

    // ---
    // delete
    // ---
}
