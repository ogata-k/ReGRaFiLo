//! Module for graph structure as graph theory.

mod config;
mod edge;
mod node;

pub use config::*;
pub(crate) use edge::*;
pub(crate) use node::*;

use crate::util::Identity;

/// graph without laypout
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Graph<Id: Identity> {
    config: GraphConfig,
    nodes: NodeStore<Id>,
    edges: EdgeStore<Id>,
}

impl<Id: Identity> Graph<Id> {
    // ---
    // constructor
    // ---

    /// construct graph with use the config
    pub fn create_by_config(config: GraphConfig) -> Self {
        Self {
            config,
            nodes: Default::default(),
            edges: Default::default(),
        }
    }

    /// construtor for Graph
    pub fn create_as_undirected_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::create_by_config(GraphConfig::undirected_graph(
            can_multiple_edge,
            use_grouping,
        ))
    }

    /// construtor for Directed Graph
    pub fn create_as_directed_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::create_by_config(GraphConfig::directed_graph(can_multiple_edge, use_grouping))
    }

    /// construtor for Mixed Graph
    pub fn create_as_mixed_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::create_by_config(GraphConfig::mixed_graph(can_multiple_edge, use_grouping))
    }

    /// construtor for Hyper Graph
    pub fn create_as_hyper_graph(can_multiple_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::hyper_graph(can_multiple_edge))
    }

    /// construtor for Directed Hyper Graph
    pub fn create_as_directed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::directed_hyper_graph(can_multiple_hyper_edge))
    }

    /// construtor for Mixed Hyper Graph
    pub fn create_as_mixed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::mixed_hyper_graph(can_multiple_hyper_edge))
    }

    // TODO 必要な実装を必要な時に
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
