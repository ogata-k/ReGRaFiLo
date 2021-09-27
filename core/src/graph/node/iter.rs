//! Module for iterator of node

use crate::graph::node::{model, NodeStore};
use crate::util::Identity;

use crate::graph::Node;
use std::borrow::Borrow;
use std::collections::btree_map::Iter;
use std::iter::Iterator;

/// Iterator for node
pub struct NodeIter<'a, Id: Identity> {
    store_iter: Iter<'a, Id, Node<Id>>,
}

impl<'a, Id: Identity> NodeIter<'a, Id> {
    /// create this iterator
    pub fn new(store: &'a NodeStore<Id>) -> Self {
        NodeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, Id: Identity> Iterator for NodeIter<'a, Id> {
    type Item = (&'a Id, model::Node<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.store_iter
            .next()
            .map(|(node_id, node)| (node_id, node.as_model()))
    }
}

/// Iterator for node point
pub struct VertexNodeIter<'a, Id: Identity> {
    store_iter: Iter<'a, Id, Node<Id>>,
}

impl<'a, Id: Identity> VertexNodeIter<'a, Id> {
    /// create this iterator
    pub fn new(store: &'a NodeStore<Id>) -> Self {
        VertexNodeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, Id: Identity> Iterator for VertexNodeIter<'a, Id> {
    type Item = (&'a Id, model::VertexNode<'a, Id>);
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
pub struct GroupNodeIter<'a, Id: Identity> {
    store_iter: Iter<'a, Id, Node<Id>>,
}

impl<'a, Id: Identity> GroupNodeIter<'a, Id> {
    /// create this iterator
    pub fn new(store: &'a NodeStore<Id>) -> Self {
        GroupNodeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, Id: Identity> Iterator for GroupNodeIter<'a, Id> {
    type Item = (&'a Id, model::GroupNode<'a, Id>);
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
pub struct GroupChildNodeIter<'a, Id: Identity> {
    group_id: Option<&'a Id>,
    is_exist_group: bool,
    group_node: Option<&'a Node<Id>>,
    specified_group_children: Option<Vec<&'a Id>>,
    store: &'a NodeStore<Id>,
    store_iter: Iter<'a, Id, Node<Id>>,
}

impl<'a, Id: Identity> GroupChildNodeIter<'a, Id> {
    /// create this iterator
    /// If specified group is root or not exist group, then return None.
    pub fn new<B: ?Sized>(group_id: Option<&'a B>, store: &'a NodeStore<Id>) -> Self
    where
        Id: Borrow<B>,
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
        let child_vec: Option<Vec<&'a Id>> = result_node.map(|group| {
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
    pub fn get_group_id(&self) -> Option<&Id> {
        if !self.is_exist_group {
            return None;
        }

        self.group_id.as_ref().map(|id| *id)
    }

    /// get group node.
    /// If specified group is root or not exist group, then return None.
    pub fn get_group(&self) -> Option<model::GroupNode<'a, Id>> {
        if !self.is_exist_group {
            return None;
        }

        self.group_node.map(|node| node.as_group_model()).flatten()
    }

    /// get group node with id.
    /// If specified Id is None as root group, then return None.
    pub fn get_group_with_id(&self) -> Option<(&'a Id, model::GroupNode<'a, Id>)> {
        match (&self.group_id, self.get_group()) {
            (Some(_group_id), Some(_group)) => Some((_group_id, _group)),
            _ => None,
        }
    }
}

impl<'a, Id: Identity> Iterator for GroupChildNodeIter<'a, Id> {
    type Item = (
        Option<(&'a Id, model::GroupNode<'a, Id>)>,
        (&'a Id, model::Node<'a, Id>),
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
