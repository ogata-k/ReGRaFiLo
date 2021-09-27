//! Module for graph structure as graph theory.

use std::borrow::Borrow;
use std::fmt;

pub use config::*;
use edge::*;
use node::*;

use crate::graph::error::GraphError;
use crate::graph::iter::*;
use crate::util::Identity;

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

pub mod helper {
    //! Module for helper for handling graph items

    use crate::graph::error::GraphError;
    use crate::graph::{model, Graph};
    use crate::util::Identity;

    /// helper for handling result data
    pub trait GraphItemExistedResultExt<Id: Identity> {
        // ---
        // node
        // ---
        /// If old node exist, then return Err(GraphError::AlreadyNodeExist). Others same.
        fn old_node_exist_to_error(self) -> Result<(), GraphError<Id>>;
        /// If old node exist, then return Ok(Some(model::Node))
        fn with_old_node_model<'a>(
            self,
            graph: &'a Graph<Id>,
        ) -> Result<Option<(Id, model::Node<'a, Id>)>, GraphError<Id>>;
        /// call callback when old node exist
        fn call_if_old_node_exist<F>(self, callback: F) -> Result<(), GraphError<Id>>
        where
            F: FnOnce(Id) -> Result<(), GraphError<Id>>;
        /// map if inserted node. If op's arg is None then create just time else already exist.
        fn map_if_node_inserted<U, F>(self, op: F) -> Result<U, GraphError<Id>>
        where
            F: FnOnce(Option<Id>) -> Result<U, GraphError<Id>>;

        // ---
        // edge
        // ---
        /// If old edge exist, then return Err(GraphError::AlreadyEdgeExist). Others same.
        fn old_edge_exist_to_error(self) -> Result<(), GraphError<Id>>;
        /// If old edge exist, then return Ok(Some(model::Edge))
        fn with_old_edge_model<'a>(
            self,
            graph: &'a Graph<Id>,
        ) -> Result<Option<(Id, model::Edge<'a, Id>)>, GraphError<Id>>;
        /// call callback when old edge exist
        fn call_if_old_edge_exist<F>(self, callback: F) -> Result<(), GraphError<Id>>
        where
            F: FnOnce(Id) -> Result<(), GraphError<Id>>;
        /// map if inserted edge. If op's arg is None then create just time else already exist.
        fn map_if_edge_inserted<U, F>(self, op: F) -> Result<U, GraphError<Id>>
        where
            F: FnOnce(Option<Id>) -> Result<U, GraphError<Id>>;
    }

