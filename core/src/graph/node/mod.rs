//! Module for edge for incidence node and it's store

mod flatten;
mod incidence;
pub mod iter;
pub mod model;

use crate::graph::node::model::NodeModel;
use crate::util::Identity;
pub use flatten::*;
pub use incidence::*;
use iter::*;
use std::borrow::Borrow;
use std::collections::btree_map::{Entry, Iter};
use std::collections::BTreeMap;
use std::fmt;
use std::mem;

/// node structure for graph
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Node<Id: Identity> {
    /// Node point
    Vertex {
        weight: i16,
        parent: Option<Id>,
        incidences: Vec<Incidence<Id>>,
    },
    /// Node group
    Group {
        weight: i16,
        parent: Option<Id>,
        children: Vec<Id>,
        incidences: Vec<Incidence<Id>>,
    },
}

impl<Id: Identity> fmt::Display for Node<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model = self.as_model();
        fmt::Display::fmt(&model, f)
    }
}

impl<Id: Identity> Node<Id> {
    // ---
    // constructor
    // ---

    /// create node point structure
    pub fn vertex() -> Self {
        Self::vertex_with_weight(1)
    }

    /// create node point structure with weight
    pub fn vertex_with_weight(weight: i16) -> Self {
        Node::Vertex {
            weight: weight,
            parent: None,
            incidences: vec![],
        }
    }

    /// create node group structure
    pub fn group(children: Vec<Id>) -> Self {
        Self::group_with_weight(1, children)
    }

    /// create node group structure with weight
    pub fn group_with_weight(weight: i16, children: Vec<Id>) -> Self {
        Node::Group {
            weight: weight,
            parent: None,
            children: children,
            incidences: vec![],
        }
    }

