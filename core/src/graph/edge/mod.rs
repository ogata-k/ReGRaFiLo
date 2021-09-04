//! Module for edge and it's store
pub mod iter;

use crate::graph::{GraphConfig, Incidence, Node};
use crate::util::Identity;
use std::borrow::Borrow;
use std::collections::btree_map::Entry;
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
    pub fn undirected(id1: Id, id2: Id) -> Self {
        Self::undirected_with_weight(id1, id2, 1_i16)
    }

    /// constructor for directed edge
    pub fn directed(source_id: Id, target_id: Id) -> Self {
        Self::directed_with_weight(source_id, target_id, 1_i16)
    }

    /// constructor for undirected hyper edge
    pub fn undirected_hyper(ids: Vec<Id>) -> Self {
        Self::undirected_hyper_with_weight(ids, 1_i16)
    }

    /// constructor for directed hyper edge
    pub fn directed_hyper(source_ids: Vec<Id>, target_ids: Vec<Id>) -> Self {
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

    /// Generate incidences data from the edge with assume that we already check support edge.
    pub fn generate_incidences_without_check(&self, edge_id: &Id) -> Vec<(Id, Incidence<Id>)> {
        let mut result = Vec::new();
        // No check support incidence with config
        match &self {
            Edge::Undirected { ids, .. } => {
                for node_id in ids {
                    result.push((node_id.clone(), Incidence::undirected(edge_id.clone())));
                }
            }
            Edge::Directed {
                source_id,
                target_id,
                ..
            } => {
                result.push((
                    source_id.clone(),
                    Incidence::directed_source(edge_id.clone()),
                ));
                result.push((
                    target_id.clone(),
                    Incidence::directed_target(edge_id.clone()),
                ));
            }
            Edge::UndirectedHyper { ids, .. } => {
                for node_id in ids {
                    result.push((
                        node_id.clone(),
                        Incidence::undirected_hyper(edge_id.clone()),
                    ));
                }
            }
            Edge::DirectedHyper {
                source_ids,
                target_ids,
                ..
            } => {
                for source_id in source_ids {
                    result.push((
                        source_id.clone(),
                        Incidence::directed_hyper_source(edge_id.clone()),
                    ));
                }

                for target_id in target_ids {
                    result.push((
                        target_id.clone(),
                        Incidence::directed_hyper_target(edge_id.clone()),
                    ));
                }
            }
        }

        result
    }

    /// get node_ids from the edge's incidenes
    pub fn incidence_into_node_ids(self) -> Vec<Id> {
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

    /// get node_ids from the edge's incidenes
    pub fn get_incidence_node_ids(&self) -> Vec<Id> {
        self.clone().incidence_into_node_ids()
    }

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---
    /// check edge is same to other edge without weight
    pub fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        use Edge::*;

        match (self, other) {
            (Undirected { ids, .. }, Undirected { ids: other_ids, .. }) => ids == other_ids,
            (
                Directed {
                    source_id,
                    target_id,
                    ..
                },
                Directed {
                    source_id: other_source_id,
                    target_id: other_target_id,
                    ..
                },
            ) => source_id == other_source_id && target_id == other_target_id,
            (UndirectedHyper { ids, .. }, UndirectedHyper { ids: other_ids, .. }) => {
                ids == other_ids
            }
            (
                DirectedHyper {
                    source_ids,
                    target_ids,
                    ..
                },
                DirectedHyper {
                    source_ids: other_source_ids,
                    target_ids: other_target_ids,
                    ..
                },
            ) => source_ids == other_source_ids && target_ids == other_target_ids,
            _ => false,
        }
    }

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

    /// check edge is undirected or directed edge
    pub fn is_edge(&self) -> bool {
        match self {
            Self::Undirected { .. } | Self::Directed { .. } => true,
            _ => false,
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

    /// check edge is undirected or directed hyper edge
    pub fn is_hyper_edge(&self) -> bool {
        match self {
            Self::UndirectedHyper { .. } | Self::DirectedHyper { .. } => true,
            _ => false,
        }
    }

    /// check configure support this edge type.
    pub fn is_support(&self, config: &GraphConfig) -> bool {
        use Edge::*;

        match self {
            Undirected { .. } => config.can_use_undirected_edge(),
            Directed { .. } => config.can_use_directed_edge(),
            UndirectedHyper { .. } => {
                config.can_use_node_group() || config.can_use_undirected_hyper_edge()
            }
            DirectedHyper { .. } => config.can_use_directed_hyper_edge(),
        }
    }

    /// check edge has illegal parameter
    pub fn has_illegal(&self) -> bool {
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

/// Store structure for edge.
#[derive(Eq, PartialEq, Clone)]
pub struct EdgeStore<Id: Identity> {
    inner: BTreeMap<Id, Edge<Id>>,
}

impl<Id: Identity> fmt::Debug for EdgeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

impl<Id: Identity> fmt::Display for EdgeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_first = true;
        f.write_str("{")?;
        for (edge_id, edge) in self.inner.iter() {
            if is_first {
                f.write_fmt(format_args!("{:?}:{}", edge_id, edge))?;
            } else {
                f.write_fmt(format_args!(", {:?}:{}", edge_id, edge))?;
            }
            is_first = false;
        }
        f.write_str("}")
    }
}

impl<Id: Identity> EdgeStore<Id> {
    // ---
    // constructor
    // ---

    /// create empty store
    pub fn create() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    // ---
    // getter
    // ---

    // ---
    // setter
    // ---

    /// add edge with pop old edge
    pub fn add_edge_with_pop_old(&mut self, edge_id: Id, edge: Edge<Id>) -> Option<Edge<Id>> {
        self.inner.insert(edge_id, edge)
    }

    // ---
    // checker
    // ---

    /// check exist edge_id
    pub fn has_edge_id<B: ?Sized>(&self, edge_id: &B) -> bool
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.contains_key(edge_id)
    }

    /// check exist same edge
    pub fn exist_same_edge(&mut self, edge: &Edge<Id>) -> bool {
        self.inner
            .iter()
            .filter(|(_, stored_edge)| (*stored_edge).is_equal_to_without_weight(edge))
            .next()
            .is_some()
    }

    /// If edge is undirected hyper edge as node grouping, we cannot use the edge wich has intersect node to other edges.
    pub fn has_intersect_group_without_same<B: ?Sized>(&self, edge_id: &B, edge: &Edge<Id>) -> bool
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Edge::UndirectedHyper { ids, .. } = edge {
            for stored_edge in self
                .inner
                .iter()
                .filter(|(k, v)| {
                    (*k).borrow() != edge_id.borrow() && !(*v).is_equal_to_without_weight(edge)
                })
                .map(|(_, v)| v)
            {
                if let Edge::UndirectedHyper {
                    ids: stored_ids, ..
                } = stored_edge
                {
                    for id in ids.iter() {
                        if stored_ids.contains(id) {
                            return true;
                        }
                    }
                }
            }
            false
        } else {
            false
        }
    }

    // ---
    // delete
    // ---
    /// clear all edges
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// remove and get edge at edge_id
    pub fn pop_edge<B: ?Sized>(&mut self, edge_id: &B) -> Option<Edge<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove(edge_id)
    }

    /// delete edge with same edge and get deleted edge_ids
    /// return value is vector of (node_id, edge_id)
    pub fn remove_by_same_edge_with_collect_removed(&mut self, edge: &Edge<Id>) -> Vec<(Id, Id)> {
        // @todo drain_filterを使った方法に置き換える
        let will_delete_node_ids_edge_ids: Vec<(Id, Id)> = self
            .inner
            .iter()
            .filter(|(_, stored_edge)| (*stored_edge).is_equal_to_without_weight(edge))
            .map(|(stored_edge_id, edge)| {
                let converted: Vec<(Id, Id)> = edge
                    .get_incidence_node_ids()
                    .into_iter()
                    .map(|node_id| (node_id, stored_edge_id.clone()))
                    .collect();

                converted
            })
            .flatten()
            .collect();

        for (_, delete_edge_id) in will_delete_node_ids_edge_ids.iter() {
            self.inner.remove_entry(delete_edge_id);
        }

        will_delete_node_ids_edge_ids
    }

    /// remove node_id and node's incidences from edge store
    /// return value is Vec<(node_id, edge_id>
    pub(crate) fn _remove_node_id_and_illegal_edge_with_collect(
        &mut self,
        deleted_node_id: Id,
        deleted_node: Node<Id>,
    ) -> Vec<(Id, Id)> {
        let Node {
            incidences: deleted_incidences,
            ..
        } = deleted_node;
        let mut will_delete_node_id_edge_id: Vec<(Id, Id)> = Vec::new();
        for incidence in deleted_incidences.into_iter() {
            match incidence {
                Incidence::Undirected { edge_id } => {
                    let edge_entry = self.inner.entry(edge_id);
                    match edge_entry {
                        Entry::Vacant(_) => {
                            // If already remove the edge, not exist.
                            continue;
                        }
                        Entry::Occupied(occupied) => {
                            if let Edge::Undirected { ids, .. } = occupied.get() {
                                // This edge is illegal because exist edge remove node_id from ids
                                // remove node id from ids
                                let remove_first = &deleted_node_id == &ids[0];
                                let remove_second = &deleted_node_id == &ids[1];

                                // remove illegal edge
                                if remove_first || remove_second {
                                    if let (
                                        remove_edge_id,
                                        Edge::Undirected {
                                            ids: removable_node_ids,
                                            ..
                                        },
                                    ) = occupied.remove_entry()
                                    {
                                        let [first_node_id, second_node_id] = removable_node_ids;

                                        // if remove first or second
                                        match (remove_first, remove_second) {
                                            (false, true) => {
                                                // retain first
                                                will_delete_node_id_edge_id
                                                    .push((first_node_id, remove_edge_id));
                                            }
                                            (true, false) => {
                                                // retain second
                                                will_delete_node_id_edge_id
                                                    .push((second_node_id, remove_edge_id));
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        unreachable!();
                                    }
                                }
                            } else {
                                panic!(
                                    "Unknown edge {} for incidence {}",
                                    occupied.get(),
                                    Incidence::Undirected {
                                        edge_id: occupied.key()
                                    }
                                )
                            }
                        }
                    }
                }
                Incidence::DirectedSource { edge_id } | Incidence::DirectedTarget { edge_id } => {
                    let edge_entry = self.inner.entry(edge_id);
                    match edge_entry {
                        Entry::Vacant(_) => {
                            // If already remove the edge, not exist.
                            continue;
                        }
                        Entry::Occupied(occupied) => {
                            if let Edge::Directed {
                                source_id,
                                target_id,
                                ..
                            } = occupied.get()
                            {
                                // This edge is illegal because exist edge remove node_id from ids
                                // remove node id from ids
                                let remove_source = &deleted_node_id == source_id;
                                let remove_target = &deleted_node_id == target_id;

                                // remove illegal edge
                                if remove_source || remove_target {
                                    if let (
                                        remove_edge_id,
                                        Edge::Directed {
                                            source_id: source_node_id,
                                            target_id: target_node_id,
                                            ..
                                        },
                                    ) = occupied.remove_entry()
                                    {
                                        // if remove source or target
                                        match (remove_source, remove_target) {
                                            (false, true) => {
                                                // retain source
                                                will_delete_node_id_edge_id
                                                    .push((source_node_id, remove_edge_id));
                                            }
                                            (true, false) => {
                                                // retain target
                                                will_delete_node_id_edge_id
                                                    .push((target_node_id, remove_edge_id));
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        unreachable!();
                                    }
                                }
                            } else {
                                panic!(
                                    "Unknown edge {} for incidence {}",
                                    occupied.get(),
                                    Incidence::Undirected {
                                        edge_id: occupied.key()
                                    }
                                )
                            }
                        }
                    }
                }
                Incidence::UndirectedHyper { edge_id } => {
                    let edge_entry = self.inner.entry(edge_id);
                    match edge_entry {
                        Entry::Vacant(_) => {
                            // If already remove the edge, not exist.
                            continue;
                        }
                        Entry::Occupied(mut occupied) => {
                            if let Edge::UndirectedHyper { ids, .. } = occupied.get_mut() {
                                // This edge is illegal because exist edge remove node_id from ids
                                // remove node id from ids
                                ids.retain(|id| &deleted_node_id != id);

                                // remove illegal edge
                                if ids.is_empty() {
                                    let _ = occupied.remove_entry();
                                    // none removable incidence edge
                                }
                            } else {
                                panic!(
                                    "Unknown edge {} for incidence {}",
                                    occupied.get(),
                                    Incidence::Undirected {
                                        edge_id: occupied.key()
                                    }
                                )
                            }
                        }
                    }
                }
                Incidence::DirectedHyperSource { edge_id }
                | Incidence::DirectedHyperTarget { edge_id } => {
                    let edge_entry = self.inner.entry(edge_id);
                    match edge_entry {
                        Entry::Vacant(_) => {
                            // If already remove the edge, not exist.
                            continue;
                        }
                        Entry::Occupied(mut occupied) => {
                            if let Edge::DirectedHyper {
                                source_ids,
                                target_ids,
                                ..
                            } = occupied.get_mut()
                            {
                                // This edge is illegal because exist edge remove node_id from ids
                                // remove node id from ids
                                source_ids.retain(|id| &deleted_node_id != id);
                                target_ids.retain(|id| &deleted_node_id != id);

                                // remove illegal edge
                                if source_ids.is_empty() || target_ids.is_empty() {
                                    if let (
                                        remove_edge_id,
                                        Edge::DirectedHyper {
                                            source_ids: removable_source_node_ids,
                                            target_ids: removable_target_node_ids,
                                            ..
                                        },
                                    ) = occupied.remove_entry()
                                    {
                                        for source_node_id in removable_source_node_ids {
                                            // retain source
                                            will_delete_node_id_edge_id
                                                .push((source_node_id, remove_edge_id.clone()));
                                        }
                                        for target_node_id in removable_target_node_ids {
                                            // retain source
                                            will_delete_node_id_edge_id
                                                .push((target_node_id, remove_edge_id.clone()));
                                        }
                                    } else {
                                        unreachable!();
                                    }
                                }
                            } else {
                                panic!(
                                    "Unknown edge {} for incidence {}",
                                    occupied.get(),
                                    Incidence::Undirected {
                                        edge_id: occupied.key()
                                    }
                                )
                            }
                        }
                    }
                }
            }
        }

        will_delete_node_id_edge_id
    }
}
