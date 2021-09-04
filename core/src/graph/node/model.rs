//! Module of node model

use crate::graph::node;
use crate::util::Identity;

use std::fmt;

/// Model for Node
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node<'a, Id: Identity> {
    inner: &'a node::Node<Id>,
}

impl<'a, Id: Identity> fmt::Display for Node<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.inner, f)
    }
}

impl<'a, Id: Identity> Node<'a, Id> {
    // ---
    // constructor
    // ---

    /// create node structure
    #[inline]
    pub(crate) fn _create(node: &'a node::Node<Id>) -> Self {
        Node { inner: node }
    }

    // ---
    // getter
    // ---

    /// get weight for the node
    pub fn get_weight(&self) -> &i16 {
        &self.inner.get_weight()
    }

    // ---
    // checker
    // ---
}
