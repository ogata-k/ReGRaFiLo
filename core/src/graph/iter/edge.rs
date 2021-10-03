//! Module for iterator of edge

use crate::util::Identity;

use crate::graph::as_model::AsEdgeModel;
use crate::graph::model;
use crate::graph::store::{Edge, EdgeStore};
use std::collections::btree_map::Iter;
use std::iter::Iterator;

/// Iterator for edge
pub struct EdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        EdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for EdgeIter<'a, NodeId, EdgeId> {
    type Item = (&'a EdgeId, model::Edge<'a, NodeId, EdgeId>);
    fn next(&mut self) -> Option<Self::Item> {
        self.store_iter
            .next()
            .map(|(edge_id, edge)| (edge_id, edge.as_model()))
    }
}

/// Iterator for undirected edge
pub struct UndirectedEdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> UndirectedEdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        UndirectedEdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for UndirectedEdgeIter<'a, NodeId, EdgeId> {
    type Item = (&'a EdgeId, model::UndirectedEdge<'a, NodeId, EdgeId>);
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
pub struct DirectedEdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> DirectedEdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        DirectedEdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for DirectedEdgeIter<'a, NodeId, EdgeId> {
    type Item = (&'a EdgeId, model::DirectedEdge<'a, NodeId, EdgeId>);
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
pub struct MixedEdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> MixedEdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        MixedEdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for MixedEdgeIter<'a, NodeId, EdgeId> {
    type Item = (&'a EdgeId, model::MixedEdge<'a, NodeId, EdgeId>);
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
pub struct UndirectedHyperEdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> UndirectedHyperEdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        UndirectedHyperEdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator
    for UndirectedHyperEdgeIter<'a, NodeId, EdgeId>
{
    type Item = (&'a EdgeId, model::UndirectedHyperEdge<'a, NodeId, EdgeId>);
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
pub struct DirectedHyperEdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> DirectedHyperEdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        DirectedHyperEdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator
    for DirectedHyperEdgeIter<'a, NodeId, EdgeId>
{
    type Item = (&'a EdgeId, model::DirectedHyperEdge<'a, NodeId, EdgeId>);
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
pub struct MixedHyperEdgeIter<'a, NodeId: Identity, EdgeId: Identity> {
    store_iter: Iter<'a, EdgeId, Edge<NodeId, EdgeId>>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> MixedHyperEdgeIter<'a, NodeId, EdgeId> {
    /// create this iterator
    pub(in crate::graph) fn new(store: &'a EdgeStore<NodeId, EdgeId>) -> Self {
        MixedHyperEdgeIter {
            store_iter: store.inner_store_iter(),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> Iterator for MixedHyperEdgeIter<'a, NodeId, EdgeId> {
    type Item = (&'a EdgeId, model::MixedHyperEdge<'a, NodeId, EdgeId>);
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
