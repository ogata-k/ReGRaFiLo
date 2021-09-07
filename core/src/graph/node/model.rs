//! Module of node model

use crate::graph::node;
use crate::graph::node::incidence::*;
use crate::util::Identity;

use std::fmt;

/// Model trait for Node
pub trait NodeModel<Id: Identity> {
    // ---
    // getter
    // ---

    /// get weight for the edge
    fn get_weight(&self) -> &i16;

    /// get node_id for group which is contains me.
    fn get_parent(&self) -> &Option<Id>;

    // ---
    // checker
    // ---
    /// check exist group which is contains me
    fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }

    /// check is node point
    fn is_vertex(&self) -> bool;

    /// check is node group
    fn is_group(&self) -> bool;
}

/// Model for Node point
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VertexNode<'a, Id: Identity> {
    weight: &'a i16,
    parent: &'a Option<Id>,
    incidences: &'a [Incidence<Id>],
}

impl<'a, Id: Identity> fmt::Display for VertexNode<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{type: Vertex, weight: {}, parent: {:?}, incidences: {{",
            self.weight, self.parent
        ))?;
        let mut is_first = true;
        for incidence in self.incidences.iter() {
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

impl<'a, Id: Identity> NodeModel<Id> for VertexNode<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> &i16 {
        &self.weight
    }

    /// get node_id for group which is contains me.
    fn get_parent(&self) -> &Option<Id> {
        self.parent
    }

    /// check is node point
    fn is_vertex(&self) -> bool {
        true
    }

    /// check is node group
    fn is_group(&self) -> bool {
        false
    }
}

impl<'a, Id: Identity> VertexNode<'a, Id> {
    // ---
    // constructor
    // ---

    /// create node point structure
    #[inline]
    pub(crate) fn _create(
        weight: &'a i16,
        parent: &'a Option<Id>,
        incidences: &'a [Incidence<Id>],
    ) -> Self {
        VertexNode {
            weight,
            parent,
            incidences,
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for Node group
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GroupNode<'a, Id: Identity> {
    weight: &'a i16,
    parent: &'a Option<Id>,
    children: &'a [Id],
    incidences: &'a [Incidence<Id>],
}

impl<'a, Id: Identity> fmt::Display for GroupNode<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{type: Group, weight: {}, parent: {:?}, children: {{",
            self.weight, self.parent
        ))?;
        let mut is_first = true;
        for child in self.children.iter() {
            if is_first {
                f.write_fmt(format_args!("{:?}", child))?;
            } else {
                f.write_fmt(format_args!(", {:?}", child))?;
            }
            is_first = false;
        }
        f.write_str("}, incidences: {")?;
        let mut is_first = true;
        for incidence in self.incidences.iter() {
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

impl<'a, Id: Identity> NodeModel<Id> for GroupNode<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> &i16 {
        &self.weight
    }

    /// get node_id for group which is contains me.
    fn get_parent(&self) -> &Option<Id> {
        self.parent
    }

    /// check is node point
    fn is_vertex(&self) -> bool {
        false
    }

    /// check is node group
    fn is_group(&self) -> bool {
        true
    }
}

impl<'a, Id: Identity> GroupNode<'a, Id> {
    // ---
    // constructor
    // ---

    /// create node group structure
    #[inline]
    pub(crate) fn _create(
        weight: &'a i16,
        parent: &'a Option<Id>,
        children: &'a [Id],
        incidences: &'a [Incidence<Id>],
    ) -> Self {
        GroupNode {
            weight,
            parent,
            children,
            incidences,
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for Node
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Node<'a, Id: Identity> {
    Vertex(VertexNode<'a, Id>),
    Group(GroupNode<'a, Id>),
}

impl<'a, Id: Identity> fmt::Display for Node<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Node::*;

        match self {
            Vertex(n) => fmt::Display::fmt(n, f),
            Group(n) => fmt::Display::fmt(n, f),
        }
    }
}

impl<'a, Id: Identity> NodeModel<Id> for Node<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> &i16 {
        use Node::*;

        match self {
            Vertex(n) => n.get_weight(),
            Group(n) => n.get_weight(),
        }
    }

    /// get node_id for group which is contains me.
    fn get_parent(&self) -> &Option<Id> {
        use Node::*;

        match self {
            Vertex(n) => n.get_parent(),
            Group(n) => n.get_parent(),
        }
    }

    /// check is node point
    fn is_vertex(&self) -> bool {
        use Node::*;

        match self {
            Vertex(n) => n.is_vertex(),
            Group(n) => n.is_vertex(),
        }
    }

    /// check is node group
    fn is_group(&self) -> bool {
        use Node::*;

        match self {
            Vertex(n) => n.is_group(),
            Group(n) => n.is_group(),
        }
    }
}

impl<'a, Id: Identity> Node<'a, Id> {
    // ---
    // constructor
    // ---

    /// create node structure
    #[inline]
    pub(crate) fn _create(node: &'a node::Node<Id>) -> Self {
        match node {
            node::Node::Vertex {
                weight,
                parent,
                incidences,
            } => Node::Vertex(VertexNode::_create(weight, parent, incidences)),
            node::Node::Group {
                weight,
                parent,
                children,
                incidences,
            } => Node::Group(GroupNode::_create(weight, parent, children, incidences)),
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}
