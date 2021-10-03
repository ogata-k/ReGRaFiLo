//! Module for Node store

use crate::graph::store::node::{FlattenIds, Incidence, Node};
use crate::util::Identity;
use std::borrow::Borrow;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::fmt;

/// Store structure for node.
#[derive(Eq, PartialEq, Clone)]
pub(in crate::graph) struct NodeStore<NodeId: Identity, EdgeId: Identity> {
    inner: BTreeMap<NodeId, Node<NodeId, EdgeId>>,
}

impl<NodeId: Identity + fmt::Debug, EdgeId: Identity + fmt::Debug> fmt::Debug
    for NodeStore<NodeId, EdgeId>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

impl<NodeId: Identity, EdgeId: Identity> fmt::Display for NodeStore<NodeId, EdgeId> {
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

impl<NodeId: Identity, EdgeId: Identity> NodeStore<NodeId, EdgeId> {
    // ---
    // constructor
    // ---

    /// create empty store
    pub(in crate::graph) fn create() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    // ---
    // getter
    // ---

    /// get node at node_id
    pub(in crate::graph) fn get_node<B: ?Sized>(&self, node_id: &B) -> Option<&Node<NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.get(node_id)
    }

    /// get node at node_id with key
    pub(in crate::graph) fn get_node_with_key<B: ?Sized>(
        &self,
        node_id: &B,
    ) -> Option<(&NodeId, &Node<NodeId, EdgeId>)>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.get_key_value(node_id)
    }

    /// get node as mutable at node_id
    pub(in crate::graph) fn get_node_as_mut<B: ?Sized>(
        &mut self,
        node_id: &B,
    ) -> Option<&mut Node<NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(node_id)
    }

    /// get incidence edge ids for node at node_id
    pub(in crate::graph) fn get_incidence_edge_ids<B: ?Sized>(&self, node_id: &B) -> Vec<&EdgeId>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        match self.inner.get(node_id) {
            None => vec![],
            Some(node) => node
                .get_incidences()
                .iter()
                .map(|incidence| incidence.get_edge_id())
                .collect(),
        }
    }

    /// get incidence edge ids from the node to top parent and get parent node ids
    pub(in crate::graph) fn get_incidence_edge_ids_from_the_node_id_and_parent_ids(
        &self,
        node_id: &NodeId,
    ) -> (Vec<&EdgeId>, Vec<&NodeId>) {
        let mut parent_node_ids = Vec::new();
        let mut incidence_edge_ids = Vec::new();
        let mut checker = vec![node_id];

        loop {
            match checker.pop() {
                None => {
                    break;
                }
                Some(_node_id) => {
                    match self.inner.get(_node_id) {
                        None => {
                            continue;
                        }
                        Some(node) => {
                            if let Some(parent_id) = node.get_parent() {
                                parent_node_ids.push(parent_id);
                                checker.push(parent_id);
                            }

                            incidence_edge_ids.extend(
                                node.get_incidences()
                                    .iter()
                                    .map(|incidence| incidence.get_edge_id()),
                            );
                        }
                    };
                }
            }
        }

        (incidence_edge_ids, parent_node_ids)
    }

    /// get incidence edge ids for node and node's parent and grandes and it's grand and ...
    pub(in crate::graph) fn get_incidence_edge_ids_until_limit_parent_from_node<'a>(
        &'a self,
        node: &'a Node<NodeId, EdgeId>,
    ) -> Vec<&'a EdgeId> {
        let mut result = Vec::new();
        let mut checker = vec![];

        if let Some(parent_id) = node.get_parent() {
            checker.push(parent_id);
        }
        let incidence_edge_ids: Vec<&EdgeId> = node
            .get_incidences()
            .iter()
            .map(|incidence| incidence.get_edge_id())
            .collect();
        result.extend(incidence_edge_ids);

        loop {
            match checker.pop() {
                None => {
                    break;
                }
                Some(node_id) => {
                    match self.inner.get_key_value(node_id) {
                        None => {
                            continue;
                        }
                        Some((_, node)) => {
                            if let Some(parent_id) = node.get_parent() {
                                checker.push(parent_id);
                            }
                            let incidence_edge_ids: Vec<&EdgeId> = node
                                .get_incidences()
                                .iter()
                                .map(|incidence| incidence.get_edge_id())
                                .collect();

                            result.extend(incidence_edge_ids);
                        }
                    };
                }
            }
        }

        result
    }

    /// get common parent or fail
    /// if return Err(None) then node_ids is empty or node at node_id is not exist.
    pub(in crate::graph) fn get_common_parent_id_or_fail(
        &self,
        node_ids: &[NodeId],
    ) -> Result<Option<&NodeId>, Option<NodeId>> {
        if node_ids.is_empty() {
            return Err(None);
        }

        let mut ids_iter = node_ids.iter();
        let first_id = ids_iter.next();

        if let Some(node) = self.inner.get(first_id.unwrap()) {
            // parent checker with first child
            let common_parent: Option<&NodeId> = node.get_parent().as_ref();

            for node_id in ids_iter {
                match self.inner.get(node_id) {
                    Some(node) => {
                        if common_parent != node.get_parent().as_ref() {
                            // need set popped old node, but old node is not exist.
                            return Err(Some(node_id.clone()));
                        }

                        // no check incidence edges to old node, because old node not exist.
                    }
                    None => {
                        return Err(None);
                    }
                }
            }

            return Ok(common_parent);
        } else {
            return Err(None);
        }
    }

    /// Flatten parent node ids of the node at specified node_id.
    /// If not exist, return None.
    pub(in crate::graph) fn flatten_parent_ids<'a, B: ?Sized>(
        &'a self,
        node_id: &'a B,
    ) -> Result<Option<FlattenIds<'a, NodeId>>, ()>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        if let Some((node_id, node)) = self.inner.get_key_value(node_id) {
            match node.get_parent().as_ref() {
                None => {
                    return Ok(Some(FlattenIds::_create_as_point(node_id)));
                }
                Some(parent_id) => {
                    let mut acc = Vec::new();
                    let mut start_parent_id = parent_id;
                    loop {
                        acc.push(start_parent_id);

                        if let Some(parent_node) = self.inner.get(start_parent_id.borrow()) {
                            match parent_node.get_parent().as_ref() {
                                None => {
                                    break;
                                }
                                Some(parent_id) => {
                                    start_parent_id = parent_id;
                                    continue;
                                }
                            }
                        } else {
                            return Err(());
                        }
                    }

                    return Ok(Some(FlattenIds::_create_as_group(node_id, acc)));
                }
            }
        } else {
            return Ok(None);
        }
    }

    /// Flatten children node ids of the node with illegal check.
    /// If not exist, do flatten with replace children. But, if fail flatten children id then return Err.
    pub(in crate::graph) fn flatten_children_id_with_check<'a>(
        &'a self,
        parent_id: &'a Option<NodeId>,
        node_id: &'a NodeId,
        illegal_ids: &[&'a NodeId],
        use_root_check: bool,
    ) -> Result<Option<FlattenIds<'a, NodeId>>, ()> {
        if use_root_check {
            if let Some(_parent_id) = parent_id {
                if _parent_id == node_id {
                    return Err(());
                }
            }
            if illegal_ids.contains(&node_id) {
                return Err(());
            }
        }

        if let Some((node_id, node)) = self.inner.get_key_value(node_id) {
            match node {
                Node::Vertex {
                    parent: node_parent_id,
                    ..
                } => {
                    if node_parent_id != &None && node_parent_id != parent_id {
                        return Err(());
                    }

                    Ok(Some(FlattenIds::_create_as_point(node_id)))
                }
                Node::Group {
                    parent: node_parent_id,
                    children,
                    ..
                } => {
                    if node_parent_id != &None && node_parent_id != parent_id {
                        return Err(());
                    }

                    let mut acc: Vec<&'a NodeId> = Vec::new();
                    for child_node_id in children.iter() {
                        acc.extend(self.rec_flatten_children_id_with_check(
                            parent_id,
                            child_node_id,
                            &illegal_ids,
                        )?);
                    }

                    Ok(Some(FlattenIds::_create_as_group(node_id, acc)))
                }
            }
        } else {
            Ok(None)
        }
    }

    /// helper for flatten_children_id function with replace at the id.
    /// if not exist at node_id, get Node as Vertex
    fn rec_flatten_children_id_with_check<'a>(
        &'a self,
        parent_id: &'a Option<NodeId>,
        node_id: &'a NodeId,
        illegal_ids: &[&'a NodeId],
    ) -> Result<Vec<&'a NodeId>, ()> {
        if let Some(_parent_id) = parent_id {
            if _parent_id == node_id {
                return Err(());
            }
        }

        if illegal_ids.contains(&node_id) {
            return Err(());
        }

        if let Some((node_id, node)) = self.inner.get_key_value(node_id.borrow()) {
            match node {
                Node::Vertex { .. } => Ok(vec![node_id]),
                Node::Group { children, .. } => {
                    let mut result: Vec<&'a NodeId> = vec![node_id];
                    for child_id in children.iter() {
                        result.extend(self.rec_flatten_children_id_with_check(
                            parent_id,
                            child_id,
                            illegal_ids,
                        )?);
                    }
                    Ok(result)
                }
            }
        } else {
            Ok(vec![node_id])
        }
    }

    /// inner store iter
    pub(in crate::graph) fn inner_store_iter<'a>(
        &'a self,
    ) -> Iter<'a, NodeId, Node<NodeId, EdgeId>> {
        self.inner.iter()
    }

    // ---
    // setter
    // ---

    /// insert node and get old node
    pub(in crate::graph) fn insert_node(
        &mut self,
        node_id: NodeId,
        node: Node<NodeId, EdgeId>,
    ) -> Option<Node<NodeId, EdgeId>> {
        self.inner.insert(node_id, node)
    }

    /// replace node's parent id for node_ids
    pub(in crate::graph) fn replace_parent_at_ids(
        &mut self,
        parent_id: NodeId,
        node_ids: &[NodeId],
    ) {
        for node_id in node_ids.iter() {
            if let Some(node) = self.inner.get_mut(node_id) {
                node.set_parent(parent_id.clone());
            }
        }
    }

    /// replace node's children ids to new id with return replaced node
    pub(in crate::graph) fn replace_children_id_to_id<B: ?Sized>(
        &mut self,
        target_id: &B,
        from_ids: &[NodeId],
        to_id: NodeId,
    ) -> Option<&Node<NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(target_id).map(|parent| {
            parent.remove_children(from_ids);
            parent.add_child(to_id);

            // remove mutable
            let parent: &Node<NodeId, EdgeId> = parent;
            parent
        })
    }

    /// add incidence for the node
    pub(in crate::graph) fn add_incidence_to_already_exist_node(
        &mut self,
        node_id: NodeId,
        incidence: Incidence<NodeId, EdgeId>,
    ) {
        let node = self.inner.get_mut(&node_id).expect(&format!(
            "Already exist node at node id {:?}. Why not exist?",
            node_id
        ));
        node.get_incidences_as_mut().push(incidence);
    }

    /// add incidence for each node
    pub(in crate::graph) fn add_incidences_each_already_exist_node(
        &mut self,
        node_incidences: Vec<(NodeId, Incidence<NodeId, EdgeId>)>,
    ) {
        for (node_id, incidences) in node_incidences.into_iter() {
            self.add_incidence_to_already_exist_node(node_id, incidences);
        }
    }

    /// replace incidence for the node
    pub(in crate::graph) fn replace_incidence_for_already_exist_node(
        &mut self,
        node_id: NodeId,
        incidence: Incidence<NodeId, EdgeId>,
        same_edge_ids: &[EdgeId],
    ) {
        let node = self.inner.get_mut(&node_id).expect(&format!(
            "Already exist node at node id {:?}. Why not exist?",
            node_id
        ));
        let incidences = node.get_incidences_as_mut();
        incidences.retain(|_incidence| !same_edge_ids.contains(_incidence.get_edge_id()));
        incidences.push(incidence);
    }

    /// replace same edge incidence for each node
    pub(in crate::graph) fn replace_incidences_each_already_exist_node(
        &mut self,
        node_incidences: Vec<(NodeId, Incidence<NodeId, EdgeId>)>,
        same_edge_ids: &[EdgeId],
    ) {
        for (node_id, incidences) in node_incidences.into_iter() {
            self.replace_incidence_for_already_exist_node(node_id, incidences, same_edge_ids);
        }
    }

    // ---
    // checker
    // ---

    // ---
    // delete
    // ---

    /// clear all nodes
    pub(in crate::graph) fn clear(&mut self) {
        self.inner.clear();
    }

    /// clear all nodes
    pub(in crate::graph) fn clear_incidence(&mut self) {
        for node in self.inner.values_mut() {
            node.clear_incidences();
        }
    }

    /// remove and get node at node_id
    pub(in crate::graph) fn remove<B: ?Sized>(
        &mut self,
        node_id: &B,
    ) -> Option<Node<NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.remove(node_id)
    }

    /// remove and get node with node_id
    pub(in crate::graph) fn remove_with_get_id<B: ?Sized>(
        &mut self,
        node_id: &B,
    ) -> Option<(NodeId, Node<NodeId, EdgeId>)>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.remove_entry(node_id)
    }

    /// remove node's children ids
    pub(in crate::graph) fn remove_children_id<B: ?Sized>(
        &mut self,
        target_id: &B,
        from_ids: &[NodeId],
    ) -> Option<&Node<NodeId, EdgeId>>
    where
        NodeId: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(target_id).map(|parent| {
            parent.remove_children(from_ids);

            // remove mutable
            let parent: &Node<NodeId, EdgeId> = parent;
            parent
        })
    }

    /// Remove incidence edge whose edge id is in specified.
    pub(in crate::graph) fn remove_edges_by_id<B: ?Sized, C: ?Sized>(
        &mut self,
        node_id: &B,
        edge_id: &C,
    ) where
        NodeId: Borrow<B>,
        B: Identity,
        EdgeId: Borrow<C>,
        C: Identity,
    {
        if let Some(node) = self.inner.get_mut(node_id) {
            node.remove_incidence_by_id(edge_id);
        }
    }

    /// Remove incidence edge whose edge ids is in specified.
    pub(in crate::graph) fn remove_edges_by_ids(
        &mut self,
        removed_node_id_edge_id: &[(NodeId, EdgeId)],
    ) {
        for (node_id, edge_id) in removed_node_id_edge_id.iter() {
            self.remove_edges_by_id(node_id, edge_id);
        }
    }
}
