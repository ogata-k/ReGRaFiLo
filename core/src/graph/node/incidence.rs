//! Module for incidence to node

use crate::util::Identity;

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

impl<Id: Identity> fmt::Display for Incidence<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Incidence::*;

        match self {
            Undirected { edge_id } => f.write_fmt(format_args!(
                "{{type: (Undirected, Source/Target), edge_id: {:?}}}",
                edge_id
            )),
            DirectedSource { edge_id } => f.write_fmt(format_args!(
                "{{type: (Directed, Source), edge_id: {:?}}}",
                edge_id
            )),
            DirectedTarget { edge_id } => f.write_fmt(format_args!(
                "{{type: (Directed, Target), edge_id: {:?}}}",
                edge_id
            )),
            UndirectedHyper { edge_id } => f.write_fmt(format_args!(
                "{{type: (UndirectedHyper, Source/Target), edge_id: {:?}}}",
                edge_id
            )),
            DirectedHyperSource { edge_id } => f.write_fmt(format_args!(
                "{{type: (DirectedHyper, Source), edge_id: {:?}}}",
                edge_id
            )),
            DirectedHyperTarget { edge_id } => f.write_fmt(format_args!(
                "{{type: (DirectedHyper, Target), edge_id: {:?}}}",
                edge_id
            )),
        }
    }
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

    // ---
    // delete
    // ---
}