    /// If this value is Ok(Some(id)), then old graph item exist at the id.
    pub type GraphItemExistedResult<Id> = Result<Option<Id>, GraphError<Id>>;
    impl<Id: Identity> GraphItemExistedResultExt<Id> for GraphItemExistedResult<Id> {
        fn with_old_node_model<'a>(
            self,
            graph: &'a Graph<Id>,
        ) -> Result<Option<(Id, model::Node<'a, Id>)>, GraphError<Id>> {
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

        fn old_node_exist_to_error(self) -> Result<(), GraphError<Id>> {
            self.and_then(old_node_exist_to_error)
        }

        fn call_if_old_node_exist<F>(self, callback: F) -> Result<(), GraphError<Id>>
        where
            F: FnOnce(Id) -> Result<(), GraphError<Id>>,
        {
            match self {
                Ok(None) => Ok(()),
                Ok(Some(node_id)) => callback(node_id),
                Err(e) => Err(e),
            }
        }

        fn map_if_node_inserted<U, F>(self, op: F) -> Result<U, GraphError<Id>>
        where
            F: FnOnce(Option<Id>) -> Result<U, GraphError<Id>>,
        {
            match self {
                Ok(s) => op(s),
                Err(e) => Err(e),
            }
        }

        fn old_edge_exist_to_error(self) -> Result<(), GraphError<Id>> {
            self.and_then(old_edge_exist_to_error)
        }

        fn with_old_edge_model<'a>(
            self,
            graph: &'a Graph<Id>,
        ) -> Result<Option<(Id, model::Edge<'a, Id>)>, GraphError<Id>> {
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

        fn call_if_old_edge_exist<F>(self, callback: F) -> Result<(), GraphError<Id>>
        where
            F: FnOnce(Id) -> Result<(), GraphError<Id>>,
        {
            match self {
                Ok(None) => Ok(()),
                Ok(Some(edge_id)) => callback(edge_id),
                Err(e) => Err(e),
            }
        }

        fn map_if_edge_inserted<U, F>(self, op: F) -> Result<U, GraphError<Id>>
        where
            F: FnOnce(Option<Id>) -> Result<U, GraphError<Id>>,
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
    pub fn old_node_exist_to_error<Id: Identity>(
        old_node_exist: Option<Id>,
    ) -> Result<(), GraphError<Id>> {
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
    pub fn old_edge_exist_to_error<Id: Identity>(
        old_edge_exist: Option<Id>,
    ) -> Result<(), GraphError<Id>> {
        match old_edge_exist {
            Some(edge_id) => {
                // old edge exist
                Err(GraphError::AlreadyExistEdgeAtId(edge_id))
            }
            None => Ok(()),
        }
    }
}
use crate::graph::edge::model::EdgeModel;
use crate::graph::model::NodeModel;
use helper::*;

/// graph without layout
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

    /// constructor for Graph
    pub fn create_as_undirected_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::create_by_config(GraphConfig::undirected_graph(
            can_multiple_edge,
            use_grouping,
        ))
    }

