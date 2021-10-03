//! Module for graph structure as graph theory.

mod as_model;
mod config;
pub mod error;
pub mod iter;
pub mod model;
mod store;

pub use config::*;

use crate::graph::as_model::{AsEdgeModel, AsNodeModel};
use crate::graph::error::GraphError;
use crate::graph::iter::{
    DirectedEdgeIter, DirectedHyperEdgeIter, EdgeIter, GroupChildNodeIter, GroupNodeIter,
    MixedEdgeIter, MixedHyperEdgeIter, NodeIter, UndirectedEdgeIter, UndirectedHyperEdgeIter,
    VertexNodeIter,
};
use crate::graph::model::{EdgeModel, NodeModel};
use crate::graph::store::{Edge, EdgeStore, Incidence, Node, NodeStore};
use crate::util::{Identity, Weight};

use std::borrow::Borrow;
use std::collections::btree_map::Entry;
use std::fmt;

/// helper for handling existed node
pub trait NodeExistedResultExt<NodeId: Identity, EdgeId: Identity> {
    /// If old node exist, then return Err(GraphError::AlreadyNodeExist). Others same.
    fn old_node_exist_to_error(self) -> Result<(), GraphError<NodeId, EdgeId>>;
    /// If old node exist, then return Ok(Some(model::Node))
    fn with_old_node_model<'a>(
        self,
        graph: &'a Graph<NodeId, EdgeId>,
    ) -> Result<Option<(NodeId, model::Node<'a, NodeId, EdgeId>)>, GraphError<NodeId, EdgeId>>;
    /// call callback when old node exist
    fn call_if_old_node_exist<F>(self, callback: F) -> Result<(), GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(NodeId) -> Result<(), GraphError<NodeId, EdgeId>>;
    /// map if inserted node. If op's arg is None then create just time else already exist.
    fn map_if_node_inserted<U, F>(self, op: F) -> Result<U, GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(Option<NodeId>) -> Result<U, GraphError<NodeId, EdgeId>>;
}

/// helper for handling existed edge
pub trait EdgeExistedResultExt<NodeId: Identity, EdgeId: Identity> {
    /// If old edge exist, then return Err(GraphError::AlreadyEdgeExist). Others same.
    fn old_edge_exist_to_error(self) -> Result<(), GraphError<NodeId, EdgeId>>;
    /// If old edge exist, then return Ok(Some(model::Edge))
    fn with_old_edge_model<'a>(
        self,
        graph: &'a Graph<NodeId, EdgeId>,
    ) -> Result<Option<(EdgeId, model::Edge<'a, NodeId, EdgeId>)>, GraphError<NodeId, EdgeId>>;
    /// call callback when old edge exist
    fn call_if_old_edge_exist<F>(self, callback: F) -> Result<(), GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(EdgeId) -> Result<(), GraphError<NodeId, EdgeId>>;
    /// map if inserted edge. If op's arg is None then create just time else already exist.
    fn map_if_edge_inserted<U, F>(self, op: F) -> Result<U, GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(Option<EdgeId>) -> Result<U, GraphError<NodeId, EdgeId>>;
}

/// If this value is Ok(Some(id)), then old graph item exist at the id.
pub type GraphItemExistedResult<Id, NodeId, EdgeId> =
    Result<Option<Id>, GraphError<NodeId, EdgeId>>;

impl<NodeId: Identity, EdgeId: Identity> NodeExistedResultExt<NodeId, EdgeId>
    for GraphItemExistedResult<NodeId, NodeId, EdgeId>
{
    fn with_old_node_model<'a>(
        self,
        graph: &'a Graph<NodeId, EdgeId>,
    ) -> Result<Option<(NodeId, model::Node<'a, NodeId, EdgeId>)>, GraphError<NodeId, EdgeId>> {
        self.map_if_node_inserted(|node_id| match node_id {
            None => Ok(None),
            Some(_node_id) => {
                let node = graph.get_node(&_node_id).expect(&format!(
                    "Already exist old node at node_id {:?}. Why not exist?",
                    _node_id
                ));
                Ok(Some((_node_id, node)))
            }
        })
    }

    fn old_node_exist_to_error(self) -> Result<(), GraphError<NodeId, EdgeId>> {
        self.and_then(old_node_exist_to_error)
    }

    fn call_if_old_node_exist<F>(self, callback: F) -> Result<(), GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(NodeId) -> Result<(), GraphError<NodeId, EdgeId>>,
    {
        match self {
            Ok(None) => Ok(()),
            Ok(Some(node_id)) => callback(node_id),
            Err(e) => Err(e),
        }
    }

    fn map_if_node_inserted<U, F>(self, op: F) -> Result<U, GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(Option<NodeId>) -> Result<U, GraphError<NodeId, EdgeId>>,
    {
        match self {
            Ok(s) => op(s),
            Err(e) => Err(e),
        }
    }
}

