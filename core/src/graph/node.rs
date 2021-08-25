//! Module for edge for incidence node and it's store

use crate::graph::GraphConfig;
use crate::util::Identity;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;

/// incidence types to node
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Incidence<Id: Identity> {
    /// A state in which an undirected edge is connected to a node.
    Undirected { edge_id: Id },

    /// A state in which an directed edge is connected to a node as source node.
    DirectedSource { edge_id: Id },

    /// A state in which an directed edge is connected to a node as target node.
    DirectedTarget { edge_id: Id },

    /// A state in which an undirected hyper edge is connected to a node.
    UndirectedHyper { edge_id: Id },

    /// A state in which an directed edge is connected to a node as source node.
    DirectedHyperSource { edge_id: Id },

    /// A state in which an directed edge is connected to a node as target node.
    DirectedHyperTarget { edge_id: Id },
}

impl<Id: Identity> Incidence<Id> {
    // ---
    // constructor
    // ---

    /// constructor for undirected edge's incidence
    pub fn undirected(edge_id: Id) -> Self {
        Self::Undirected { edge_id: edge_id }
    }

    /// constructor for directed edge's incidence for source node
    pub fn directed_source(edge_id: Id) -> Self {
        Self::DirectedSource { edge_id: edge_id }
    }

    /// constructor for directed edge's incidence for target node
    pub fn directed_target(edge_id: Id) -> Self {
        Self::DirectedTarget { edge_id: edge_id }
    }

    /// constructor for undirected hyper edge's incidence
    pub fn undirected_hyper(edge_id: Id) -> Self {
        Self::UndirectedHyper { edge_id: edge_id }
    }

    /// constructor for directed hyper edge's incidence for source node
    pub fn directed_hyper_source(edge_id: Id) -> Self {
        Self::DirectedHyperSource { edge_id: edge_id }
    }

    /// constructor for directed hyper edge's incidence for target node
    pub fn directed_hyper_target(edge_id: Id) -> Self {
        Self::DirectedHyperTarget { edge_id: edge_id }
    }

    // ---
    // getter
    // ---

    /// get edge_id for the incidence edge
    pub fn get_edge_id(&self) -> &Id {
        use Incidence::*;

        match self {
            Undirected { edge_id, .. }
            | DirectedSource { edge_id, .. }
            | DirectedTarget { edge_id, .. }
            | UndirectedHyper { edge_id, .. }
            | DirectedHyperSource { edge_id, .. }
            | DirectedHyperTarget { edge_id, .. } => edge_id,
        }
    }

    // ---
    // checker
    // ---

    /// check the incidence edge is undirected edge
    pub fn is_undirected(&self) -> bool {
        if let Self::Undirected { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed edge which connect to node as source node
    pub fn is_directed_source(&self) -> bool {
        if let Self::DirectedSource { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed edge which connect to node as target node
    pub fn is_directed_target(&self) -> bool {
        if let Self::DirectedTarget { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed edge which connect to node as source or target node
    pub fn is_directed(&self) -> bool {
        match self {
            Self::DirectedSource { .. } | Self::DirectedTarget { .. } => true,
            _ => false,
        }
    }

    /// check the incidence edge is undirected hyper edge
    pub fn is_undirected_hyper(&self) -> bool {
        if let Self::UndirectedHyper { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed hyper edge which connect to node as source node
    pub fn is_directed_hyper_source(&self) -> bool {
        if let Self::DirectedHyperSource { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed  hyper edge which connect to node as target node
    pub fn is_directed_hyper_target(&self) -> bool {
        if let Self::DirectedHyperTarget { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed hyper edge which connect to node as source or target node
    pub fn is_directed_hyper(&self) -> bool {
        match self {
            Self::DirectedHyperSource { .. } | Self::DirectedHyperTarget { .. } => true,
            _ => false,
        }
    }

    /// check configure support this edge type.
    pub fn is_support(&self, config: &GraphConfig) -> bool {
        use Incidence::*;

        match self {
            Undirected { .. } => config.is_undirected_graph() || config.is_mixed_graph(),
            DirectedSource { .. } | DirectedTarget { .. } => {
                config.is_directed_graph() || config.is_mixed_graph()
            }
            UndirectedHyper { .. } => {
                config.is_hyper_graph() || config.is_mixed_hyper_graph() || config.has_group()
            }
            DirectedHyperSource { .. } | DirectedHyperTarget { .. } => {
                config.is_hyper_graph() || config.is_mixed_hyper_graph()
            }
        }
    }

    // ---
    // delete
    // ---
}

/// node structure for graph
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node<Id: Identity> {
    weight: i16,
    incidences: Vec<Incidence<Id>>,
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

    // ---
    // getter
    // ---

    /// get incidences list for the node
    pub fn get_incidences(&self) -> &[Incidence<Id>] {
        &self.incidences
    }

    /// get weight for the node
    pub fn get_weight(&self) -> &i16 {
        &self.weight
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
    pub fn clear_incidences(&mut self) -> usize {
        let deleted = self.incidences.len();
        self.incidences.clear();

        deleted
    }

    /// delete incidence with same edge_id and get deleted count
    pub fn remove_incidence_by_id<B: ?Sized>(&mut self, edge_id: &B) -> usize
    where
        Id: Borrow<B>,
        B: Identity,
    {
        let mut deleted = 0;
        self.incidences.retain(|incidence| {
            // check as borrowed because of no clone.
            if incidence.get_edge_id().borrow() != edge_id {
                // retain
                true
            } else {
                // to delete
                deleted += 1;
                false
            }
        });

        deleted
    }
}

/// Store structure for node.
#[derive(Eq, PartialEq, Clone)]
pub struct NodeStore<Id: Identity> {
    inner: BTreeMap<Id, Node<Id>>,
}

impl<Id: Identity> Default for NodeStore<Id> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<Id: Identity + fmt::Debug> fmt::Debug for NodeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

impl<Id: Identity> NodeStore<Id> {
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
