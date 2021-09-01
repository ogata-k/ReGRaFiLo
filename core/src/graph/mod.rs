//! Module for graph structure as graph theory.

mod config;
mod edge;
mod error;
mod node;

pub use config::*;
use edge::*;
pub use error::*;
use node::*;

use crate::util::Identity;
use std::fmt;

/// graph without laypout
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Graph<Id: Identity> {
    config: GraphConfig,
    nodes: NodeStore<Id>,
    edges: EdgeStore<Id>,
}

impl<Id: Identity> fmt::Display for Graph<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}{{config: {}, nodes: {}, edges: {}}}",
            self.config.get_type(),
            self.config,
            self.nodes,
            self.edges
        ))
    }
}

impl<Id: Identity> Graph<Id> {
    // ---
    // constructor
    // ---

    /// construct graph with use the config
    pub fn create_by_config(config: GraphConfig) -> Self {
        Self {
            config,
            nodes: NodeStore::create(),
            edges: EdgeStore::create(),
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
    pub fn create_as_undirected_hyper_graph(can_multiple_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::undirected_hyper_graph(can_multiple_edge))
    }

    /// construtor for Directed Hyper Graph
    pub fn create_as_directed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::directed_hyper_graph(can_multiple_hyper_edge))
    }

    /// construtor for Mixed Hyper Graph
    pub fn create_as_mixed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::mixed_hyper_graph(can_multiple_hyper_edge))
    }

    // ---
    // getter
    // ---

    pub fn get_config(&self) -> &GraphConfig {
        &self.config
    }

    // ---
    // setter
    // ---

    /// Add node at the node_id, if not exist. If exist at the node_id, not replace.
    pub fn add_node(&mut self, node_id: Id) {
        self.nodes.set_as_node(node_id);
    }

    /// Add undirected edge without weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_undirected_edge(
        &mut self,
        replace: bool,
        edge_id: Id,
        node_id1: Id,
        node_id2: Id,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(replace, edge_id, Edge::undirected(node_id1, node_id2))
    }

    /// Add directed edge without weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_directed_edge(
        &mut self,
        replace: bool,
        edge_id: Id,
        source_node_id: Id,
        target_node_id: Id,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            replace,
            edge_id,
            Edge::directed(source_node_id, target_node_id),
        )
    }

    /// Add undirected hyper edge without weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_undirected_hyper_edge(
        &mut self,
        replace: bool,
        edge_id: Id,
        node_ids: Vec<Id>,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(replace, edge_id, Edge::undirected_hyper(node_ids))
    }

    /// Add directed hyper edge without weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_directed_hyper_edge(
        &mut self,
        replace: bool,
        edge_id: Id,
        source_node_ids: Vec<Id>,
        target_node_ids: Vec<Id>,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            replace,
            edge_id,
            Edge::directed_hyper(source_node_ids, target_node_ids),
        )
    }

    /// Add undirected edge with weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_undirected_edge_with_weight(
        &mut self,
        replace: bool,
        edge_id: Id,
        node_id1: Id,
        node_id2: Id,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            replace,
            edge_id,
            Edge::undirected_with_weight(node_id1, node_id2, weight),
        )
    }

    /// Add directed edge with weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_directed_edge_with_weight(
        &mut self,
        replace: bool,
        edge_id: Id,
        source_node_id: Id,
        target_node_id: Id,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            replace,
            edge_id,
            Edge::directed_with_weight(source_node_id, target_node_id, weight),
        )
    }

    /// Add undirected hyper edge with weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_undirected_hyper_edge_with_weight(
        &mut self,
        replace: bool,
        edge_id: Id,
        node_ids: Vec<Id>,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            replace,
            edge_id,
            Edge::undirected_hyper_with_weight(node_ids, weight),
        )
    }

    /// Add directed hyper edge with weight. If exist at the edge_id, not replace when replace is false.
    pub fn add_directed_hyper_edge_with_weight(
        &mut self,
        replace: bool,
        edge_id: Id,
        source_node_ids: Vec<Id>,
        target_node_ids: Vec<Id>,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            replace,
            edge_id,
            Edge::directed_hyper_with_weight(source_node_ids, target_node_ids, weight),
        )
    }

    /// Add edge. If exist at the edge_id, not replace when replace is false.
    fn add_edge(
        &mut self,
        replace: bool,
        edge_id: Id,
        edge: Edge<Id>,
    ) -> Result<(), GraphError<Id>> {
        let config = self.get_config();

        // check illegal edge
        if edge.has_illegal() {
            return Err(GraphError::IllegalEdge(edge_id));
        }

        // check or get flag
        if !edge.is_support(config) {
            return Err(GraphError::EdgeNotSupported(edge_id, edge));
        }

        let exist_edge_id = self.edges.has_edge_id(&edge_id);
        if !replace && exist_edge_id {
            return Err(GraphError::EdgeAlreadyExist(edge_id));
        }

        let can_multiple = if edge.is_edge() {
            config.can_multiple_edge()
        } else {
            config.can_multiple_hyper_edge()
        };

        // remove edge
        let mut removed_edge_ids: Vec<Id> = if can_multiple {
            Vec::new()
        } else {
            self.edges.remove_by_same_edge_with_collect_removed(&edge)
        };
        if exist_edge_id {
            // replace true or not true, if exist_old_edge removed it when insert new edge.
            removed_edge_ids.push(edge_id.clone());
        }
        // remove mutable
        let removed_edge_ids = removed_edge_ids;

        // create incidence data
        let incidences = self._generate_edge_incidences(&edge_id, &edge);

        // add edge (and old edge delete)
        let _ = self.edges.add_edge_with_pop_old(edge_id, edge);

        // remove incidence data for node
        if !removed_edge_ids.is_empty() {
            self.nodes.remove_edges_by_ids(&removed_edge_ids);
        }

        // add incidence data for node
        self.nodes.add_incidences_each_node(incidences);

        Ok(())
    }

    /// the helper function. generate incidence data
    fn _generate_edge_incidences(&self, edge_id: &Id, edge: &Edge<Id>) -> Vec<(Id, Incidence<Id>)> {
        let mut result = Vec::new();
        match edge {
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

    // ---
    // checker
    // ---

    // ---
    // delete
    // ---
}
