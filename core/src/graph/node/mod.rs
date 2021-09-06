//! Module for edge for incidence node and it's store

mod incidence;
pub mod iter;
pub mod model;

use crate::util::Identity;
pub use incidence::*;
use iter::*;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;

/// node structure for graph
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node<Id: Identity> {
    weight: i16,
    incidences: Vec<Incidence<Id>>,
}

impl<Id: Identity> fmt::Display for Node<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model = self.as_model();
        fmt::Display::fmt(&model, f)
    }
}

impl<Id: Identity> Default for Node<Id> {
    fn default() -> Self {
        Self {
            weight: 1,
            incidences: vec![],
        }
    }
}

impl<Id: Identity> Node<Id> {
    // ---
    // constructor
    // ---

    /// create node structure
    pub fn create() -> Self {
        Self::create_with_weight(1)
    }

    /// create node structure with weight
    pub fn create_with_weight(weight: i16) -> Self {
        Self {
            weight: weight,
            incidences: vec![],
        }
    }

    /// create model as node
    #[inline]
    pub fn as_model<'a>(&'a self) -> model::Node<'a, Id> {
        model::Node::_create(&self.weight, &self.incidences)
    }

    // ---
    // getter
    // ---

    /// get weight for the node
    pub fn get_weight(&self) -> &i16 {
        &self.weight
    }

    /// get incidences list for the node
    pub fn get_incidences(&self) -> &[Incidence<Id>] {
        &self.incidences
    }

    /// get edge_ids from the node's incidenes
    pub fn incidences_into_edge_ids(self) -> Vec<Id> {
        self.incidences
            .into_iter()
            .map(|incidence| match incidence {
                Incidence::Undirected { edge_id }
                | Incidence::DirectedSource { edge_id }
                | Incidence::DirectedTarget { edge_id }
                | Incidence::UndirectedHyper { edge_id }
                | Incidence::DirectedHyperSource { edge_id }
                | Incidence::DirectedHyperTarget { edge_id } => edge_id,
            })
            .collect()
    }

    /// into incidence list
    pub fn into_incidences(self) -> Vec<Incidence<Id>> {
        self.incidences
    }

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---

    // ---
    // delete
    // ---

    /// delete all incidence
    pub fn clear_incidences(&mut self) {
        self.incidences.clear();
    }

    /// delete incidence with same edge id and get deleted count
    pub fn remove_incidence_by_id<B: ?Sized>(&mut self, edge_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.incidences.retain(|incidence| {
            // check as borrowed because of no clone.
            if incidence.get_edge_id().borrow() != edge_id {
                // retain
                true
            } else {
                // to delete
                false
            }
        });
    }
}

/// Store structure for node.
#[derive(Eq, PartialEq, Clone)]
pub struct NodeStore<Id: Identity> {
    inner: BTreeMap<Id, Node<Id>>,
}

impl<Id: Identity + fmt::Debug> fmt::Debug for NodeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

impl<Id: Identity> fmt::Display for NodeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut is_first = true;
        f.write_str("{")?;
        for (node_id, node) in self.inner.iter() {
            if is_first {
                f.write_fmt(format_args!("{:?}:{}", node_id, node))?;
            } else {
                f.write_fmt(format_args!(", {:?}:{}", node_id, node))?;
            }
            is_first = false;
        }
        f.write_str("}")
    }
}

impl<Id: Identity> NodeStore<Id> {
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

    /// get node at node_id
    pub fn get_node<B: ?Sized>(&self, node_id: &B) -> Option<&Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get(node_id)
    }

    /// to iterator for node
    pub fn iter<'a>(
        &'a self,
    ) -> NodeIter<'a, Id, impl Iterator<Item = (&'a Id, model::Node<'a, Id>)>> {
        let iter = self
            .inner
            .iter()
            .map(|(node_id, node)| (node_id, node.as_model()));

        NodeIter::new(iter)
    }

    // ---
    // setter
    // ---
    /// clear all nodes
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// clear all nodes
    pub fn clear_all_incidence(&mut self) {
        for node in self.inner.values_mut() {
            node.clear_incidences();
        }
    }

    /// Add node if not exist. If exist, not replace.
    pub fn set_as_node(&mut self, node_id: Id) {
        let entry = self.inner.entry(node_id);
        entry.or_insert_with(|| Node::create());
    }

    /// add incidence for the node
    pub fn add_incidence(&mut self, node_id: Id, incidence: Incidence<Id>) {
        let entry_node = self.inner.entry(node_id).or_insert_with(|| Node::create());
        entry_node.incidences.push(incidence);
    }

    /// add incidence for each node
    pub fn add_incidences_each_node(&mut self, node_incidences: Vec<(Id, Incidence<Id>)>) {
        for (node_id, incidences) in node_incidences.into_iter() {
            self.add_incidence(node_id, incidences);
        }
    }

    // ---
    // checker
    // ---

    // ---
    // delete
    // ---

    /// remove and get node at node_id
    pub fn pop_node<B: ?Sized>(&mut self, node_id: &B) -> Option<Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove(node_id)
    }

    /// remove and get node at node_id
    pub fn pop_node_with_get_id<B: ?Sized>(&mut self, node_id: &B) -> Option<(Id, Node<Id>)>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove_entry(node_id)
    }

    /// Remove incidence edge whose edge id is in specified.
    pub fn remove_edges_by_id<B: ?Sized>(&mut self, node_id: &B, edge_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some(node) = self.inner.get_mut(node_id) {
            node.remove_incidence_by_id(edge_id);
        }
    }

    /// Remove incidence edge whose edge ids is in specified.
    pub fn remove_edges_by_ids(&mut self, removed_node_id_edge_id: &[(Id, Id)]) {
        for (node_id, edge_id) in removed_node_id_edge_id.iter() {
            self.remove_edges_by_id(node_id, edge_id);
        }
    }
}