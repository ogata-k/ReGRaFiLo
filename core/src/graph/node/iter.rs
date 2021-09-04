//! Module for iterator of node

use crate::graph::node::model;
use crate::util::Identity;

use std::iter::Iterator;

/// Iterator for node
pub struct NodeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::Node<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> NodeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::Node<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::Node<'a, Id>)>,
    {
        NodeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for NodeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::Node<'a, Id>)>,
{
    type Item = (&'a Id, model::Node<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
