//! Module for Node item

use crate::util::Identity;
use std::borrow::Borrow;
use std::fmt;
use std::mem;

/// incidence types to node
#[derive(Debug, Eq, PartialEq, Clone)]
pub(in crate::graph) enum Incidence<Id: Identity> {
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
    pub(in crate::graph) fn undirected(edge_id: Id) -> Self {
        Self::Undirected { edge_id: edge_id }
    }

    /// constructor for directed edge's incidence for source node
    pub(in crate::graph) fn directed_source(edge_id: Id) -> Self {
        Self::DirectedSource { edge_id: edge_id }
    }

    /// constructor for directed edge's incidence for target node
    pub(in crate::graph) fn directed_target(edge_id: Id) -> Self {
        Self::DirectedTarget { edge_id: edge_id }
    }

    /// constructor for undirected hyper edge's incidence
    pub(in crate::graph) fn undirected_hyper(edge_id: Id) -> Self {
        Self::UndirectedHyper { edge_id: edge_id }
    }

    /// constructor for directed hyper edge's incidence for source node
    pub(in crate::graph) fn directed_hyper_source(edge_id: Id) -> Self {
        Self::DirectedHyperSource { edge_id: edge_id }
    }

    /// constructor for directed hyper edge's incidence for target node
    pub(in crate::graph) fn directed_hyper_target(edge_id: Id) -> Self {
        Self::DirectedHyperTarget { edge_id: edge_id }
    }

    // ---
    // getter
    // ---

