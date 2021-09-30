//! Module for Edge item

use crate::util::Identity;
use std::fmt;

/// helper for sort id and for sort with distinct.
/// If sort for undirected or directed edge, must not use distinct option because of exist self loop.
fn sort_ids<T: Ord>(vec: &mut Vec<T>, distinct: bool) {
    vec.sort();
    if distinct {
        vec.dedup();
    }
}

/// Edge status for graph edge.
/// If edge's node-ids is vector or array, the ids is always sorted (it sort when onstruct).
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub(in crate::graph) enum Edge<Id: Identity> {
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

impl<Id: Identity> fmt::Display for Edge<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Edge::*;

        match self {
            Undirected { weight, ids } => f.write_fmt(format_args!(
                "{{weight: {}, link: {:?}--{:?}}}",
                weight, ids[0], ids[1]
            )),
            Directed {
                weight,
                source_id,
                target_id,
            } => f.write_fmt(format_args!(
                "{{weight: {}, link: {:?}->{:?}}}",
                weight, source_id, target_id
            )),
            UndirectedHyper { weight, ids } => {
                f.write_fmt(format_args!("{{weight: {}, link: ", weight))?;
                f.debug_set().entries(ids.iter()).finish()?;
                f.write_str("}")
            }
            DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => {
                f.write_fmt(format_args!("{{weight: {}, link: ", weight))?;
                f.debug_set().entries(source_ids.iter()).finish()?;
                f.write_str("->")?;
                f.debug_set().entries(target_ids.iter()).finish()?;
                f.write_str("}")
            }
        }
    }
}

impl<Id: Identity> Edge<Id> {
    // ---
    // constructor
    // ---

    /// constructor for undirected edge
    pub(in crate::graph) fn undirected(id1: Id, id2: Id) -> Self {
        Self::undirected_with_weight(id1, id2, 1_i16)
    }

    /// constructor for directed edge
    pub(in crate::graph) fn directed(source_id: Id, target_id: Id) -> Self {
        Self::directed_with_weight(source_id, target_id, 1_i16)
    }

    /// constructor for undirected hyper edge
    pub(in crate::graph) fn undirected_hyper(ids: Vec<Id>) -> Self {
        Self::undirected_hyper_with_weight(ids, 1_i16)
    }

    /// constructor for directed hyper edge
    pub(in crate::graph) fn directed_hyper(source_ids: Vec<Id>, target_ids: Vec<Id>) -> Self {
        Self::directed_hyper_with_weight(source_ids, target_ids, 1_i16)
    }