    /// constructor for Directed Graph
    pub fn create_as_directed_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::create_by_config(GraphConfig::directed_graph(can_multiple_edge, use_grouping))
    }

    /// constructor for Mixed Graph
    pub fn create_as_mixed_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::create_by_config(GraphConfig::mixed_graph(can_multiple_edge, use_grouping))
    }

    /// constructor for Hyper Graph
    pub fn create_as_undirected_hyper_graph(can_multiple_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::undirected_hyper_graph(can_multiple_edge))
    }

    /// constructor for Directed Hyper Graph
    pub fn create_as_directed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::create_by_config(GraphConfig::directed_hyper_graph(can_multiple_hyper_edge))
    }

    /// constructor for Mixed Hyper Graph
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
    // getter Node
    // ---

    /// get incidence node ids to edges which incidence to the node which is between the node_id and top parent and get parent node ids
    fn get_incidence_node_ids_from_self_and_parent_node_ids(
        &self,
        node_id: &Id,
    ) -> (Vec<&Id>, Vec<&Id>) {
        let (incidence_edge_ids_from_self, parent_node_ids) = self
            .nodes
            .get_incidence_edge_ids_from_the_node_id_and_parent_ids(node_id);
        let incidence_node_ids_from_self = self
            .edges
            .get_incidence_node_ids_by_ids(incidence_edge_ids_from_self.as_slice());

        (incidence_node_ids_from_self, parent_node_ids)
    }

    /// get node at node_id
    pub fn get_node<'a, B: ?Sized>(&'a self, node_id: &B) -> Option<model::Node<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.nodes.get_node(node_id).map(|node| node.as_model())
    }

    /// get node point at node_id
    pub fn get_vertex_node<'a, B: ?Sized>(
        &'a self,
        node_id: &B,
    ) -> Option<model::VertexNode<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.nodes
            .get_node(node_id)
            .map(|node| node.as_vertex_model())
            .flatten()
    }

    /// get node group at node_id
    pub fn get_group_node<'a, B: ?Sized>(&'a self, node_id: &B) -> Option<model::GroupNode<'a, Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.nodes
            .get_node(node_id)
            .map(|node| node.as_group_model())
            .flatten()
    }

    /// to iterator for node
    pub fn node_iter<'a>(&'a self) -> NodeIter<'a, Id> {
        self.nodes.node_iter()
    }

    /// to iterator for node point
    pub fn vertex_node_iter<'a>(&'a self) -> VertexNodeIter<'a, Id> {
        self.nodes.vertex_node_iter()
    }

    /// to iterator for node group
    pub fn group_node_iter<'a>(&'a self) -> GroupNodeIter<'a, Id> {
        self.nodes.group_node_iter()
    }

    /// to iterator for grouping child nodes
    pub fn group_child_node_iter<'a, B: ?Sized>(
        &'a self,
        group_id: Option<&'a B>,
    ) -> GroupChildNodeIter<'a, Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.nodes.group_child_node_iter(group_id)
    }

    // ---
    // getter Edge
    // ---

    /// get edge ids which have same incidence nodes
    pub fn get_same_edge_ids(&self, edge: &Edge<Id>) -> Vec<Id> {
        match edge.get_incidence_node_ids_as_ref().first() {
            None => vec![],
            Some(node_id) => {
                return match self.nodes.get_node(node_id) {
                    None => Vec::new(),
                    Some(node) => {
                        let will_check_edge_ids: Vec<&Id> = node
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
    pub fn edge_iter<'a>(&'a self) -> EdgeIter<'a, Id> {
        self.edges.edge_iter()
    }

    /// to iterator for undirected edge
    pub fn undirected_edge_iter<'a>(&'a self) -> UndirectedEdgeIter<'a, Id> {
        self.edges.undirected_edge_iter()
    }

    /// to iterator for directed edge
    pub fn directed_edge_iter<'a>(&'a self) -> DirectedEdgeIter<'a, Id> {
        self.edges.directed_edge_iter()
    }

    /// to iterator for undirected of directed edge
    pub fn mixed_edge_iter<'a>(&'a self) -> MixedEdgeIter<'a, Id> {
        self.edges.mixed_edge_iter()
    }

    /// to iterator for undirected hyper edge
    pub fn undirected_hyper_edge_iter<'a>(&'a self) -> UndirectedHyperEdgeIter<'a, Id> {
        self.edges.undirected_hyper_edge_iter()
    }

    /// to iterator for directed hyper edge
    pub fn directed_hyper_edge_iter<'a>(&'a self) -> DirectedHyperEdgeIter<'a, Id> {
        self.edges.directed_hyper_edge_iter()
    }

    /// to iterator for undirected or directed hyper edge
    pub fn mixed_hyper_edge_iter<'a>(&'a self) -> MixedHyperEdgeIter<'a, Id> {
        self.edges.mixed_hyper_edge_iter()
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
        parent_id: Option<Id>,
        node_id: Id,
    ) -> GraphItemExistedResult<Id> {
        self.add_vertex_node_with_weight(parent_id, node_id, 1)
    }

    /// add vertex node with weight if not exist.
    /// If already exist at the id, then will not vertex node and return the node_id.
    pub fn add_vertex_node_with_weight(
        &mut self,
        parent_id: Option<Id>,
        node_id: Id,
        weight: i16,
    ) -> GraphItemExistedResult<Id> {
        self.add_vertex_node_with_weight_if_old_not_exist(parent_id, node_id, weight)
    }

    /// add vertex node with weight.
    /// If already exist at the id, then will not vertex node and return the node_id.
    fn add_vertex_node_with_weight_if_old_not_exist(
        &mut self,
        parent_id: Option<Id>,
        node_id: Id,
        weight: i16,
    ) -> GraphItemExistedResult<Id> {
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
            if let Some(parent) = self.nodes.get_node(&_parent_id) {
                if !parent.is_group() {
                    return Err(GraphError::NotExistGroup(parent_id.unwrap()));
                }
            } else {
                return Err(GraphError::NotExistGroup(parent_id.unwrap()));
            }
        }

        // can create vertex node
        let mut node = Node::vertex_with_weight(weight);
        node.set_parent_optional(parent_id);
        self.nodes.insert_node(node_id, node);

        Ok(None)
    }

    /// add group node if not exist.
    /// If already exist at the id, then will not create group node and return the node id.
    pub fn add_group_node(
        &mut self,
        parent_id: Option<Id>,
        node_id: Id,
        children: Vec<Id>,
    ) -> GraphItemExistedResult<Id> {
        self.add_group_node_with_weight(parent_id, node_id, 1, children)
    }

    /// add group node with weight if not exist.
    /// If already exist at the id, then will not create group node and return the node id.
    pub fn add_group_node_with_weight(
        &mut self,
        parent_id: Option<Id>,
        node_id: Id,
        weight: i16,
        children: Vec<Id>,
    ) -> GraphItemExistedResult<Id> {
        self.add_group_node_with_weight_if_old_not_exist(parent_id, node_id, weight, children)
    }

    /// add group node with weight.
    /// If already exist at the id, then will not create group node and return the node id.
    /// If use the mode to create not exist vertex node and children is available, create not exist child as vertex node.
    fn add_group_node_with_weight_if_old_not_exist(
        &mut self,
        parent_id: Option<Id>,
        node_id: Id,
        weight: i16,
        child_node_ids: Vec<Id>,
    ) -> GraphItemExistedResult<Id> {
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
            return Err(GraphError::NotExistChildrenCannotMakeEdge(
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
            let mut child_node = Node::vertex_with_weight(1);
            child_node.set_parent(node_id.clone());
            self.nodes.insert_node(not_exist_child_id, child_node);
        }

        let mut group_node = Node::group_with_weight(weight, child_node_ids);
        group_node.set_parent_optional(parent_id);
        self.nodes.insert_node(node_id, group_node);

        Ok(None)
    }

    /// update node weight from it's old weight.
    pub fn update_node_weight<B: ?Sized, F>(
        &mut self,
        node_id: &B,
        new_weight: F,
    ) -> Result<(), GraphError<Id>>
    where
        Id: Borrow<B>,
        B: Identity + ToOwned<Owned = Id>,
        F: FnOnce(model::NodeKind, i16) -> i16,
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
        edge_id: Id,
        node_id1: Id,
        node_id2: Id,
    ) -> GraphItemExistedResult<Id> {
        self.add_undirected_edge_with_weight(edge_id, node_id1, node_id2, 1)
    }

    /// add undirected edge with weight.
    /// If already exist at the id, then will not create undirected edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_undirected_edge_with_weight(
        &mut self,
        edge_id: Id,
        node_id1: Id,
        node_id2: Id,
        weight: i16,
    ) -> GraphItemExistedResult<Id> {
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
        edge_id: Id,
        source_node_id: Id,
        target_node_id: Id,
    ) -> GraphItemExistedResult<Id> {
        self.add_directed_edge_with_weight(edge_id, source_node_id, target_node_id, 1)
    }

    /// add directed edge with weight.
    /// If already exist at the id, then will not create directed edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_directed_edge_with_weight(
        &mut self,
        edge_id: Id,
        source_node_id: Id,
        target_node_id: Id,
        weight: i16,
    ) -> GraphItemExistedResult<Id> {
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
        edge_id: Id,
        node_ids: Vec<Id>,
    ) -> GraphItemExistedResult<Id> {
        self.add_undirected_hyper_edge_with_weight(edge_id, node_ids, 1)
    }

    /// add undirected hyper edge with weight.
    /// If already exist at the id, then will not create undirected hyper edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_undirected_hyper_edge_with_weight(
        &mut self,
        edge_id: Id,
        node_ids: Vec<Id>,
        weight: i16,
    ) -> GraphItemExistedResult<Id> {
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
        edge_id: Id,
        source_node_ids: Vec<Id>,
        target_node_ids: Vec<Id>,
    ) -> GraphItemExistedResult<Id> {
        self.add_directed_hyper_edge_with_weight(edge_id, source_node_ids, target_node_ids, 1)
    }

    /// add directed hyper edge with weight.
    /// If already exist at the id, then will not create directed hyper edge and return the edge id.
    /// If use the mode to create not exist vertex node and children is available, create not exist incidence nodes as vertex node.
    pub fn add_directed_hyper_edge_with_weight(
        &mut self,
        edge_id: Id,
        source_node_ids: Vec<Id>,
        target_node_ids: Vec<Id>,
        weight: i16,
    ) -> GraphItemExistedResult<Id> {
        self.add_edge_with_weight_if_old_not_exist(
            edge_id,
            Edge::directed_hyper_with_weight(source_node_ids, target_node_ids, weight),
        )
    }

    /// Add edge. If exist at the edge_id, not replace when replace is false.
    /// If inserted at the edge_id, replace insert at the edge_id
    fn add_edge_with_weight_if_old_not_exist(
        &mut self,
        edge_id: Id,
        edge: Edge<Id>,
    ) -> GraphItemExistedResult<Id> {
        // check old exist
        if self.edges.get_edge(&edge_id).is_some() {
            // old edge exist
            return Ok(Some(edge_id));
        }

        let config: &GraphConfig = self.get_config();

        // check support edge
        if !edge.is_support(config) {
            return Err(GraphError::EdgeNotSupported(edge_id, edge.into()));
        }

        // check illegal edge
        if edge.has_illegal() {
            return Err(GraphError::IllegalEdge(edge_id, edge.into()));
        }

        // check can construct the edge
        let not_exist_child_ids = self.check_incidence_nodes_can_make_edge(&edge_id, &edge)?;
        if !not_exist_child_ids.is_empty() && !config.can_create_not_exist_vertex_node() {
            return Err(GraphError::NotExistChildrenCannotMakeEdge(
                edge_id,
                not_exist_child_ids,
            ));
        }

        // check same edge
        let can_multiple = if edge.is_edge() {
            config.can_multiple_edge()
        } else {
            config.can_multiple_hyper_edge()
        };
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
                let child_node = Node::vertex_with_weight(1);
                self.nodes.insert_node(not_exist_child_id, child_node);
            }

            //create incidence data from edge
            let incidences = edge.generate_incidences_without_check(&edge_id);

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
                let child_node = Node::vertex_with_weight(1);
                self.nodes.insert_node(not_exist_child_id, child_node);
            }

            //create incidence data from edge
            let incidences = edge.generate_incidences_without_check(&edge_id);

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
    ) -> Result<(), GraphError<Id>>
    where
        Id: Borrow<B>,
        B: Identity + ToOwned<Owned = Id>,
        F: FnOnce(model::EdgeKind, i16) -> i16,
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
        parent_id: &Option<Id>,
        node_id: &Id,
        child_node_ids: &[Id],
    ) -> Result<Vec<Id>, GraphError<Id>> {
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
        edge_id: &Id,
        edge: &Edge<Id>,
    ) -> Result<Vec<Id>, GraphError<Id>> {
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

    /// delete node at node_id if exist with remove illegal edge.
    ///
    /// If exist node, then return the id.
    /// If specify node is group, remove the group node and not remove the group's children.
    pub fn delete_node<B: ?Sized>(&mut self, node_id: &B) -> Option<Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            // If exist parent. remove the child from the parent group which have this node as the child.
            if let Some(parent_id) = remove_node.get_parent() {
                if let Some(_parent) = self.nodes.get_node_as_mut::<Id>(parent_id) {
                    _parent.remove_child(node_id);
                }
            }

            let _children: Vec<Id> = remove_node.get_children().to_vec();

            let will_delete_incidences = self
                .edges
                .remove_node_id_and_illegal_edge_with_collect(&remove_node_id, remove_node);
            self.nodes.remove_edges_by_ids(&will_delete_incidences);

            for child_id in _children.iter() {
                if let Some(child) = self.nodes.get_node_as_mut::<Id>(child_id) {
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
    pub fn delete_node_if_group_with_child<B: ?Sized>(&mut self, node_id: &B) -> Option<Id>
    where
        Id: Borrow<B>,
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
    ) -> Option<Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            if is_root {
                // If exist parent. remove the child from the parent group which have this node as the child.
                if let Some(parent_id) = remove_node.get_parent() {
                    if let Some(_parent) = self.nodes.get_node_as_mut::<Id>(parent_id) {
                        _parent.remove_child(node_id);
                    }
                }
            }

            let mut _children: Vec<Id> = remove_node.get_children().to_vec();

            let will_delete_incidences = self
                .edges
                .remove_node_id_and_illegal_edge_with_collect(&remove_node_id, remove_node);
            self.nodes.remove_edges_by_ids(&will_delete_incidences);

            for child_id in _children.iter() {
                self.rec_delete_node_if_group_with_child::<Id>(child_id, false);
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
    pub fn delete_node_with_edge<B: ?Sized>(&mut self, node_id: &B) -> Option<Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            // If exist parent. remove the child from the parent group which have this node as the child.
            if let Some(parent_id) = remove_node.get_parent() {
                if let Some(_parent) = self.nodes.get_node_as_mut::<Id>(parent_id) {
                    _parent.remove_child(node_id);
                }
            }

            let _children: Vec<Id> = remove_node.get_children().to_vec();

            let edge_ids = remove_node.incidences_into_edge_ids();
            for edge_id in edge_ids.iter() {
                self.delete_edge::<Id>(edge_id);
            }

            for child_id in _children.iter() {
                if let Some(child) = self.nodes.get_node_as_mut::<Id>(child_id) {
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
    ) -> Option<Id>
    where
        Id: Borrow<B>,
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
    ) -> Option<Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_node_id, remove_node)) = self.nodes.remove_with_get_id(node_id) {
            if is_root {
                // If exist parent. remove the child from the parent group which have this node as the child.
                if let Some(parent_id) = remove_node.get_parent() {
                    if let Some(_parent) = self.nodes.get_node_as_mut::<Id>(parent_id) {
                        _parent.remove_child(node_id);
                    }
                }
            }

            let mut _children: Vec<Id> = remove_node.get_children().to_vec();

            let edge_ids = remove_node.incidences_into_edge_ids();
            for edge_id in edge_ids.iter() {
                self.delete_edge::<Id>(edge_id);
            }

            for child_id in _children.iter() {
                self.rec_delete_node_with_edge_if_group_with_child::<Id>(child_id, false);
            }

            Some(remove_node_id)
        } else {
            None
        }
    }

    /// delete edge without delete node.
    ///
    /// If exist edge, then return the id.
    pub fn delete_edge<B: ?Sized>(&mut self, edge_id: &B) -> Option<Id>
    where
        Id: Borrow<B>,
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
    pub fn delete_edge_with_node<B: ?Sized>(&mut self, edge_id: &B) -> Option<Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_edge_id, remove_edge)) = self.edges.remove_with_get_id(edge_id) {
            let incidence_node_ids = remove_edge.into_incidence_node_ids();
            for incidence_node_id in incidence_node_ids.iter() {
                self.delete_node::<Id>(incidence_node_id);
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
    ) -> Option<Id>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some((remove_edge_id, remove_edge)) = self.edges.remove_with_get_id(edge_id) {
            let incidence_node_ids = remove_edge.into_incidence_node_ids();
            for incidence_node_id in incidence_node_ids.iter() {
                self.delete_node_if_group_with_child::<Id>(incidence_node_id);
            }

            Some(remove_edge_id)
        } else {
            None
        }
    }
}