    /// get edge_id for the incidence edge
    pub(in crate::graph) fn get_edge_id(&self) -> &Id {
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
    pub(in crate::graph) fn is_undirected(&self) -> bool {
        if let Self::Undirected { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed edge which connect to node as source node
    pub(in crate::graph) fn is_directed_source(&self) -> bool {
        if let Self::DirectedSource { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed edge which connect to node as target node
    pub(in crate::graph) fn is_directed_target(&self) -> bool {
        if let Self::DirectedTarget { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed edge which connect to node as source or target node
    pub(in crate::graph) fn is_directed(&self) -> bool {
        match self {
            Self::DirectedSource { .. } | Self::DirectedTarget { .. } => true,
            _ => false,
        }
    }

    /// check the incidence edge is undirected hyper edge
    pub(in crate::graph) fn is_undirected_hyper(&self) -> bool {
        if let Self::UndirectedHyper { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed hyper edge which connect to node as source node
    pub(in crate::graph) fn is_directed_hyper_source(&self) -> bool {
        if let Self::DirectedHyperSource { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed  hyper edge which connect to node as target node
    pub(in crate::graph) fn is_directed_hyper_target(&self) -> bool {
        if let Self::DirectedHyperTarget { .. } = self {
            true
        } else {
            false
        }
    }

    /// check the incidence edge is directed hyper edge which connect to node as source or target node
    pub(in crate::graph) fn is_directed_hyper(&self) -> bool {
        match self {
            Self::DirectedHyperSource { .. } | Self::DirectedHyperTarget { .. } => true,
            _ => false,
        }
    }

    // ---
    // delete
    // ---
}

/// node structure for graph
#[derive(Debug, Eq, PartialEq, Clone)]
pub(in crate::graph) enum Node<Id: Identity> {
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
        match self {
            Node::Vertex {
                weight,
                parent,
                incidences,
            } => {
                f.write_fmt(format_args!(
                    "{{type: Vertex, weight: {}, parent: {:?}, incidences: {{",
                    weight, parent
                ))?;
                let mut is_first = true;
                for incidence in incidences.iter() {
                    if is_first {
                        f.write_fmt(format_args!("{}", incidence))?;
                    } else {
                        f.write_fmt(format_args!(", {}", incidence))?;
                    }
                    is_first = false;
                }
                f.write_str("}}")
            }
            Node::Group {
                weight,
                parent,
                incidences,
                children,
            } => {
                f.write_fmt(format_args!(
                    "{{type: Group, weight: {}, parent: {:?}, children: {{",
                    weight, parent
                ))?;
                let mut is_first = true;
                for child in children.iter() {
                    if is_first {
                        f.write_fmt(format_args!("{:?}", child))?;
                    } else {
                        f.write_fmt(format_args!(", {:?}", child))?;
                    }
                    is_first = false;
                }
                f.write_str("}, incidences: {")?;
                let mut is_first = true;
                for incidence in incidences.iter() {
                    if is_first {
                        f.write_fmt(format_args!("{}", incidence))?;
                    } else {
                        f.write_fmt(format_args!(", {}", incidence))?;
                    }
                    is_first = false;
                }
                f.write_str("}}")
            }
        }
    }
}

impl<Id: Identity> Node<Id> {
    // ---
    // constructor
    // ---

    /// create node point structure
    pub(in crate::graph) fn vertex() -> Self {
        Self::vertex_with_weight(1)
    }

    /// create node point structure with weight
    pub(in crate::graph) fn vertex_with_weight(weight: i16) -> Self {
        Node::Vertex {
            weight: weight,
            parent: None,
            incidences: vec![],
        }
    }

    /// create node group structure
    pub(in crate::graph) fn group(children: Vec<Id>) -> Self {
        Self::group_with_weight(1, children)
    }

    /// create node group structure with weight
    pub(in crate::graph) fn group_with_weight(weight: i16, children: Vec<Id>) -> Self {
        Node::Group {
            weight: weight,
            parent: None,
            children: children,
            incidences: vec![],
        }
    }

    // ---
    // getter
    // ---

    /// get weight for the node
    pub(in crate::graph) fn get_weight(&self) -> i16 {
        match self {
            Node::Vertex { weight, .. } => *weight,
            Node::Group { weight, .. } => *weight,
        }
    }

    /// get parent node_id for the node
    pub(in crate::graph) fn get_parent(&self) -> &Option<Id> {
        match &self {
            Node::Vertex { parent, .. } => parent,
            Node::Group { parent, .. } => parent,
        }
    }

    /// get count of children
    pub(in crate::graph) fn get_child_count(&self) -> usize {
        match &self {
            Node::Vertex { .. } => 0,
            Node::Group { children, .. } => children.iter().count(),
        }
    }

    /// get children. If this node is vertex node, return empty list.
    pub(in crate::graph) fn get_children_as_ref(&self) -> Vec<&Id> {
        match &self {
            Node::Vertex { .. } => Vec::new(),
            Node::Group { children, .. } => children.iter().collect(),
        }
    }

    /// get children. If this node is vertex node, return empty list.
    pub(in crate::graph) fn get_children(&self) -> &[Id] {
        match &self {
            Node::Vertex { .. } => &[],
            Node::Group { children, .. } => children.as_slice(),
        }
    }

    /// get incidences list for the node
    pub(in crate::graph) fn get_incidences(&self) -> &[Incidence<Id>] {
        match &self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        }
    }

    /// get incidences list for the node
    pub(in crate::graph) fn get_incidences_as_mut(&mut self) -> &mut Vec<Incidence<Id>> {
        match self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        }
    }

    /// get edge_ids from the node's incidences
    pub(in crate::graph) fn incidences_into_edge_ids(self) -> Vec<Id> {
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
    pub(in crate::graph) fn into_incidences(self) -> Vec<Incidence<Id>> {
        match self {
            Node::Vertex { incidences, .. } => incidences,
            Node::Group { incidences, .. } => incidences,
        }
    }

    /// into pair of parent id and incidence list
    pub(in crate::graph) fn into_parent_and_incidences(self) -> (Option<Id>, Vec<Incidence<Id>>) {
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
    pub(in crate::graph) fn set_parent(&mut self, parent_id: Id) -> Option<Id> {
        match self {
            Node::Vertex { parent, .. } => parent.replace(parent_id),
            Node::Group { parent, .. } => parent.replace(parent_id),
        }
    }

    /// replace parent node_id
    pub(in crate::graph) fn set_parent_optional(&mut self, parent_id: Option<Id>) -> Option<Id> {
        match self {
            Node::Vertex { parent, .. } => mem::replace(parent, parent_id),
            Node::Group { parent, .. } => mem::replace(parent, parent_id),
        }
    }

    /// set weight
    pub(in crate::graph) fn set_weight(&mut self, weight: i16) {
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
    pub(in crate::graph) fn replace_incidences(
        &mut self,
        new_incidences: Vec<Incidence<Id>>,
    ) -> Vec<Incidence<Id>> {
        match self {
            Node::Vertex { incidences, .. } => mem::replace(incidences, new_incidences),
            Node::Group { incidences, .. } => mem::replace(incidences, new_incidences),
        }
    }

    /// add child if this node is group
    pub(in crate::graph) fn add_child(&mut self, new_id: Id) {
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
    pub(in crate::graph) fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }

    /// check is node point
    pub(in crate::graph) fn is_vertex(&self) -> bool {
        match self {
            Node::Vertex { .. } => true,
            Node::Group { .. } => false,
        }
    }

    /// check is node group
    pub(in crate::graph) fn is_group(&self) -> bool {
        match self {
            Node::Vertex { .. } => false,
            Node::Group { .. } => true,
        }
    }

    // ---
    // delete
    // ---

    /// remove parent id
    pub(in crate::graph) fn remove_parent(&mut self) {
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
    pub(in crate::graph) fn remove_child<B: ?Sized>(&mut self, child_id: &B)
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
    pub(in crate::graph) fn remove_children(&mut self, children: &[Id]) {
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
    pub(in crate::graph) fn clear_incidences(&mut self) {
        self.get_incidences_as_mut().clear()
    }

    /// delete incidence with same edge id and get deleted count
    pub(in crate::graph) fn remove_incidence_by_id<B: ?Sized>(&mut self, edge_id: &B)
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