    /// constructor for undirected edge with edge
    pub(in crate::graph) fn undirected_with_weight(id1: Id, id2: Id, weight: i16) -> Self {
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
    pub(in crate::graph) fn directed_with_weight(
        source_id: Id,
        target_id: Id,
        weight: i16,
    ) -> Self {
        Self::Directed {
            weight: weight,
            source_id: source_id,
            target_id: target_id,
        }
    }

    /// constructor for undirected hyper edge with weight
    pub(in crate::graph) fn undirected_hyper_with_weight(mut ids: Vec<Id>, weight: i16) -> Self {
        sort_ids(&mut ids, true);

        Self::UndirectedHyper {
            weight: weight,
            ids: ids,
        }
    }

    /// constructor for directed hyper edge with weight
    pub(in crate::graph) fn directed_hyper_with_weight(
        mut source_ids: Vec<Id>,
        mut target_ids: Vec<Id>,
        weight: i16,
    ) -> Self {
        sort_ids(&mut source_ids, true);
        sort_ids(&mut target_ids, true);

        Self::DirectedHyper {
            weight: weight,
            source_ids: source_ids,
            target_ids: target_ids,
        }
    }

    // ---
    // getter
    // ---

    /// get weight for the edge.
    /// If weight is 1 or no weight, the edge's weight is 1.
    pub(in crate::graph) fn get_weight(&self) -> i16 {
        match self {
            Edge::Undirected { weight, .. }
            | Edge::Directed { weight, .. }
            | Edge::UndirectedHyper { weight, .. }
            | Edge::DirectedHyper { weight, .. } => *weight,
        }
    }

    /// get node_ids from the edge's incidences
    pub(in crate::graph) fn into_incidence_node_ids(self) -> Vec<Id> {
        match self {
            Edge::Undirected {
                ids: [id1, id2], ..
            } => vec![id1, id2],
            Edge::Directed {
                source_id,
                target_id,
                ..
            } => vec![source_id, target_id],
            Edge::UndirectedHyper { ids, .. } => ids,
            Edge::DirectedHyper {
                source_ids,
                target_ids,
                ..
            } => {
                let mut result = Vec::new();
                result.extend(source_ids);
                result.extend(target_ids);
                result
            }
        }
    }

    /// get node_ids from the edge's incidences
    pub(in crate::graph) fn get_incidence_node_ids_as_ref(&self) -> Vec<&Id> {
        match self {
            Edge::Undirected {
                ids: [id1, id2], ..
            } => vec![id1, id2],
            Edge::Directed {
                source_id,
                target_id,
                ..
            } => vec![source_id, target_id],
            Edge::UndirectedHyper { ids, .. } => ids.iter().collect(),
            Edge::DirectedHyper {
                source_ids,
                target_ids,
                ..
            } => {
                let mut result = Vec::new();
                result.extend(source_ids);
                result.extend(target_ids);
                result
            }
        }
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    pub(in crate::graph) fn get_source_ids(&self) -> Vec<&Id> {
        use Edge::*;

        match self {
            Undirected { .. } | UndirectedHyper { .. } => Vec::new(),
            Directed { source_id, .. } => vec![source_id],
            DirectedHyper { source_ids, .. } => source_ids.iter().collect(),
        }
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    pub(in crate::graph) fn get_target_ids(&self) -> Vec<&Id> {
        use Edge::*;

        match self {
            Undirected { .. } | UndirectedHyper { .. } => Vec::new(),
            Directed { target_id, .. } => vec![target_id],
            DirectedHyper { target_ids, .. } => target_ids.iter().collect(),
        }
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    pub(in crate::graph) fn get_source_target_ids(&self) -> Vec<&Id> {
        use Edge::*;

        match self {
            Undirected { ids, .. } => ids.iter().collect(),
            UndirectedHyper { ids, .. } => ids.iter().collect(),
            Directed { .. } | DirectedHyper { .. } => Vec::new(),
        }
    }

    // ---
    // setter
    // ---

    /// set weight
    pub(in crate::graph) fn set_weight(&mut self, weight: i16) {
        use Edge::*;

        match self {
            Undirected {
                weight: mut _weight,
                ..
            }
            | Directed {
                weight: mut _weight,
                ..
            }
            | UndirectedHyper {
                weight: mut _weight,
                ..
            }
            | DirectedHyper {
                weight: mut _weight,
                ..
            } => _weight = weight,
        }
    }

    // ---
    // checker
    // ---
    /// check edge is same to other edge without weight
    pub(in crate::graph) fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.get_incidence_node_ids_as_ref() == other.get_incidence_node_ids_as_ref()
    }

    /// check edge is undirected edge
    pub(in crate::graph) fn is_undirected(&self) -> bool {
        use Edge::*;

        match self {
            Undirected { .. } => true,
            _ => false,
        }
    }

    /// check edge is directed edge
    pub(in crate::graph) fn is_directed(&self) -> bool {
        use Edge::*;

        match self {
            Directed { .. } => true,
            _ => false,
        }
    }

    /// check edge is undirected or directed edge
    pub(in crate::graph) fn is_edge(&self) -> bool {
        use Edge::*;

        match self {
            Undirected { .. } | Directed { .. } => true,
            _ => false,
        }
    }

    /// check edge is undirected hyper edge
    pub(in crate::graph) fn is_undirected_hyper(&self) -> bool {
        use Edge::*;

        match self {
            UndirectedHyper { .. } => true,
            _ => false,
        }
    }

    /// check edge is directed hyper edge
    pub(in crate::graph) fn is_directed_hyper(&self) -> bool {
        use Edge::*;

        match self {
            DirectedHyper { .. } => true,
            _ => false,
        }
    }

    /// check edge is undirected or directed hyper edge
    pub(in crate::graph) fn is_hyper_edge(&self) -> bool {
        use Edge::*;

        match self {
            UndirectedHyper { .. } | DirectedHyper { .. } => true,
            _ => false,
        }
    }

    /// check edge has illegal parameter
    pub(in crate::graph) fn has_illegal(&self) -> bool {
        match self {
            Edge::Undirected { ids, .. } => ids.len() != 2,
            Edge::Directed {
                source_id: _,
                target_id: _,
                ..
            } => false,
            Edge::UndirectedHyper { ids, .. } => ids.is_empty(),
            Edge::DirectedHyper {
                source_ids,
                target_ids,
                ..
            } => source_ids.is_empty() || target_ids.is_empty(),
        }
    }

    // ---
    // delete
    // ---
}
