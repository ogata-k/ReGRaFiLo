//! Module of node model

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

    // ---
    // checker
    // ---
}

/// Model for Node
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node<'a, Id: Identity> {
    weight: &'a i16,
    incidences: &'a [Incidence<Id>],
}

impl<'a, Id: Identity> fmt::Display for Node<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{{weight: {}, incidences: {{", self.weight))?;
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

impl<'a, Id: Identity> NodeModel<Id> for Node<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> &i16 {
        &self.weight
    }
}

impl<'a, Id: Identity> Node<'a, Id> {
    // ---
    // constructor
    // ---

    /// create node structure
    #[inline]
    pub(crate) fn _create(weight: &'a i16, incidences: &'a [Incidence<Id>]) -> Self {
        Node { weight, incidences }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}