    /// create model as node
    #[inline]
    pub fn as_model<'a>(&'a self) -> model::Node<'a, Id> {
        model::Node::_create(&self)
    }

    /// create model as node point
    #[inline]
    pub fn as_vertex_model<'a>(&'a self) -> Option<model::VertexNode<'a, Id>> {
        match self {
            Node::Vertex {
                weight,
                parent,
                incidences,
            } => Some(model::VertexNode::_create(weight, parent, incidences)),
            _ => None,
        }
    }

    /// create model as node group
    #[inline]
    pub fn as_group_model<'a>(&'a self) -> Option<model::GroupNode<'a, Id>> {
        match self {
            Node::Group {
                weight,
                parent,
                children,
                incidences,
            } => Some(model::GroupNode::_create(
                weight, parent, children, incidences,
            )),
            _ => None,
        }
    }

    // ---
    // getter
    // ---

    /// get weight for the node
    pub fn get_weight(&self) -> i16 {
        self.as_model().get_weight()
    }

    /// get weight for the node
    pub fn get_kind(&self) -> model::NodeKind {
        self.as_model().get_kind()
    }

    /// get parent node_id for the node
    pub fn get_parent(&self) -> &Option<Id> {
        match &self {
            Node::Vertex { parent, .. } => parent,
            Node::Group { parent, .. } => parent,
        }
    }

    /// get count of children
    pub fn get_child_count(&self) -> usize {
        match &self {
            Node::Vertex { .. } => 0,
            Node::Group { children, .. } => children.iter().count(),
        }
    }

    /// get children. If this node is vertex node, return empty list.
    pub fn get_children_as_ref(&self) -> Vec<&Id> {
        match &self {
            Node::Vertex { .. } => Vec::new(),
            Node::Group { children, .. } => children.iter().collect(),
        }
    }

    /// get children. If this node is vertex node, return empty list.
    pub fn get_children(&self) -> &[Id] {
        match &self {
            Node::Vertex { .. } => &[],
            Node::Group { children, .. } => children.as_slice(),
        }
    }

    /// get incidences list for the node
    pub fn get_incidences(&self) -> &[Incidence<Id>] {
        match &self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        }
    }

    /// get incidences list for the node
    fn get_incidences_as_mut(&mut self) -> &mut Vec<Incidence<Id>> {
        match self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        }
    }

    /// get edge_ids from the node's incidences
    pub fn incidences_into_edge_ids(self) -> Vec<Id> {
        let incidences = match self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        };
        incidences
            .into_iter()
            .map(|incidence| match incidence {
                Incidence::Undirected { edge_id }
                | Incidence::DirectedSource { edge_id }
                | Incidence::DirectedTarget { edge_id }
                | Incidence::UndirectedHyper { edge_id }
                | Incidence::DirectedHyperSource { edge_id }
                | Incidence::DirectedHyperTarget { edge_id } => edge_id,
            })
            .collect()
    }

    /// into incidence list
    pub fn into_incidences(self) -> Vec<Incidence<Id>> {
        match self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        }
    }

    /// into pair of parent id and incidence list
    pub fn into_parent_and_incidences(self) -> (Option<Id>, Vec<Incidence<Id>>) {
        match self {
            Node::Vertex {
                parent, incidences, ..
            } => (parent, incidences),
            Node::Group {
                parent, incidences, ..
            } => (parent, incidences),
        }
    }

    // ---
    // setter
    // ---
    /// replace parent node_id
    pub fn set_parent(&mut self, parent_id: Id) -> Option<Id> {
        match self {
            Node::Vertex { parent, .. } => parent.replace(parent_id),
            Node::Group { parent, .. } => parent.replace(parent_id),
        }
    }

    /// replace parent node_id
    pub fn set_parent_optional(&mut self, parent_id: Option<Id>) -> Option<Id> {
        match self {
            Node::Vertex { parent, .. } => mem::replace(parent, parent_id),
            Node::Group { parent, .. } => mem::replace(parent, parent_id),
        }
    }

    /// set weight
    pub fn set_weight(&mut self, weight: i16) {
        use Node::*;

        match self {
            Vertex {
                weight: mut _weight,
                ..
            }
            | Group {
                weight: mut _weight,
                ..
            } => _weight = weight,
        }
    }

    /// replace incidences
    pub fn replace_incidences(&mut self, new_incidences: Vec<Incidence<Id>>) -> Vec<Incidence<Id>> {
        match self {
            Node::Vertex { incidences, .. } => mem::replace(incidences, new_incidences),
            Node::Group { incidences, .. } => mem::replace(incidences, new_incidences),
        }
    }

    /// add child if this node is group
    pub fn add_child(&mut self, new_id: Id) {
        match self {
            Node::Group {
                children: _children,
                ..
            } => {
                _children.push(new_id);
                _children.sort();
                _children.dedup();
            }
            _ => {}
        }
    }

    // ---
    // checker
    // ---
    /// check exist group which is contains me
    pub fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }

    /// check is node point
    pub fn is_vertex(&self) -> bool {
        self.as_model().is_vertex()
    }

    /// check is node group
    pub fn is_group(&self) -> bool {
        self.as_model().is_group()
    }

    // ---
    // delete
    // ---

    /// remove parent id
    pub fn remove_parent(&mut self) {
        match self {
            Node::Vertex { parent, .. } => {
                let _ = mem::replace(parent, None);
            }
            Node::Group { parent, .. } => {
                let _ = mem::replace(parent, None);
            }
        }
    }

    /// remove specified child
    pub fn remove_child<B: ?Sized>(&mut self, child_id: &B)
        where
            Id: Borrow<B>,
            B: Identity,
    {
        match self {
            Node::Group {
                children: _children,
                ..
            } => {
                _children.retain(|_child_id| _child_id.borrow() != child_id);
            }
            _ => {}
        }
    }

    /// remove specified children
    pub fn remove_children(&mut self, children: &[Id]) {
        match self {
            Node::Group {
                children: _children,
                ..
            } => {
                _children.retain(|child| !children.contains(child));
            }
            _ => {}
        }
    }

    /// delete all incidence
    pub fn clear_incidences(&mut self) {
        self.get_incidences_as_mut().clear()
    }

    /// delete incidence with same edge id and get deleted count
    pub fn remove_incidence_by_id<B: ?Sized>(&mut self, edge_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.get_incidences_as_mut().retain(|incidence| {
            // check as borrowed because of no clone.
            if incidence.get_edge_id().borrow() != edge_id {
                // retain
                true
            } else {
                // to delete
                false
            }
        });
    }
}

