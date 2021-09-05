//! Module for graph structure as graph theory.

mod config;
mod edge;
pub mod error;
mod node;

pub mod iter {
    //! Module for iterator for graph items
    pub use crate::graph::edge::iter::*;
    pub use crate::graph::node::iter::*;
}

pub mod model {
    //! Module for model of item
    pub use crate::graph::edge::model::*;
    pub use crate::graph::node::model::*;
}

pub use config::*;
use edge::*;
use node::*;

use crate::graph::error::GraphError;
use crate::graph::iter::*;
use crate::util::Identity;
use std::borrow::Borrow;
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

    /// get graph configure
    pub fn get_config(&self) -> &GraphConfig {
        &self.config
    }

    // ---
    // node
    // ---

    /// get node at node_id
    pub fn get_node<'a, B: ?Sized>(&'a self, node_id: &B) -> Option<model::Node<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.nodes.get_node(node_id).map(|node| node.as_model())
    }

    /// to iterator for node
    pub fn node_iter<'a>(
        &'a self,
    ) -> NodeIter<'a, Id, impl Iterator<Item = (&'a Id, model::Node<'a, Id>)>> {
        self.nodes.iter()
    }

    // ---
    // edge
    // ---

    /// get edge at edge_id
    pub fn get_edge<'a, B: ?Sized>(&'a self, edge_id: &B) -> Option<model::Edge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges.get_edge(edge_id).map(|edge| edge.as_model())
    }

    /// get undirected edge at edge_id
    pub fn get_undirected_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::UndirectedEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_undirected_model())
            .flatten()
    }

    /// get directed edge at edge_id
    pub fn get_directed_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::DirectedEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_directed_model())
            .flatten()
    }

    /// get mixed edge at edge_id
    pub fn get_mixed_edge<'a, B: ?Sized>(&'a self, edge_id: &B) -> Option<model::MixedEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_mixed_model())
            .flatten()
    }

    /// get node grouping at edge_id as alias to getter undirected hyper edge at edge_id
    pub fn get_node_grouping<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::UndirectedHyperEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.get_undirected_hyper_edge(edge_id)
    }

    /// get undirected hyper edge at edge_id
    pub fn get_undirected_hyper_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::UndirectedHyperEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_undirected_hyper_model())
            .flatten()
    }

    /// get directed hyper edge at edge_id
    pub fn get_directed_hyper_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::DirectedHyperEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_directed_hyper_model())
            .flatten()
    }

    /// get mixed hyper edge at edge_id
    pub fn get_mixed_hyper_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::MixedHyperEdge<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_mixed_hyper_model())
            .flatten()
    }

    /// to iterator for edge
    pub fn edge_iter<'a>(
        &'a self,
    ) -> EdgeIter<'a, Id, impl Iterator<Item = (&'a Id, model::Edge<'a, Id>)>> {
        self.edges.edge_iter()
    }

    /// to iterator for undirected edge
    pub fn undirected_edge_iter<'a>(
        &'a self,
    ) -> UndirectedEdgeIter<'a, Id, impl Iterator<Item = (&'a Id, model::UndirectedEdge<'a, Id>)>>
    {
        self.edges.undirected_edge_iter()
    }

    /// to iterator for directed edge
    pub fn directed_edge_iter<'a>(
        &'a self,
    ) -> DirectedEdgeIter<'a, Id, impl Iterator<Item = (&'a Id, model::DirectedEdge<'a, Id>)>> {
        self.edges.directed_edge_iter()
    }

    /// to iterator for undirected of directed edge
    pub fn mixed_edge_iter<'a>(
        &'a self,
    ) -> MixedEdgeIter<'a, Id, impl Iterator<Item = (&'a Id, model::MixedEdge<'a, Id>)>> {
        self.edges.mixed_edge_iter()
    }

    /// to iterator for node grouping.
    /// alias iterator for undirected hyper edge
    pub fn node_gruoping_iter<'a>(
        &'a self,
    ) -> UndirectedHyperEdgeIter<
        'a,
        Id,
        impl Iterator<Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>)>,
    > {
        self.undirected_hyper_edge_iter()
    }

    /// to iterator for undirected hyper edge
    pub fn undirected_hyper_edge_iter<'a>(
        &'a self,
    ) -> UndirectedHyperEdgeIter<
        'a,
        Id,
        impl Iterator<Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>)>,
    > {
        self.edges.undirected_hyper_edge_iter()
    }

    /// to iterator for directed hyper edge
    pub fn directed_hyper_edge_iter<'a>(
        &'a self,
    ) -> DirectedHyperEdgeIter<
        'a,
        Id,
        impl Iterator<Item = (&'a Id, model::DirectedHyperEdge<'a, Id>)>,
    > {
        self.edges.directed_hyper_edge_iter()
    }

    /// to iterator for undirected or directed hyper edge
    pub fn mixed_hyper_edge_iter<'a>(
        &'a self,
    ) -> MixedHyperEdgeIter<'a, Id, impl Iterator<Item = (&'a Id, model::MixedHyperEdge<'a, Id>)>>
    {
        self.edges.mixed_hyper_edge_iter()
    }

    // ---
    // setter
    // ---

    /// Add node at the node_id, if not exist. If exist at the node_id, not replace.
    pub fn add_node(&mut self, node_id: Id) {
        self.nodes.set_as_node(node_id);
    }

    /// Add undirected edge without weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_undirected_edge(
        &mut self,
        edge_id: Id,
        node_id1: Id,
        node_id2: Id,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(edge_id, Edge::undirected(node_id1, node_id2))
    }

    /// Add directed edge without weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_directed_edge(
        &mut self,
        edge_id: Id,
        source_node_id: Id,
        target_node_id: Id,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(edge_id, Edge::directed(source_node_id, target_node_id))
    }

    /// Add undirected hyper edge as node group. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_node_grouping(
        &mut self,
        edge_id: Id,
        node_ids: Vec<Id>,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(edge_id, Edge::undirected_hyper(node_ids))
    }

    /// Add undirected hyper edge without weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_undirected_hyper_edge(
        &mut self,
        edge_id: Id,
        node_ids: Vec<Id>,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(edge_id, Edge::undirected_hyper(node_ids))
    }

    /// Add directed hyper edge without weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_directed_hyper_edge(
        &mut self,
        edge_id: Id,
        source_node_ids: Vec<Id>,
        target_node_ids: Vec<Id>,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            edge_id,
            Edge::directed_hyper(source_node_ids, target_node_ids),
        )
    }

    /// Add undirected edge with weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_undirected_edge_with_weight(
        &mut self,
        edge_id: Id,
        node_id1: Id,
        node_id2: Id,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            edge_id,
            Edge::undirected_with_weight(node_id1, node_id2, weight),
        )
    }

    /// Add directed edge with weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_directed_edge_with_weight(
        &mut self,
        edge_id: Id,
        source_node_id: Id,
        target_node_id: Id,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            edge_id,
            Edge::directed_with_weight(source_node_id, target_node_id, weight),
        )
    }

    /// Add undirected hyper edge with weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_undirected_hyper_edge_with_weight(
        &mut self,
        edge_id: Id,
        node_ids: Vec<Id>,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            edge_id,
            Edge::undirected_hyper_with_weight(node_ids, weight),
        )
    }

    /// Add directed hyper edge with weight. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    pub fn add_directed_hyper_edge_with_weight(
        &mut self,
        edge_id: Id,
        source_node_ids: Vec<Id>,
        target_node_ids: Vec<Id>,
        weight: i16,
    ) -> Result<(), GraphError<Id>> {
        self.add_edge(
            edge_id,
            Edge::directed_hyper_with_weight(source_node_ids, target_node_ids, weight),
        )
    }

    /// Add edge. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    fn add_edge(&mut self, edge_id: Id, edge: Edge<Id>) -> Result<(), GraphError<Id>> {
        let config: &GraphConfig = self.get_config();

        // check illegal edge
        if edge.has_illegal() {
            return Err(GraphError::IllegalEdge(edge_id, edge.into()));
        }

        // check or get flag
        if !edge.is_support(config) {
            return Err(GraphError::EdgeNotSupported(edge_id, edge.into()));
        }

        // If use node grouping, check intersect node on nodes of edge and nodes of other edges.
        // In other words, this software only supports one grouping hierarchy.
        //
        // i.e. Usually use subgraph in subgraph at other soft if the one contains another. But this soft cannot use.
        if config.can_use_node_group()
            && self.edges.has_intersect_group_without_same(&edge_id, &edge)
        {
            return Err(GraphError::NotSameNodeGroupHaveIntersect(
                edge_id,
                edge.into(),
            ));
        }

        // check same edge
        let can_multiple = if edge.is_edge() {
            config.can_multiple_edge()
        } else {
            config.can_multiple_hyper_edge()
        };
        let can_replace = config.can_replace_same_edge();
        let exist_same_edge: bool = if can_multiple || can_replace {
            false
        } else {
            self.edges.exist_same_edge(&edge)
        };
        if !can_multiple && exist_same_edge {
            return Err(GraphError::ExistSameEdge(edge_id, edge.into()));
        }

        if can_replace {
            // get same edge to remove
            // Vec<(node_id, edge_id)>
            let mut will_remove_node_id_and_edge_id: Vec<(Id, Id)> = if can_multiple {
                Vec::new()
            } else {
                self.edges.remove_by_same_edge_with_collect_removed(&edge)
            };
            if let Some(old_edge) = self.edges.pop_edge(&edge_id) {
                let edge_node_id_and_edge_id: Vec<(Id, Id)> = old_edge
                    .get_incidence_node_ids()
                    .into_iter()
                    .map(|node_id| (node_id, edge_id.clone()))
                    .collect();

                // replace true or not true, if exist_old_edge removed it when insert new edge.
                will_remove_node_id_and_edge_id.extend(edge_node_id_and_edge_id);
            }
            // remove mutable
            let will_remove_node_id_edge_id = will_remove_node_id_and_edge_id;

            // remove old incidence data for node
            if !will_remove_node_id_edge_id.is_empty() {
                self.nodes.remove_edges_by_ids(&will_remove_node_id_edge_id);
            }
        } else {
            // remove old incidence data at the id for node before add new edge
            if let Some(old_edge) = self.edges.pop_edge(&edge_id) {
                for node_id in old_edge.get_incidence_node_ids().iter() {
                    self.nodes.remove_edges_by_id(node_id, &edge_id);
                }
            }
        }

        //create incidence data from edge
        let incidences = edge.generate_incidences_without_check(&edge_id);

        // add edge (old edge deleted)
        let _ = self.edges.add_edge_with_pop_old(edge_id, edge);

        // add incidence data for node
        self.nodes.add_incidences_each_node(incidences);

        Ok(())
    }

    // ---
    // checker
    // ---

    // ---
    // delete
    // ---
    /// clear nodes and edges
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// clear all nodes
    pub fn clear_node(&mut self) {
        self.clear();
    }

    /// clear all edges
    pub fn clear_edge(&mut self) {
        self.nodes.clear_all_incidence();
        self.edges.clear();
    }

    /// delete node at node_id if exist with remove illegal edge.
    pub fn delete_node<B: ?Sized>(&mut self, node_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((pop_node_id, pop_node)) = self.nodes.pop_node_with_get_id(node_id) {
            let will_delete_incidences = self
                .edges
                ._remove_node_id_and_illegal_edge_with_collect(pop_node_id, pop_node);
            self.nodes.remove_edges_by_ids(&will_delete_incidences);
        }
    }

    /// delete nodes at node_id if exist with remove illegal edge.
    pub fn delete_nodes(&mut self, node_ids: &[Id]) {
        for node_id in node_ids.iter() {
            self.delete_node(node_id);
        }
    }

    /// delete node at the node_id with incidence edges
    pub fn delete_node_with_edge<B: ?Sized>(&mut self, node_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some(pop_node) = self.nodes.pop_node(node_id) {
            let edge_ids = pop_node.incidences_into_edge_ids();
            self.delete_edges(&edge_ids);
        }
    }

    /// delete node at the node_id with incidence edges
    pub fn delete_nodes_with_edge(&mut self, node_ids: &[Id]) {
        for node_id in node_ids {
            self.delete_node_with_edge(node_id);
        }
    }

    /// delete edge without delete node
    pub fn delete_edge<B: ?Sized>(&mut self, edge_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some(pop_edge) = self.edges.pop_edge(edge_id) {
            let will_delete_node_id = pop_edge.incidence_into_node_ids();
            for delete_node_id in will_delete_node_id {
                self.nodes
                    .remove_edges_by_id(delete_node_id.borrow(), &edge_id);
            }
        }
    }

    /// delete edges without delete node
    pub fn delete_edges(&mut self, edge_ids: &[Id]) {
        for edge_id in edge_ids {
            self.delete_edge(edge_id);
        }
    }

    /// delete edge with incidence node
    pub fn delete_edge_with_node<B: ?Sized>(&mut self, edge_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some(pop_edge) = self.edges.pop_edge(edge_id) {
            let will_delete_incidences = pop_edge.incidence_into_node_ids();
            self.delete_nodes(&will_delete_incidences);
        }
    }

    /// delete edges with incidence node
    pub fn delete_edges_with_node(&mut self, edge_ids: &[Id]) {
        let mut will_delete_nodes: Vec<Id> = Vec::new();
        for edge_id in edge_ids.iter() {
            if let Some(pop_edge) = self.edges.pop_edge(edge_id) {
                will_delete_nodes.extend(pop_edge.incidence_into_node_ids());
            }
        }
        // remove node_ids to unique
        will_delete_nodes.sort();
        will_delete_nodes.dedup();

        self.delete_nodes(&will_delete_nodes);
    }
}
