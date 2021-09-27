//! Module for iterator of edge

use crate::graph::edge::{model, Edge, EdgeStore};
use crate::util::Identity;

use std::iter::Iterator;
use std::collections::btree_map::Iter;

/// Iterator for edge
pub struct EdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> EdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        EdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for EdgeIter<'a, Id>
{
    type Item = (&'a Id, model::Edge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        self.store_iter
            .next()
            .map(|(edge_id, edge)| (edge_id, edge.as_model()))
    }
}

/// Iterator for undirected edge
pub struct UndirectedEdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> UndirectedEdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        UndirectedEdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for UndirectedEdgeIter<'a, Id>
{
    type Item = (&'a Id, model::UndirectedEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((edge_id, edge)) => {
                    let undirected_edge = edge.as_undirected_model();
                    match undirected_edge {
                        None => {
                            continue;
                        }
                        Some(_undirected_edge) => {
                            return Some((edge_id, _undirected_edge));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for directed edge
pub struct DirectedEdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> DirectedEdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        DirectedEdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for DirectedEdgeIter<'a, Id>
{
    type Item = (&'a Id, model::DirectedEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((edge_id, edge)) => {
                    let directed_edge = edge.as_directed_model();
                    match directed_edge {
                        None => {
                            continue;
                        }
                        Some(_directed_edge) => {
                            return Some((edge_id, _directed_edge));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for undirected or directed edge
pub struct MixedEdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> MixedEdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        MixedEdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for MixedEdgeIter<'a, Id>
{
    type Item = (&'a Id, model::MixedEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((edge_id, edge)) => {
                    let mixed_edge = edge.as_mixed_model();
                    match mixed_edge {
                        None => {
                            continue;
                        }
                        Some(_mixed_edge) => {
                            return Some((edge_id, _mixed_edge));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for undirected hyper edge
pub struct UndirectedHyperEdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> UndirectedHyperEdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        UndirectedHyperEdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for UndirectedHyperEdgeIter<'a, Id>
{
    type Item = (&'a Id, model::UndirectedHyperEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((edge_id, edge)) => {
                    let undirected_hyper_edge = edge.as_undirected_hyper_model();
                    match undirected_hyper_edge {
                        None => {
                            continue;
                        }
                        Some(_undirected_hyper_edge) => {
                            return Some((edge_id, _undirected_hyper_edge));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for directed hyper edge
pub struct DirectedHyperEdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> DirectedHyperEdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        DirectedHyperEdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for DirectedHyperEdgeIter<'a, Id>
{
    type Item = (&'a Id, model::DirectedHyperEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((edge_id, edge)) => {
                    let directed_hyper_edge = edge.as_directed_hyper_model();
                    match directed_hyper_edge {
                        None => {
                            continue;
                        }
                        Some(_directed_hyper_edge) => {
                            return Some((edge_id, _directed_hyper_edge));
                        }
                    }
                }
            }
        }
    }
}

/// Iterator for undirected or directed hyper edge
pub struct MixedHyperEdgeIter<'a, Id:  Identity>
{
    store_iter: Iter<'a, Id, Edge<Id>>,
}

impl<'a, Id:  Identity> MixedHyperEdgeIter<'a, Id>
{
    /// create this iterator
    pub fn new(store: &'a EdgeStore<Id>) -> Self
    {
        MixedHyperEdgeIter { store_iter: store.inner_store_iter() }
    }
}

impl<'a, Id:  Identity> Iterator for MixedHyperEdgeIter<'a, Id>
{
    type Item = (&'a Id, model::MixedHyperEdge<'a, Id>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.store_iter.next() {
                None => {
                    return None;
                }
                Some((edge_id, edge)) => {
                    let mixed_hyper_edge = edge.as_mixed_hyper_model();
                    match mixed_hyper_edge {
                        None => {
                            continue;
                        }
                        Some(_mixed_hyper_edge) => {
                            return Some((edge_id, _mixed_hyper_edge));
                        }
                    }
                }
            }
        }
    }
}