/// Store structure for node.
#[derive(Eq, PartialEq, Clone)]
pub struct NodeStore<Id: Identity> {
    inner: BTreeMap<Id, Node<Id>>,
}

impl<Id: Identity + fmt::Debug> fmt::Debug for NodeStore<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

impl<Id: Identity> fmt::Display for NodeStore<Id> {
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

impl<Id: Identity> NodeStore<Id> {
    // ---
    // constructor
    // ---

    /// create empty store
    pub fn create() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    // ---
    // getter
    // ---

    /// get node at node_id
    pub fn get_node<B: ?Sized>(&self, node_id: &B) -> Option<&Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get(node_id)
    }

    /// get node at node_id with key
    pub fn get_node_with_key<B: ?Sized>(&self, node_id: &B) -> Option<(&Id, &Node<Id>)>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get_key_value(node_id)
    }

    /// get node as mutable at node_id
    pub fn get_node_as_mut<B: ?Sized>(&mut self, node_id: &B) -> Option<&mut Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(node_id)
    }

    /// get incidence edge ids for node at node_id
    pub fn get_incidence_edge_ids<B: ?Sized>(&self, node_id: &B) -> Vec<&Id>
    where
        Id: Borrow<B>,
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
    pub fn get_incidence_edge_ids_from_the_node_id_and_parent_ids(
        &self,
        node_id: &Id,
    ) -> (Vec<&Id>, Vec<&Id>) {
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
    pub fn get_incidence_edge_ids_until_limit_parent_from_node<'a>(
        &'a self,
        node: &'a Node<Id>,
    ) -> Vec<&'a Id> {
        let mut result = Vec::new();
        let mut checker = vec![];

        if let Some(parent_id) = node.get_parent() {
            checker.push(parent_id);
        }
        let incidence_edge_ids: Vec<&Id> = node
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
                            let incidence_edge_ids: Vec<&Id> = node
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
    pub fn get_common_parent_id_or_fail(&self, node_ids: &[Id]) -> Result<Option<&Id>, Option<Id>> {
        if node_ids.is_empty() {
            return Err(None);
        }

        let mut ids_iter = node_ids.iter();
        let first_id = ids_iter.next();

        if let Some(node) = self.inner.get(first_id.unwrap()) {
            // parent checker with first child
            let common_parent: Option<&Id> = node.get_parent().as_ref();

            for node_id in ids_iter {
                match self.inner.get(node_id) {
                    Some(node) => {
                        if common_parent != node.get_parent().as_ref() {
                            // need set poped old node, but old node is not exist.
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
    pub fn flatten_parent_ids<'a, B: ?Sized>(
        &'a self,
        node_id: &'a B,
    ) -> Result<Option<FlattenIds<'a, Id>>, ()>
    where
        Id: Borrow<B>,
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
    pub fn flatten_children_id_with_check<'a>(
        &'a self,
        parent_id: &'a Option<Id>,
        node_id: &'a Id,
        illegal_ids: &[&'a Id],
        use_root_check: bool,
    ) -> Result<Option<FlattenIds<'a, Id>>, ()> {
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

                    let mut acc: Vec<&'a Id> = Vec::new();
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
        parent_id: &'a Option<Id>,
        node_id: &'a Id,
        illegal_ids: &[&'a Id],
    ) -> Result<Vec<&'a Id>, ()> {
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
                    let mut result: Vec<&'a Id> = vec![node_id];
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
    pub fn inner_store_iter<'a>(&'a self) -> Iter<'a, Id, Node<Id>> {
        self.inner.iter()
    }

    /// to iterator for node
    pub fn node_iter<'a>(&'a self) -> NodeIter<'a, Id> {
        NodeIter::new(self)
    }

    /// to iterator for node point
    pub fn vertex_node_iter<'a>(&'a self) -> VertexNodeIter<'a, Id> {
        VertexNodeIter::new(self)
    }

    /// to iterator for node group
    pub fn group_node_iter<'a>(&'a self) -> GroupNodeIter<'a, Id> {
        GroupNodeIter::new(self)
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
        GroupChildNodeIter::new(group_id, &self)
    }

    // ---
    // setter
    // ---

    /// insert node and get old node
    pub fn insert_node(&mut self, node_id: Id, node: Node<Id>) -> Option<Node<Id>> {
        self.inner.insert(node_id, node)
    }

    /// replace node's parent id for node_ids
    pub fn replace_parent_at_ids(&mut self, parent_id: Id, node_ids: &[Id]) {
        for node_id in node_ids.iter() {
            if let Some(node) = self.inner.get_mut(node_id) {
                node.set_parent(parent_id.clone());
            }
        }
    }

    /// replace node's children ids to new id with return replaced node
    pub fn replace_children_id_to_id<B: ?Sized>(
        &mut self,
        target_id: &B,
        from_ids: &[Id],
        to_id: Id,
    ) -> Option<&Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(target_id).map(|parent| {
            parent.remove_children(from_ids);
            parent.add_child(to_id);

            // remove mutable
            let parent: &Node<Id> = parent;
            parent
        })
    }

    /// add incidence for the node
    pub fn add_incidence_to_already_exist_node(&mut self, node_id: Id, incidence: Incidence<Id>) {
        let node = self.inner.get_mut(&node_id).expect(&format!(
            "Already exist node at node id {:?}. Why not exist?",
            node_id
        ));
        node.get_incidences_as_mut().push(incidence);
    }

    /// add incidence for each node
    pub fn add_incidences_each_already_exist_node(
        &mut self,
        node_incidences: Vec<(Id, Incidence<Id>)>,
    ) {
        for (node_id, incidences) in node_incidences.into_iter() {
            self.add_incidence_to_already_exist_node(node_id, incidences);
        }
    }

    /// replace incidence for the node
    pub fn replace_incidence_for_already_exist_node(
        &mut self,
        node_id: Id,
        incidence: Incidence<Id>,
        same_edge_ids: &[Id],
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
    pub fn replace_incidences_each_already_exist_node(
        &mut self,
        node_incidences: Vec<(Id, Incidence<Id>)>,
        same_edge_ids: &[Id],
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
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// clear all nodes
    pub fn clear_incidence(&mut self) {
        for node in self.inner.values_mut() {
            node.clear_incidences();
        }
    }

    /// remove and get node at node_id
    pub fn remove<B: ?Sized>(&mut self, node_id: &B) -> Option<Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove(node_id)
    }

    /// remove and get node with node_id
    pub fn remove_with_get_id<B: ?Sized>(&mut self, node_id: &B) -> Option<(Id, Node<Id>)>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.remove_entry(node_id)
    }

    /// remove node's children ids
    pub fn remove_children_id<B: ?Sized>(
        &mut self,
        target_id: &B,
        from_ids: &[Id],
    ) -> Option<&Node<Id>>
    where
        Id: Borrow<B>,
        B: Identity,
    {
        self.inner.get_mut(target_id).map(|parent| {
            parent.remove_children(from_ids);

            // remove mutable
            let parent: &Node<Id> = parent;
            parent
        })
    }

    /// Remove incidence edge whose edge id is in specified.
    pub fn remove_edges_by_id<B: ?Sized>(&mut self, node_id: &B, edge_id: &B)
    where
        Id: Borrow<B>,
        B: Identity,
    {
        if let Some(node) = self.inner.get_mut(node_id) {
            node.remove_incidence_by_id(edge_id);
        }
    }

    /// Remove incidence edge whose edge ids is in specified.
    pub fn remove_edges_by_ids(&mut self, removed_node_id_edge_id: &[(Id, Id)]) {
        for (node_id, edge_id) in removed_node_id_edge_id.iter() {
            self.remove_edges_by_id(node_id, edge_id);
        }
    }
}
