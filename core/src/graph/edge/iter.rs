//! Module for iterator of edge

use crate::graph::edge::model;
use crate::util::Identity;

use std::iter::Iterator;

/// Iterator for edge
pub struct EdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::Edge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> EdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::Edge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::Edge<'a, Id>)>,
    {
        EdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for EdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::Edge<'a, Id>)>,
{
    type Item = (&'a Id, model::Edge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator for undirected edge
pub struct UndirectedEdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::UndirectedEdge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> UndirectedEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::UndirectedEdge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::UndirectedEdge<'a, Id>)>,
    {
        UndirectedEdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for UndirectedEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::UndirectedEdge<'a, Id>)>,
{
    type Item = (&'a Id, model::UndirectedEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator for directed edge
pub struct DirectedEdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::DirectedEdge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> DirectedEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::DirectedEdge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::DirectedEdge<'a, Id>)>,
    {
        DirectedEdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for DirectedEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::DirectedEdge<'a, Id>)>,
{
    type Item = (&'a Id, model::DirectedEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator for undirected or directed edge
pub struct MixedEdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::MixedEdge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> MixedEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::MixedEdge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::MixedEdge<'a, Id>)>,
    {
        MixedEdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for MixedEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::MixedEdge<'a, Id>)>,
{
    type Item = (&'a Id, model::MixedEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator for undirected hyper edge
pub struct UndirectedHyperEdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> UndirectedHyperEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>)>,
    {
        UndirectedHyperEdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for UndirectedHyperEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>)>,
{
    type Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator for directed hyper edge
pub struct DirectedHyperEdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::DirectedHyperEdge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> DirectedHyperEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::DirectedHyperEdge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::DirectedHyperEdge<'a, Id>)>,
    {
        DirectedHyperEdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for DirectedHyperEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::DirectedHyperEdge<'a, Id>)>,
{
    type Item = (&'a Id, model::DirectedHyperEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator for undirected or directed hyper edge
pub struct MixedHyperEdgeIter<'a, Id: 'a + Identity, I>
where
    I: Iterator<Item = (&'a Id, model::MixedHyperEdge<'a, Id>)>,
{
    inner: I,
}

impl<'a, Id: 'a + Identity, I> MixedHyperEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::MixedHyperEdge<'a, Id>)>,
{
    /// create this iterator
    pub fn new(iter: I) -> Self
    where
        I: Iterator<Item = (&'a Id, model::MixedHyperEdge<'a, Id>)>,
    {
        MixedHyperEdgeIter { inner: iter }
    }
}

impl<'a, Id: 'a + Identity, I> Iterator for MixedHyperEdgeIter<'a, Id, I>
where
    I: Iterator<Item = (&'a Id, model::MixedHyperEdge<'a, Id>)>,
{
    type Item = (&'a Id, model::MixedHyperEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