impl<NodeId: Identity, EdgeId: Identity> EdgeExistedResultExt<NodeId, EdgeId>
    for GraphItemExistedResult<EdgeId, NodeId, EdgeId>
{
    fn old_edge_exist_to_error(self) -> Result<(), GraphError<NodeId, EdgeId>> {
        self.and_then(old_edge_exist_to_error)
    }

    fn with_old_edge_model<'a>(
        self,
        graph: &'a Graph<NodeId, EdgeId>,
    ) -> Result<Option<(EdgeId, model::Edge<'a, NodeId, EdgeId>)>, GraphError<NodeId, EdgeId>> {
        self.map_if_edge_inserted(|edge_id| match edge_id {
            None => Ok(None),
            Some(_edge_id) => {
                let edge = graph.get_edge(&_edge_id).expect(&format!(
                    "Already exist old edge at edge_id {:?}. Why not exist?",
                    _edge_id
                ));
                Ok(Some((_edge_id, edge)))
            }
        })
    }

    fn call_if_old_edge_exist<F>(self, callback: F) -> Result<(), GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(EdgeId) -> Result<(), GraphError<NodeId, EdgeId>>,
    {
        match self {
            Ok(None) => Ok(()),
            Ok(Some(edge_id)) => callback(edge_id),
            Err(e) => Err(e),
        }
    }

    fn map_if_edge_inserted<U, F>(self, op: F) -> Result<U, GraphError<NodeId, EdgeId>>
    where
        F: FnOnce(Option<EdgeId>) -> Result<U, GraphError<NodeId, EdgeId>>,
    {
        match self {
            Ok(s) => op(s),
            Err(e) => Err(e),
        }
    }
}

/// helper for create already exist node error.
///
/// e.g.
/// let result: Result<Option<Id>, GraphError>;  // result for create new node
/// result.and_then(old_node_exist_to_error)?;
pub fn old_node_exist_to_error<NodeId: Identity, EdgeId: Identity>(
    old_node_exist: Option<NodeId>,
) -> Result<(), GraphError<NodeId, EdgeId>> {
    match old_node_exist {
        Some(node_id) => {
            // old node exist
            Err(GraphError::AlreadyExistNodeAtId(node_id))
        }
        None => Ok(()),
    }
}

/// helper for create already exist edge error.
///
/// e.g.
/// let result: Result<Option<Id>, GraphError>;  // result for create new edge
/// result.and_then(old_edge_exist_to_error)?;
pub fn old_edge_exist_to_error<NodeId: Identity, EdgeId: Identity>(
    old_edge_exist: Option<EdgeId>,
) -> Result<(), GraphError<NodeId, EdgeId>> {
    match old_edge_exist {
        Some(edge_id) => {
            // old edge exist
            Err(GraphError::AlreadyExistEdgeAtId(edge_id))
        }
        None => Ok(()),
    }
}

/// graph without layout
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Graph<NodeId: Identity, EdgeId: Identity> {
    config: GraphConfig,
    nodes: NodeStore<NodeId, EdgeId>,
    edges: EdgeStore<NodeId, EdgeId>,
}

impl<NodeId: Identity, EdgeId: Identity> fmt::Display for Graph<NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}{{config: {}, nodes: {}, edges: {}}}",
            self.config.get_graph_type(),
            self.config,
            self.nodes,
            self.edges
        ))
    }
}

impl<NodeId: Identity, EdgeId: Identity> Graph<NodeId, EdgeId> {
    // ---
    // constructor
    // ---

    /// construct graph with use the config
    pub fn create(config: GraphConfig) -> Self {
        Self {
            config,
            nodes: NodeStore::create(),
            edges: EdgeStore::create(),
        }
    }

    /// Generate incidences data from the edge with assume that we already check support edge.
    fn generate_incidences_without_check(
        &self,
        edge_id: &EdgeId,
        edge: &store::Edge<NodeId, EdgeId>,
    ) -> Vec<(NodeId, store::Incidence<NodeId, EdgeId>)> {
        let mut result = Vec::new();
        // No check support incidence with config
        match &edge {
            store::Edge::Undirected { ids, .. } => {
                for node_id in ids {
                    result.push((
                        node_id.clone(),
                        store::Incidence::undirected(edge_id.clone()),
                    ));
                }
            }
            store::Edge::Directed {
                source_id,
                target_id,
                ..
            } => {
                result.push((
                    source_id.clone(),
                    store::Incidence::directed_source(edge_id.clone()),
                ));
                result.push((
                    target_id.clone(),
                    store::Incidence::directed_target(edge_id.clone()),
                ));
            }
            store::Edge::UndirectedHyper { ids, .. } => {
                for node_id in ids {
                    result.push((
                        node_id.clone(),
                        store::Incidence::undirected_hyper(edge_id.clone()),
                    ));
                }
            }
            store::Edge::DirectedHyper {
                source_ids,
                target_ids,
                ..
            } => {
                for source_id in source_ids {
                    result.push((
                        source_id.clone(),
                        store::Incidence::directed_hyper_source(edge_id.clone()),
                    ));
                }

                for target_id in target_ids {
                    result.push((
                        target_id.clone(),
                        store::Incidence::directed_hyper_target(edge_id.clone()),
                    ));
                }
            }
        }

        result
    }

    // ---
    // getter
    // ---

    /// get graph configure
    pub fn get_config(&self) -> &GraphConfig {
        &self.config
    }

    // ---
    // getter Node
    // ---

    /// get incidence node ids to edges which incidence to the node which is between the node_id and top parent and get parent node ids
    fn get_incidence_node_ids_from_self_and_parent_node_ids(
        &self,
        node_id: &NodeId,
    ) -> (Vec<&NodeId>, Vec<&NodeId>) {
        let (incidence_edge_ids_from_self, parent_node_ids) = self
            .nodes
            .get_incidence_edge_ids_from_the_node_id_and_parent_ids(node_id);
        let incidence_node_ids_from_self = self
            .edges
            .get_incidence_node_ids_by_ids(incidence_edge_ids_from_self.as_slice());

        (incidence_node_ids_from_self, parent_node_ids)
    }

