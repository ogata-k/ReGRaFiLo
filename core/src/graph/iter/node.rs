//! Module for iterator of node

use crate::graph::as_model::AsNodeModel;
use crate::graph::model;
use crate::graph::store;
use crate::graph::store::{Node, NodeStore};
use crate::util::Identity;
use std::borrow::Borrow;
use std::collections::btree_map::Iter;
use std::iter::Iterator;

/// Iterator for node
pub struct NodeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, NodeId, store::Node<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> NodeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a NodeStore<NodeId, EdgeId>) -> Self {
        NodeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for NodeIter<'a, NodeId, EdgeId> {
    type Item = (&'a NodeId, model::Node<'a, NodeId, EdgeId>);
    fn next(&mut self) -> Option<Self::Item> {
        self.store_iter
            .next()
            .map(|(node_id, node)| (node_id, node.as_model()))
    }
}

/// Iterator for node point
pub struct VertexNodeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, NodeId, Node<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> VertexNodeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a NodeStore<NodeId, EdgeId>) -> Self {
        VertexNodeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for VertexNodeIter<'a, NodeId, EdgeId> {
    type Item = (&'a NodeId, model::VertexNode<'a, NodeId, EdgeId>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((node_id, node)) => {
                    let vertex_node = node.as_vertex_model();
                    match vertex_node {
                        None => {
                            continue;
                        }
                        Some(_vertex_node) => {
                            return Some((node_id, _vertex_node));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for node grouping
pub struct GroupNodeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, NodeId, Node<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> GroupNodeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a NodeStore<NodeId, EdgeId>) -> Self {
        GroupNodeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for GroupNodeIter<'a, NodeId, EdgeId> {
    type Item = (&'a NodeId, model::GroupNode<'a, NodeId, EdgeId>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((node_id, node)) => {
                    let group_node = node.as_group_model();
                    match group_node {
                        None => {
                            continue;
                        }
                        Some(_group_node) => {
                            return Some((node_id, _group_node));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for grouping child node
#[derive(Debug)]
pub struct GroupChildNodeIter<'a, NodeId: Identity, EdgeId: Identity> {
    group_id: Option<&'a NodeId>,
    is_exist_group: bool,
    group_node: Option<&'a Node<NodeId, EdgeId>>,
    specified_group_children: Option<Vec<&'a NodeId>>,
    store: &'a NodeStore<NodeId, EdgeId>,
    store_iter: Iter<'a, NodeId, Node<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> GroupChildNodeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    /// If specified group is root or not exist group, then return None.
    pub(in crate::graph) fn new<B: ?Sized>(
        group_id: Option<&'a B>,
        store: &'a NodeStore<NodeId, EdgeId>,
    ) -> Self
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        // get group
        let group_result = match group_id {
            None => None,
            Some(_group_id) => {
                // get item with key because fix exist node
                store.get_node_with_key(_group_id)
            }
        };
        let result_id = group_result.map(|(id, _)| id);
        let result_node = group_result.map(|(_, node)| node);

        // collect group info
        let group_model = group_result
            .map(|(_, node)| node.as_group_model())
            .flatten();
        // exist flag for root group or specified group
        let is_exist_group = group_id.is_none() || (group_id.is_some() && group_model.is_some());
        let child_vec: Option<Vec<&'a NodeId>> = result_node.map(|group| {
            group
                .get_children_as_ref()
                .iter()
                .map(|child_id| *child_id)
                .collect()
        });

        GroupChildNodeIter {
            group_id: result_id,
            is_exist_group,
            group_node: result_node,
            specified_group_children: child_vec,
            store,
            store_iter: store.inner_store_iter(),
        }
    }

    /// specified group is exist
    pub fn is_exist_group(&self) -> bool {
        self.is_exist_group
    }

    /// specified group is root
    pub fn is_root_group(&self) -> bool {
        self.is_exist_group && self.group_id.is_none() && self.group_node.is_none()
    }

    /// get group id.
    pub fn get_group_id(&self) -> Option<&NodeId> {
        if !self.is_exist_group {
            return None;
        }

        self.group_id.as_ref().map(|id| *id)
    }

    /// get group node.
    /// If specified group is root or not exist group, then return None.
    pub fn get_group(&self) -> Option<model::GroupNode<'a, NodeId, EdgeId>> {
        if !self.is_exist_group {
            return None;
        }

        self.group_node.map(|node| node.as_group_model()).flatten()
    }

    /// get group node with id.
    /// If specified Id is None as root group, then return None.
    pub fn get_group_with_id(&self) -> Option<(&'a NodeId, model::GroupNode<'a, NodeId, EdgeId>)> {
        match (&self.group_id, self.get_group()) {
            (Some(_group_id), Some(_group)) => Some((_group_id, _group)),
            _ => None,
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for GroupChildNodeIter<'a, NodeId, EdgeId> {
    type Item = (
        Option<(&'a NodeId, model::GroupNode<'a, NodeId, EdgeId>)>,
        (&'a NodeId, model::Node<'a, NodeId, EdgeId>),
    );
    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_exist_group {
            return None;
        }

        loop {
            match self.specified_group_children.as_mut() {
                // specified root group
                None => {
                    let iter_next = self.store_iter.next();
                    match iter_next {
                        None => {
                            return None;
                        }
                        Some(_iter_next) => {
                            let converted_child =
                                if _iter_next.1.get_parent().as_ref().map(|id| id.borrow())
                                    == self.group_id
                                {
                                    Some((_iter_next.0, _iter_next.1.as_model()))
                                } else {
                                    None
                                };
                            if converted_child.is_none() {
                                continue;
                            }

                            let group_model =
                                self.group_node.map(|node| node.as_group_model()).flatten();
                            match (&self.group_id, group_model, converted_child) {
                                (None, None, Some(node)) => {
                                    return Some((None, node));
                                }
                                (Some(_group_id), Some(_group), Some(node)) => {
                                    return Some((Some((_group_id, _group)), node));
                                }
                                _ => {
                                    return None;
                                }
                            }
                        }
                    }
                }
                // specified not root group
                Some(children) => {
                    let child_id = if children.is_empty() {
                        None
                    } else {
                        Some(children.remove(0))
                    };
                    match child_id {
                        None => {
                            return None;
                        }
                        Some(_child_id) => {
                            let child = self.store.get_node_with_key(&_child_id);
                            match child {
                                None => {
                                    return None;
                                }
                                Some(_child) => {
                                    let converted_child =
                                        if _child.1.get_parent().as_ref().map(|id| id.borrow())
                                            == self.group_id
                                        {
                                            Some((_child.0, _child.1.as_model()))
                                        } else {
                                            None
                                        };
                                    if converted_child.is_none() {
                                        continue;
                                    }

                                    let group_model =
                                        self.group_node.map(|node| node.as_group_model()).flatten();
                                    match (&self.group_id, group_model, converted_child) {
                                        (None, None, Some(node)) => {
                                            return Some((None, node));
                                        }
                                        (Some(_group_id), Some(_group), Some(node)) => {
                                            return Some((Some((_group_id, _group)), node));
                                        }
                                        _ => {
                                            return None;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
        }
    }
}
