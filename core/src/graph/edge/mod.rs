//! Module for edge and it's store

pub mod iter;
pub mod model;

use crate::graph::edge::iter::*;
use crate::graph::edge::model::EdgeModel;
use crate::graph::{GraphConfig, Incidence, Node};
use crate::util::Identity;
use std::borrow::Borrow;
use std::collections::btree_map::{Entry, Iter};
use std::collections::BTreeMap;
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
        let model = self.as_model();
        fmt::Display::fmt(&model, f)
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
        sort_ids(&mut ids, true);

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
        sort_ids(&mut source_ids, true);
        sort_ids(&mut target_ids, true);

        Self::DirectedHyper {
            weight: weight,
            source_ids: source_ids,
            target_ids: target_ids,
        }
    }

    /// create model as edge
    #[inline]
    pub fn as_model<'a>(&'a self) -> model::Edge<'a, Id> {
        model::Edge::_create(&self)
    }

    /// create model as undirected edge
    #[inline]
    pub fn as_undirected_model<'a>(&'a self) -> Option<model::UndirectedEdge<'a, Id>> {
        match self {
            Edge::Undirected { weight, ids } => Some(model::UndirectedEdge::_create(weight, ids)),
            _ => None,
        }
    }

    /// create model as directed edge
    #[inline]
    pub fn as_directed_model<'a>(&'a self) -> Option<model::DirectedEdge<'a, Id>> {
        match self {
            Edge::Directed {
                weight,
                source_id,
                target_id,
            } => Some(model::DirectedEdge::_create(weight, source_id, target_id)),
            _ => None,
        }
    }

    /// create model as mixed edge
    #[inline]
    pub fn as_mixed_model<'a>(&'a self) -> Option<model::MixedEdge<'a, Id>> {
        model::MixedEdge::_create(&self)
    }

    /// create model as undirected hyper edge
    #[inline]
    pub fn as_undirected_hyper_model<'a>(&'a self) -> Option<model::UndirectedHyperEdge<'a, Id>> {
        match self {
            Edge::UndirectedHyper { weight, ids } => {
                Some(model::UndirectedHyperEdge::_create(weight, ids.as_slice()))
            }
            _ => None,
        }
    }

    /// create model as mixed hyper edge
    #[inline]
    pub fn as_directed_hyper_model<'a>(&'a self) -> Option<model::DirectedHyperEdge<'a, Id>> {
        match self {
            Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => Some(model::DirectedHyperEdge::_create(
                weight,
                source_ids.as_slice(),
                target_ids.as_slice(),
            )),
            _ => None,
        }
    }

    /// create model as mixed hyper edge
    #[inline]
    pub fn as_mixed_hyper_model<'a>(&'a self) -> Option<model::MixedHyperEdge<'a, Id>> {
        model::MixedHyperEdge::_create(&self)
    }

    // ---
    // getter
    // ---

    /// get weight for the edge.
    /// If weight is 1 or no weight, the edge's weight is 1.
    pub fn get_weight(&self) -> i16 {
        self.as_model().get_weight()
    }

    /// get kind for the edge.
    pub fn get_kind(&self) -> model::EdgeKind {
        self.as_model().get_kind()
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
    pub fn into_incidence_node_ids(self) -> Vec<Id> {
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
    pub fn get_incidence_node_ids_as_ref(&self) -> Vec<&Id> {
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
    pub fn get_source_ids(&self) -> Vec<&Id> {
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
    pub fn get_target_ids(&self) -> Vec<&Id> {
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
    pub fn get_source_target_ids(&self) -> Vec<&Id> {
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
    pub fn set_weight(&mut self, weight: i16) {
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
    pub fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        let self_model = self.as_model();
        let other_model = other.as_model();

        self_model.is_equal_to_without_weight(&other_model)
    }

    /// check edge is undirected edge
    pub fn is_undirected(&self) -> bool {
        self.as_model().is_undirected()
    }

    /// check edge is directed edge
    pub fn is_directed(&self) -> bool {
        self.as_model().is_directed()
    }

    /// check edge is undirected or directed edge
    pub fn is_edge(&self) -> bool {
        self.as_model().is_edge()
    }

    /// check edge is undirected hyper edge
    pub fn is_undirected_hyper(&self) -> bool {
        self.as_model().is_undirected_hyper()
    }

    /// check edge is directed hyper edge
    pub fn is_directed_hyper(&self) -> bool {
        self.as_model().is_directed_hyper()
    }

    /// check edge is undirected or directed hyper edge
    pub fn is_hyper_edge(&self) -> bool {
        self.as_model().is_hyper_edge()
    }

    /// check configure support this edge type.
    pub fn is_support(&self, config: &GraphConfig) -> bool {
        use Edge::*;

        match self {
            Undirected { .. } => config.can_use_undirected_edge(),
            Directed { .. } => config.can_use_directed_edge(),
            UndirectedHyper { .. } => config.can_use_undirected_hyper_edge(),
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

    /// get edge at edge_id
    pub fn get_edge<B: ?Sized>(&self, edge_id: &B) -> Option<&Edge<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get(edge_id)
    }

    /// get edge as mutable at edge_id
    pub(crate) fn _get_edge_as_mut<B: ?Sized>(&mut self, edge_id: &B) -> Option<&mut Edge<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(edge_id)
    }

    /// get incidence node ids searched by edge_ids.
    pub fn get_incidence_node_ids_by_ids(&self, edge_ids: &[&Id]) -> Vec<&Id> {
        let mut result = Vec::new();
        for edge_id in edge_ids.iter() {
            match self.inner.get(edge_id) {
                None => {
                    continue;
                }
                Some(edge) => {
                    result.extend(edge.get_incidence_node_ids_as_ref());
                }
            }
        }

        result
    }

    /// inner store iter
    pub(crate) fn _iter<'a>(&'a self) -> Iter<'a, Id, Edge<Id>> {
        self.inner.iter()
    }

    /// to iterator for edge
    pub fn edge_iter<'a>(&'a self) -> EdgeIter<'a, Id> {
        EdgeIter::new(self)
    }

    /// to iterator for undirected edge
    pub fn undirected_edge_iter<'a>(&'a self) -> UndirectedEdgeIter<'a, Id> {
        UndirectedEdgeIter::new(self)
    }

    /// to iterator for directed edge
    pub fn directed_edge_iter<'a>(&'a self) -> DirectedEdgeIter<'a, Id> {
        DirectedEdgeIter::new(self)
    }

    /// to iterator for undirected or directed edge
    pub fn mixed_edge_iter<'a>(&'a self) -> MixedEdgeIter<'a, Id> {
        MixedEdgeIter::new(self)
    }

    /// to iterator for undirected hyper edge
    pub fn undirected_hyper_edge_iter<'a>(&'a self) -> UndirectedHyperEdgeIter<'a, Id> {
        UndirectedHyperEdgeIter::new(self)
    }

    /// to iterator for directed hyper edge
    pub fn directed_hyper_edge_iter<'a>(&'a self) -> DirectedHyperEdgeIter<'a, Id> {
        DirectedHyperEdgeIter::new(self)
    }

    /// to iterator for undirected or directed hyper edge
    pub fn mixed_hyper_edge_iter<'a>(&'a self) -> MixedHyperEdgeIter<'a, Id> {
        MixedHyperEdgeIter::new(self)
    }

    // ---
    // setter
    // ---

    /// insert edge
    pub fn insert_edge(&mut self, edge_id: Id, edge: Edge<Id>) -> Option<Edge<Id>> {
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

    // ---
    // delete
    // ---
    /// clear all edges
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// remove and get edge at edge_id
    pub fn remove<B: ?Sized>(&mut self, edge_id: &B) -> Option<Edge<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove(edge_id)
    }

    /// remove and get edge with edge_id
    pub fn remove_with_get_id<B: ?Sized>(&mut self, edge_id: &B) -> Option<(Id, Edge<Id>)>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove_entry(edge_id)
    }

    /// remove node_id and node's incidences from edge store
    /// return value is Vec<(node_id, edge_id>
    pub(crate) fn _remove_node_id_and_illegal_edge_with_collect(
        &mut self,
        deleted_node_id: &Id,
        deleted_node: Node<Id>,
    ) -> Vec<(Id, Id)> {
        let deleted_incidences = deleted_node.into_incidences();
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
                                let remove_first = deleted_node_id == &ids[0];
                                let remove_second = deleted_node_id == &ids[1];

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
                                let remove_source = deleted_node_id == source_id;
                                let remove_target = deleted_node_id == target_id;

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
                                ids.retain(|id| deleted_node_id != id);

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
                                source_ids.retain(|id| deleted_node_id != id);
                                target_ids.retain(|id| deleted_node_id != id);

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