    /// get node at node_id
    pub fn get_node<'a, B: ?Sized>(&'a self, node_id: &B) -> Option<model::Node<'a, NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.nodes.get_node(node_id).map(|node| node.as_model())
    }

    /// get node point at node_id
    pub fn get_vertex_node<'a, B: ?Sized>(
        &'a self,
        node_id: &B,
    ) -> Option<model::VertexNode<'a, NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.nodes
            .get_node(node_id)
            .map(|node| node.as_vertex_model())
            .flatten()
    }

    /// get node group at node_id
    pub fn get_group_node<'a, B: ?Sized>(
        &'a self,
        node_id: &B,
    ) -> Option<model::GroupNode<'a, NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.nodes
            .get_node(node_id)
            .map(|node| node.as_group_model())
            .flatten()
    }

    /// to iterator for node
    pub fn node_iter<'a>(&'a self) -> NodeIter<'a, NodeId, EdgeId> {
        NodeIter::new(&self.nodes)
    }

    /// to iterator for node point
    pub fn vertex_node_iter<'a>(&'a self) -> VertexNodeIter<'a, NodeId, EdgeId> {
        VertexNodeIter::new(&self.nodes)
    }

    /// to iterator for node group
    pub fn group_node_iter<'a>(&'a self) -> GroupNodeIter<'a, NodeId, EdgeId> {
        GroupNodeIter::new(&self.nodes)
    }

    /// to iterator for grouping child nodes
    pub fn group_child_node_iter<'a, B: ?Sized>(
        &'a self,
        group_id: Option<&'a B>,
    ) -> GroupChildNodeIter<'a, NodeId, EdgeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        GroupChildNodeIter::new(group_id, &self.nodes)
    }

    // ---
    // getter Edge
    // ---

    /// get edge ids which have same incidence nodes
    fn get_same_edge_ids(&self, edge: &Edge<NodeId, EdgeId>) -> Vec<EdgeId> {
        match edge.get_incidence_node_ids_as_ref().first() {
            None => vec![],
            Some(node_id) => {
                return match self.nodes.get_node(node_id) {
                    None => Vec::new(),
                    Some(node) => {
                        let will_check_edge_ids: Vec<&EdgeId> = node
                            .get_incidences()
                            .iter()
                            .map(|incidence| incidence.get_edge_id())
                            .collect();
                        let mut result = Vec::new();

                        for edge_id in will_check_edge_ids.iter() {
                            if let Some(other_edge) = self.edges.get_edge(*edge_id) {
                                if edge.is_equal_to_without_weight(other_edge) {
                                    result.push((*edge_id).clone());
                                }
                            }
                        }

                        result
                    }
                };
            }
        }
    }

    /// get edge at edge_id
    pub fn get_edge<'a, B: ?Sized>(&'a self, edge_id: &B) -> Option<model::Edge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        self.edges.get_edge(edge_id).map(|edge| edge.as_model())
    }

    /// get undirected edge at edge_id
    pub fn get_undirected_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::UndirectedEdge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
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
    ) -> Option<model::DirectedEdge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_directed_model())
            .flatten()
    }

    /// get mixed edge at edge_id
    pub fn get_mixed_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::MixedEdge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_mixed_model())
            .flatten()
    }

    /// get undirected hyper edge at edge_id
    pub fn get_undirected_hyper_edge<'a, B: ?Sized>(
        &'a self,
        edge_id: &B,
    ) -> Option<model::UndirectedHyperEdge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
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
    ) -> Option<model::DirectedHyperEdge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
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
    ) -> Option<model::MixedHyperEdge<'a, NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        self.edges
            .get_edge(edge_id)
            .map(|edge| edge.as_mixed_hyper_model())
            .flatten()
    }

    /// to iterator for edge
    pub fn edge_iter<'a>(&'a self) -> EdgeIter<'a, NodeId, EdgeId> {
        EdgeIter::new(&self.edges)
    }

    /// to iterator for undirected edge
    pub fn undirected_edge_iter<'a>(&'a self) -> UndirectedEdgeIter<'a, NodeId, EdgeId> {
        UndirectedEdgeIter::new(&self.edges)
    }

    /// to iterator for directed edge
    pub fn directed_edge_iter<'a>(&'a self) -> DirectedEdgeIter<'a, NodeId, EdgeId> {
        DirectedEdgeIter::new(&self.edges)
    }

    /// to iterator for undirected of directed edge
    pub fn mixed_edge_iter<'a>(&'a self) -> MixedEdgeIter<'a, NodeId, EdgeId> {
        MixedEdgeIter::new(&self.edges)
    }

    /// to iterator for undirected hyper edge
    pub fn undirected_hyper_edge_iter<'a>(&'a self) -> UndirectedHyperEdgeIter<'a, NodeId, EdgeId> {
        UndirectedHyperEdgeIter::new(&self.edges)
    }

    /// to iterator for directed hyper edge
    pub fn directed_hyper_edge_iter<'a>(&'a self) -> DirectedHyperEdgeIter<'a, NodeId, EdgeId> {
        DirectedHyperEdgeIter::new(&self.edges)
    }

    /// to iterator for undirected or directed hyper edge
    pub fn mixed_hyper_edge_iter<'a>(&'a self) -> MixedHyperEdgeIter<'a, NodeId, EdgeId> {
        MixedHyperEdgeIter::new(&self.edges)
    }

    // ---
    // setter
    // ---

    // ---
    // setter Node
    // ---

    /// add vertex node if not exist.
    /// If already exist at the id, then will not vertex node and return the node_id.
    pub fn add_vertex_node(
        &mut self,
        parent_id: Option<NodeId>,
        node_id: NodeId,
    ) -> GraphItemExistedResult<NodeId, NodeId, EdgeId> {
        self.add_vertex_node_with_weight(parent_id, node_id, 1)
    }

    /// add vertex node with weight if not exist.
    /// If already exist at the id, then will not vertex node and return the node_id.
    pub fn add_vertex_node_with_weight(
        &mut self,
        parent_id: Option<NodeId>,
        node_id: NodeId,
        weight: Weight,
    ) -> GraphItemExistedResult<NodeId, NodeId, EdgeId> {
        self.add_vertex_node_with_weight_if_old_not_exist(parent_id, node_id, weight)
    }

    /// add vertex node with weight.
    /// If already exist at the id, then will not vertex node and return the node_id.
    fn add_vertex_node_with_weight_if_old_not_exist(
        &mut self,
        parent_id: Option<NodeId>,
        node_id: NodeId,
        weight: Weight,
    ) -> GraphItemExistedResult<NodeId, NodeId, EdgeId> {
        // check old exist
        if self.nodes.get_node(&node_id).is_some() {
            // old node exist
            return Ok(Some(node_id));
        }

        let config = self.get_config();

        if let Some(_parent_id) = &parent_id {
            // check support grouping
            if !config.can_use_group_node() {
                return Err(GraphError::NotSupportGroupNode(parent_id.unwrap()));
            }

            // check specified name
            if _parent_id == &node_id {
                return Err(GraphError::CannotCreateVertex(
                    Some(parent_id.unwrap()),
                    node_id,
                ));
            }

            // check parent
            if let Some(parent) = self.nodes.get_node_as_mut(&_parent_id) {
                if !parent.is_group() {
                    return Err(GraphError::NotExistGroup(parent_id.unwrap()));
                }

                parent.add_child(node_id.clone());
            } else {
                return Err(GraphError::NotExistGroup(parent_id.unwrap()));
            }
        }

        // can create vertex node
        let mut node = store::Node::vertex_with_weight(weight);
        node.set_parent_optional(parent_id);
        self.nodes.insert_node(node_id, node);

        Ok(None)
    }

    /// add group node if not exist.
    /// If already exist at the id, then will not create group node and return the node id.
    pub fn add_group_node(
        &mut self,
        parent_id: Option<NodeId>,
        node_id: NodeId,
        children: Vec<NodeId>,
    ) -> GraphItemExistedResult<NodeId, NodeId, EdgeId> {
        self.add_group_node_with_weight(parent_id, node_id, 1, children)
    }

    /// add group node with weight if not exist.
    /// If already exist at the id, then will not create group node and return the node id.
    pub fn add_group_node_with_weight(
        &mut self,
        parent_id: Option<NodeId>,
        node_id: NodeId,
        weight: Weight,
        children: Vec<NodeId>,
    ) -> GraphItemExistedResult<NodeId, NodeId, EdgeId> {
        self.add_group_node_with_weight_if_old_not_exist(parent_id, node_id, weight, children)
    }

    /// add group node with weight.
    /// If already exist at the id, then will not create group node and return the node id.
    /// If use the mode to create not exist vertex node and children is available, create not exist child as vertex node.
    fn add_group_node_with_weight_if_old_not_exist(
        &mut self,
        parent_id: Option<NodeId>,
        node_id: NodeId,
        weight: Weight,
        child_node_ids: Vec<NodeId>,
    ) -> GraphItemExistedResult<NodeId, NodeId, EdgeId> {
        // check old exist
        if self.nodes.get_node(&node_id).is_some() {
            // old node exist
            return Ok(Some(node_id));
        }

        let config = self.get_config();

        // check support grouping
        if !config.can_use_group_node() {
            return Err(GraphError::NotSupportGroupNode(node_id));
        }

        // check specified name
        if let Some(_parent_id) = &parent_id {
            if _parent_id == &node_id {
                return Err(GraphError::CannotCreateGroup(
                    parent_id,
                    node_id,
                    child_node_ids,
                ));
            }
        }

        // cleaning specify children
        let mut child_node_ids = child_node_ids;
        child_node_ids.sort();
        child_node_ids.dedup();
        let child_node_ids = child_node_ids;

        // check illegal children
        let not_exist_child_ids =
            self.check_children_can_be_made_group(&parent_id, &node_id, &child_node_ids)?;
        if !not_exist_child_ids.is_empty() && !config.can_create_not_exist_vertex_node() {
            return Err(GraphError::NotExistChildrenCannotMakeAsGroupChild(
                node_id,
                not_exist_child_ids,
            ));
        }

        // check and modify parent
        match &parent_id {
            None => {
                // can create group node
            }
            Some(_parent_id) => {
                // check parent
                match self.nodes.get_node_as_mut(&_parent_id) {
                    Some(parent) if !parent.is_group() => {
                        return Err(GraphError::NotExistGroup(parent_id.unwrap()));
                    }
                    None => {
                        return Err(GraphError::NotExistGroup(parent_id.unwrap()));
                    }
                    Some(parent) => {
                        // can create group node

                        // remove children from old parent and set the group id
                        parent.remove_children(&child_node_ids);
                        parent.add_child(node_id.clone());
                    }
                }
            }
        }

        // replace parent for exist child node
        for child_id in child_node_ids.iter() {
            if !not_exist_child_ids.contains(child_id) {
                let node = self
                    .nodes
                    .get_node_as_mut(child_id)
                    .expect("Fail resolve exist node.");
                node.set_parent(node_id.clone());
            }
        }

        // already check can create. create under this graph root
        for not_exist_child_id in not_exist_child_ids.into_iter() {
            let mut child_node = store::Node::vertex_with_weight(1);
            child_node.set_parent(node_id.clone());
            self.nodes.insert_node(not_exist_child_id, child_node);
        }

        let mut group_node = store::Node::group_with_weight(weight, child_node_ids);
        group_node.set_parent_optional(parent_id);
        self.nodes.insert_node(node_id, group_node);

        Ok(None)
    }

    /// update node weight from it's old weight.
    pub fn update_node_weight<B: ?Sized, F>(
        &mut self,
        node_id: &B,
        new_weight: F,
    ) -> Result<(), GraphError<NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity + ToOwned<Owned = NodeId>,
        F: FnOnce(model::NodeKind, Weight) -> Weight,
    {
        return match self.nodes.get_node_as_mut(node_id) {
            None => Err(GraphError::NotExistNodeAtId(node_id.to_owned())),
            Some(node) => {
                let model = node.as_model();
                let kind = model.get_kind();
                let old_weight = model.get_weight();
                node.set_weight(new_weight(kind, old_weight));

                Ok(())
            }
        };
    }

    // ---
    // setter Edge
    // ---

    /// add undirected edge.
    /// If already exist at the id, then will not create undirected edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_undirected_edge(
        &mut self,
        edge_id: EdgeId,
        node_id1: NodeId,
        node_id2: NodeId,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_undirected_edge_with_weight(edge_id, node_id1, node_id2, 1 as Weight)
    }

    /// add undirected edge with weight.
    /// If already exist at the id, then will not create undirected edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_undirected_edge_with_weight(
        &mut self,
        edge_id: EdgeId,
        node_id1: NodeId,
        node_id2: NodeId,
        weight: Weight,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_edge_with_weight_if_old_not_exist(
            edge_id,
            Edge::undirected_with_weight(node_id1, node_id2, weight),
        )
    }

    /// add directed edge.
    /// If already exist at the id, then will not create directed edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_directed_edge(
        &mut self,
        edge_id: EdgeId,
        source_node_id: NodeId,
        target_node_id: NodeId,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_directed_edge_with_weight(edge_id, source_node_id, target_node_id, 1)
    }

    /// add directed edge with weight.
    /// If already exist at the id, then will not create directed edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_directed_edge_with_weight(
        &mut self,
        edge_id: EdgeId,
        source_node_id: NodeId,
        target_node_id: NodeId,
        weight: Weight,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_edge_with_weight_if_old_not_exist(
            edge_id,
            Edge::directed_with_weight(source_node_id, target_node_id, weight),
        )
    }

    /// add undirected hyper edge.
    /// If already exist at the id, then will not create undirected hyper edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_undirected_hyper_edge(
        &mut self,
        edge_id: EdgeId,
        node_ids: Vec<NodeId>,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_undirected_hyper_edge_with_weight(edge_id, node_ids, 1)
    }

    /// add undirected hyper edge with weight.
    /// If already exist at the id, then will not create undirected hyper edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_undirected_hyper_edge_with_weight(
        &mut self,
        edge_id: EdgeId,
        node_ids: Vec<NodeId>,
        weight: Weight,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_edge_with_weight_if_old_not_exist(
            edge_id,
            Edge::undirected_hyper_with_weight(node_ids, weight),
        )
    }

    /// add directed hyper edge.
    /// If already exist at the id, then will not create directed hyper edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_directed_hyper_edge(
        &mut self,
        edge_id: EdgeId,
        source_node_ids: Vec<NodeId>,
        target_node_ids: Vec<NodeId>,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_directed_hyper_edge_with_weight(edge_id, source_node_ids, target_node_ids, 1)
    }

    /// add directed hyper edge with weight.
    /// If already exist at the id, then will not create directed hyper edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_directed_hyper_edge_with_weight(
        &mut self,
        edge_id: EdgeId,
        source_node_ids: Vec<NodeId>,
        target_node_ids: Vec<NodeId>,
        weight: Weight,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        self.add_edge_with_weight_if_old_not_exist(
            edge_id,
            Edge::directed_hyper_with_weight(source_node_ids, target_node_ids, weight),
        )
    }

    /// Add edge. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    fn add_edge_with_weight_if_old_not_exist(
        &mut self,
        edge_id: EdgeId,
        edge: Edge<NodeId, EdgeId>,
    ) -> GraphItemExistedResult<EdgeId, NodeId, EdgeId> {
        // check old exist
        if self.edges.get_edge(&edge_id).is_some() {
            // old edge exist
            return Ok(Some(edge_id));
        }

        let config: &GraphConfig = self.get_config();

        // check support edge
        if !self.is_support_edge(&edge) {
            return Err(GraphError::EdgeNotSupported(edge_id, edge.into()));
        }

        // check illegal edge
        if edge.has_illegal() {
            return Err(GraphError::IllegalEdge(edge_id, edge.into()));
        }

        // check can construct the edge
        let not_exist_child_ids = self.check_incidence_nodes_can_make_edge(&edge_id, &edge)?;
        if !not_exist_child_ids.is_empty() && !config.can_create_not_exist_vertex_node() {
            return Err(GraphError::NotExistChildrenCannotMakeAsEdgeIncidence(
                edge_id,
                edge.into(),
                not_exist_child_ids,
            ));
        }

        // check same edge
        let can_multiple = config.can_multiple_edge();

        let can_replace_mode = config.can_replace_same_edge();
        let same_edge_ids = self.get_same_edge_ids(&edge);

        if !(can_replace_mode || can_multiple) && !same_edge_ids.is_empty() {
            return Err(GraphError::ExistSameEdge(
                edge_id,
                edge.into(),
                same_edge_ids,
            ));
        }

        // can create edge

        if can_replace_mode {
            // do replace

            // already check can create. create under this graph root
            for not_exist_child_id in not_exist_child_ids.into_iter() {
                let child_node = store::Node::vertex_with_weight(1);
                self.nodes.insert_node(not_exist_child_id, child_node);
            }

            //create incidence data from edge
            let incidences = self.generate_incidences_without_check(&edge_id, &edge);

            // replace edge
            for same_edge_id in same_edge_ids.iter() {
                self.edges.remove(same_edge_id);
            }
            self.edges.insert_edge(edge_id, edge);

            // replace incidence data for node
            self.nodes
                .replace_incidences_each_already_exist_node(incidences, &same_edge_ids);

            Ok(None)
        } else {
            // do not replace

            // already check can create. create under this graph root
            for not_exist_child_id in not_exist_child_ids.into_iter() {
                let child_node = store::Node::vertex_with_weight(1);
                self.nodes.insert_node(not_exist_child_id, child_node);
            }

            //create incidence data from edge
            let incidences = self.generate_incidences_without_check(&edge_id, &edge);

            // add edge (old edge not exist)
            self.edges.insert_edge(edge_id, edge);

            // add incidence data for node
            self.nodes
                .add_incidences_each_already_exist_node(incidences);

            Ok(None)
        }
    }

    /// update edge weight from it's old weight.
    pub fn update_edge_weight<B: ?Sized, F>(
        &mut self,
        edge_id: &B,
        new_weight: F,
    ) -> Result<(), GraphError<NodeId, EdgeId>>
    where
        EdgeId: Borrow<B>,
        B: Identity + ToOwned<Owned = EdgeId>,
        F: FnOnce(model::EdgeKind, Weight) -> Weight,
    {
        return match self.edges.get_edge_as_mut(edge_id) {
            None => Err(GraphError::NotExistEdgeAtId(edge_id.to_owned())),
            Some(edge) => {
                let model = edge.as_model();
                let kind = model.get_kind();
                let old_weight = model.get_weight();
                edge.set_weight(new_weight(kind, old_weight));

                Ok(())
            }
        };
    }

    // ---
    // checker
    // ---

    /// check children to be able to make group at node id in the parent
    fn check_children_can_be_made_group(
        &self,
        parent_id: &Option<NodeId>,
        node_id: &NodeId,
        child_node_ids: &[NodeId],
    ) -> Result<Vec<NodeId>, GraphError<NodeId, EdgeId>> {
        let mut not_exist_children_id = Vec::new();
        let mut exist_error_children_id = Vec::new();

        // check illegal incidence edge (e.g. child <-> node_id or parent's parents)
        let illegal_node_ids = match parent_id.as_ref() {
            Some(_parent_id) => {
                let (incidence_node_ids_from_self, parent_ids) =
                    self.get_incidence_node_ids_from_self_and_parent_node_ids(_parent_id);
                let mut _illegal_node_ids = incidence_node_ids_from_self;
                _illegal_node_ids.extend(parent_ids);
                // set the node_id at this time because of satisfy lifetime.
                _illegal_node_ids.push(_parent_id);
                _illegal_node_ids.push(node_id);
                _illegal_node_ids
            }
            None => vec![node_id],
        };

        // check available root with set child node as vertex node if not exist.
        for child_node_id in child_node_ids.iter() {
            if illegal_node_ids.contains(&child_node_id) {
                // illegal child node id
                exist_error_children_id.push(child_node_id.clone());
                continue;
            }

            match self.nodes.get_node(child_node_id) {
                None => {
                    not_exist_children_id.push(child_node_id.clone());
                    continue;
                }
                Some(child_node) => {
                    match child_node.get_parent() {
                        Some(child_parent_id) => {
                            if parent_id == &None || parent_id.as_ref() != Some(child_parent_id) {
                                // illegal parent
                                exist_error_children_id.push(child_node_id.clone());
                                continue;
                            }
                        }
                        None => {
                            // If not specify parent, not exist parent ids or parent's incidence nodes. So no check,
                            if parent_id == &None {
                                continue;
                            }

                            // If specify parent, exist parent ids or parent's incidence nodes. So check child incidence to parents.
                            let mut child_children = child_node.get_children_as_ref();
                            loop {
                                match child_children.pop() {
                                    None => {
                                        break;
                                    }
                                    Some(child_child_id) => {
                                        if illegal_node_ids.contains(&child_child_id) {
                                            // illegal child child node id
                                            exist_error_children_id.push(child_child_id.clone());
                                            break;
                                        }

                                        match self.nodes.get_node(child_child_id) {
                                            None => {
                                                unreachable!(
                                                    "Already exist child node is not exist."
                                                );
                                            }
                                            Some(child_child_node) => {
                                                child_children
                                                    .extend(child_child_node.get_children_as_ref());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !exist_error_children_id.is_empty() {
            return Err(GraphError::SpecifiedIllegalChildren(
                node_id.clone(),
                exist_error_children_id,
            ));
        }

        Ok(not_exist_children_id)
    }

    /// check incidence nodes to be able to make group at edge id
    fn check_incidence_nodes_can_make_edge(
        &self,
        edge_id: &EdgeId,
        edge: &store::Edge<NodeId, EdgeId>,
    ) -> Result<Vec<NodeId>, GraphError<NodeId, EdgeId>> {
        let mut not_exist_children_id = Vec::new();
        let mut exist_error_children_id = Vec::new();
        let mut checkers = Vec::new();
        // prepare checkers
        if edge.is_directed_hyper() {
            for source_id in edge.get_source_ids().iter() {
                match self.nodes.flatten_parent_ids(*source_id) {
                    Ok(Some(flatten)) => {
                        checkers.push(flatten);
                    }
                    Ok(None) => {
                        not_exist_children_id.push((*source_id).clone());
                    }
                    Err(()) => {
                        exist_error_children_id.push((*source_id).clone());
                    }
                }
            }
        };

        let wait_checks = if edge.is_directed_hyper() {
            edge.get_target_ids()
        } else {
            edge.get_incidence_node_ids_as_ref()
        };

        // run check
        'check: for wait_check in wait_checks.into_iter() {
            match self.nodes.flatten_parent_ids(wait_check) {
                Ok(Some(flatten)) => {
                    for checker in checkers.iter() {
                        if checker.children_contains_other(&flatten)
                            || flatten.children_contains_other(checker)
                        {
                            exist_error_children_id.push(flatten.get_root().clone());
                            continue 'check;
                        }
                    }
                    checkers.push(flatten);
                }
                Ok(None) => {
                    not_exist_children_id.push((*wait_check).clone());
                }
                Err(()) => {
                    exist_error_children_id.push((*wait_check).clone());
                }
            }
        }

        if !exist_error_children_id.is_empty() {
            return Err(GraphError::SpecifiedIllegalIncidenceNodeIds(
                edge_id.clone(),
                edge.clone().into(),
                exist_error_children_id,
            ));
        }

        Ok(not_exist_children_id)
    }

    /// check configure support this edge type.
    fn is_support_edge(&self, edge: &Edge<NodeId, EdgeId>) -> bool {
        use store::Edge::*;
        let config = self.get_config();

        match edge {
            Undirected { .. } => config.can_use_undirected_edge(),
            Directed { .. } => config.can_use_directed_edge(),
            UndirectedHyper { .. } => config.can_use_undirected_hyper_edge(),
            DirectedHyper { .. } => config.can_use_directed_hyper_edge(),
        }
    }

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
        self.nodes.clear_incidence();
        self.edges.clear();
    }

    /// remove node_id and node's incidences from edge store
    /// return value is Vec<(node_id, edge_id>
    fn remove_node_id_and_illegal_edge_with_collect(
        &mut self,
        deleted_node_id: &NodeId,
        deleted_node: Node<NodeId, EdgeId>,
    ) -> Vec<(NodeId, EdgeId)> {
        let deleted_incidences = deleted_node.into_incidences();
        let mut will_delete_node_id_edge_id: Vec<(NodeId, EdgeId)> = Vec::new();
        for incidence in deleted_incidences.into_iter() {
            match incidence {
                Incidence::Undirected { edge_id, .. } => {
                    let edge_entry = self.edges.entry(edge_id);
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
                                    "Unknown edge {} at the edge_id {:?} for undirected incidence ",
                                    occupied.get(),
                                    occupied.key(),
                                )
                            }
                        }
                    }
                }
                Incidence::DirectedSource { edge_id, .. }
                | Incidence::DirectedTarget { edge_id, .. } => {
                    let edge_entry = self.edges.entry(edge_id);
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
                                    "Unknown edge {} at the edge_id {:?} for directed source or target incidence ",
                                    occupied.get(),
                                    occupied.key(),
                                )
                            }
                        }
                    }
                }
                Incidence::UndirectedHyper { edge_id, .. } => {
                    let edge_entry = self.edges.entry(edge_id);
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
                                    "Unknown edge {} at the edge_id {:?} for undirected hyper incidence ",
                                    occupied.get(),
                                    occupied.key(),
                                )
                            }
                        }
                    }
                }
                Incidence::DirectedHyperSource { edge_id, .. }
                | Incidence::DirectedHyperTarget { edge_id, .. } => {
                    let edge_entry = self.edges.entry(edge_id);
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
                                    "Unknown edge {} at the edge_id {:?} for directed hyper source or target incidence ",
                                    occupied.get(),
                                    occupied.key(),
                                )
                            }
                        }
                    }
                }
            }
        }

        will_delete_node_id_edge_id
    }

    /// delete node at node_id if exist with remove illegal edge.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and not remove the group's children.
    pub fn delete_node<B: ?Sized>(&mut self, node_id: &B) -> Option<NodeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            // If exist parent. remove the child from the parent group which have this node as the child.
            if let Some(parent_id) = remove_node.get_parent() {
                if let Some(_parent) = self.nodes.get_node_as_mut::<NodeId>(parent_id) {
                    _parent.remove_child(node_id);
                }
            }

            let _children: Vec<NodeId> = remove_node.get_children().to_vec();

            let will_delete_incidences =
                self.remove_node_id_and_illegal_edge_with_collect(&remove_node_id, remove_node);
            self.nodes.remove_edges_by_ids(&will_delete_incidences);

            for child_id in _children.iter() {
                if let Some(child) = self.nodes.get_node_as_mut::<NodeId>(child_id) {
                    child.remove_parent();
                }
            }

            Some(remove_node_id)
        } else {
            None
        }
    }

    /// delete node at node_id if exist with remove illegal edge.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and the group's children.
    pub fn delete_node_if_group_with_child<B: ?Sized>(&mut self, node_id: &B) -> Option<NodeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.rec_delete_node_if_group_with_child(node_id, true)
    }

    /// delete node at node_id if exist with remove illegal edge.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and the group's children.
    fn rec_delete_node_if_group_with_child<B: ?Sized>(
        &mut self,
        node_id: &B,
        is_root: bool,
    ) -> Option<NodeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            if is_root {
                // If exist parent. remove the child from the parent group which have this node as the child.
                if let Some(parent_id) = remove_node.get_parent() {
                    if let Some(_parent) = self.nodes.get_node_as_mut::<NodeId>(parent_id) {
                        _parent.remove_child(node_id);
                    }
                }
            }

            let mut _children: Vec<NodeId> = remove_node.get_children().to_vec();

            let will_delete_incidences =
                self.remove_node_id_and_illegal_edge_with_collect(&remove_node_id, remove_node);
            self.nodes.remove_edges_by_ids(&will_delete_incidences);

            for child_id in _children.iter() {
                self.rec_delete_node_if_group_with_child::<NodeId>(child_id, false);
            }

            Some(remove_node_id)
        } else {
            None
        }
    }

    /// delete node at the node_id with incidence edges.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and not remove the group's children.
    pub fn delete_node_with_edge<B: ?Sized>(&mut self, node_id: &B) -> Option<NodeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            // If exist parent. remove the child from the parent group which have this node as the child.
            if let Some(parent_id) = remove_node.get_parent() {
                if let Some(_parent) = self.nodes.get_node_as_mut::<NodeId>(parent_id) {
                    _parent.remove_child(node_id);
                }
            }

            let _children: Vec<NodeId> = remove_node.get_children().to_vec();

            let edge_ids = remove_node.incidences_into_edge_ids();
            for edge_id in edge_ids.iter() {
                self.delete_edge::<EdgeId>(edge_id);
            }

            for child_id in _children.iter() {
                if let Some(child) = self.nodes.get_node_as_mut::<NodeId>(child_id) {
                    child.remove_parent();
                }
            }

            Some(remove_node_id)
        } else {
            None
        }
    }

    /// delete node at the node_id with incidence edges.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and the group's children.
    pub fn delete_node_with_edge_if_group_with_child<B: ?Sized>(
        &mut self,
        node_id: &B,
    ) -> Option<NodeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.rec_delete_node_with_edge_if_group_with_child(node_id, true)
    }

    /// delete node at the node_id with incidence edges.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and the group's children.
    fn rec_delete_node_with_edge_if_group_with_child<B: ?Sized>(
        &mut self,
        node_id: &B,
        is_root: bool,
    ) -> Option<NodeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            if is_root {
                // If exist parent. remove the child from the parent group which have this node as the child.
                if let Some(parent_id) = remove_node.get_parent() {
                    if let Some(_parent) = self.nodes.get_node_as_mut::<NodeId>(parent_id) {
                        _parent.remove_child(node_id);
                    }
                }
            }

            let mut _children: Vec<NodeId> = remove_node.get_children().to_vec();

            let edge_ids = remove_node.incidences_into_edge_ids();
            for edge_id in edge_ids.iter() {
                self.delete_edge::<EdgeId>(edge_id);
            }

            for child_id in _children.iter() {
                self.rec_delete_node_with_edge_if_group_with_child::<NodeId>(child_id, false);
            }

            Some(remove_node_id)
        } else {
            None
        }
    }

    /// delete edge without delete node.
    ///
    /// If exist edge, then return the id.
    pub fn delete_edge<B: ?Sized>(&mut self, edge_id: &B) -> Option<EdgeId>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_edge_id, remove_edge)) = self.edges.remove_with_get_id(edge_id) {
            let incidence_node_ids = remove_edge.into_incidence_node_ids();
            for incidence_node_id in incidence_node_ids.iter() {
                self.nodes
                    .remove_edges_by_id(incidence_node_id.borrow(), &edge_id);
            }

            Some(remove_edge_id)
        } else {
            None
        }
    }

    /// delete edge with incidence node.
    ///
    /// If exist edge, then return the id.
    /// If specify edge's incidence node is group, remove the group node and not remove the group's children.
    pub fn delete_edge_with_node<B: ?Sized>(&mut self, edge_id: &B) -> Option<EdgeId>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_edge_id, remove_edge)) = self.edges.remove_with_get_id(edge_id) {
            let incidence_node_ids = remove_edge.into_incidence_node_ids();
            for incidence_node_id in incidence_node_ids.iter() {
                self.delete_node::<NodeId>(incidence_node_id);
            }

            Some(remove_edge_id)
        } else {
            None
        }
    }

    /// delete edge with incidence node.
    ///
    /// If exist edge, then return the id.
    /// If specify edge's incidence node is group, remove the group node and the group's children.
    pub fn delete_edge_with_node_if_group_with_child<B: ?Sized>(
        &mut self,
        edge_id: &B,
    ) -> Option<EdgeId>
    where
        EdgeId: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_edge_id, remove_edge)) = self.edges.remove_with_get_id(edge_id) {
            let incidence_node_ids = remove_edge.into_incidence_node_ids();
            for incidence_node_id in incidence_node_ids.iter() {
                self.delete_node_if_group_with_child::<NodeId>(incidence_node_id);
            }

            Some(remove_edge_id)
        } else {
            None
        }
    }
}
