//! Module of edge model

use crate::util::{Identity, Weight};

use std::fmt;
use std::marker::PhantomData;

/// Kind of Edge model
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EdgeKind {
    /// Kind for Undirected edge
    Undirected,
    /// Kind for Directed edge
    Directed,
    /// Kind for Undirected hyper edge
    UndirectedHyper,
    /// Kind for Directed hyper edge
    DirectedHyper,
}

impl EdgeKind {
    /// check edge is undirected edge
    pub fn is_undirected(&self) -> bool {
        self == &EdgeKind::Undirected
    }

    /// check edge is directed edge
    pub fn is_directed(&self) -> bool {
        self == &EdgeKind::Directed
    }

    /// check edge is undirected or directed edge
    pub fn is_edge(&self) -> bool {
        self.is_undirected() || self.is_directed()
    }

    /// check edge is undirected hyper edge
    pub fn is_undirected_hyper(&self) -> bool {
        self == &EdgeKind::UndirectedHyper
    }

    /// check edge is directed hyper edge
    pub fn is_directed_hyper(&self) -> bool {
        self == &EdgeKind::DirectedHyper
    }

    /// check edge is undirected or directed hyper edge
    pub fn is_hyper_edge(&self) -> bool {
        self.is_undirected_hyper() || self.is_directed_hyper()
    }
}

/// Model trait for Edge
pub trait EdgeModel<NodeId: Identity, EdgeId: Identity> {
    // ---
    // getter
    // ---

    /// get weight for the edge.
    fn get_weight(&self) -> Weight;

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind;

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId>;

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId>;

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId>;

    // ---
    // checker
    // ---

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool;

    /// check edge is undirected edge
    fn is_undirected(&self) -> bool {
        self.get_kind().is_undirected()
    }

    /// check edge is directed edge
    fn is_directed(&self) -> bool {
        self.get_kind().is_directed()
    }

    /// check edge is undirected or directed edge
    fn is_edge(&self) -> bool {
        self.get_kind().is_edge()
    }

    /// check edge is undirected hyper edge
    fn is_undirected_hyper(&self) -> bool {
        self.get_kind().is_undirected_hyper()
    }

    /// check edge is directed hyper edge
    fn is_directed_hyper(&self) -> bool {
        self.get_kind().is_directed_hyper()
    }

    /// check edge is undirected or directed hyper edge
    fn is_hyper_edge(&self) -> bool {
        self.get_kind().is_hyper_edge()
    }
}

/// Model for undirected edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UndirectedEdge<'a, NodeId: Identity, EdgeId: Identity> {
    pub(in crate::graph) weight: &'a Weight,
    pub(in crate::graph) incidence: &'a [NodeId; 2],
    pub(in crate::graph) _edge_id: PhantomData<EdgeId>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display for UndirectedEdge<'a, NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{weight: {}, link: {:?}--{:?}}}",
            self.weight, self.incidence[0], self.incidence[1]
        ))
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for UndirectedEdge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        EdgeKind::Undirected
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        Vec::new()
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        Vec::new()
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        self.incidence.iter().collect()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

/// Model for directed edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DirectedEdge<'a, NodeId: Identity, EdgeId: Identity> {
    pub(in crate::graph) weight: &'a Weight,
    pub(in crate::graph) incidence: (&'a NodeId, &'a NodeId),
    pub(in crate::graph) _edge_id: PhantomData<EdgeId>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display for DirectedEdge<'a, NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{weight: {}, link: {:?}->{:?}}}",
            self.weight, self.incidence.0, self.incidence.1
        ))
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for DirectedEdge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        EdgeKind::Directed
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        vec![self.incidence.0]
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        vec![self.incidence.1]
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        Vec::new()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

/// Model for edge or hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MixedEdge<'a, NodeId: Identity, EdgeId: Identity> {
    Undirected(UndirectedEdge<'a, NodeId, EdgeId>),
    Directed(DirectedEdge<'a, NodeId, EdgeId>),
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display for MixedEdge<'a, NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MixedEdge::*;

        match self {
            Undirected(e) => fmt::Display::fmt(e, f),
            Directed(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for MixedEdge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_weight(),
            Directed(e) => e.get_weight(),
        }
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_kind(),
            Directed(e) => e.get_kind(),
        }
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_source_ids(),
            Directed(e) => e.get_source_ids(),
        }
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_target_ids(),
            Directed(e) => e.get_target_ids(),
        }
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_source_target_ids(),
            Directed(e) => e.get_source_target_ids(),
        }
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        use MixedEdge::*;

        match (self, other) {
            (Undirected(self_edge), Undirected(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            (Directed(self_edge), Directed(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            _ => false,
        }
    }
}

/// Model for undirected hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UndirectedHyperEdge<'a, NodeId: Identity, EdgeId: Identity> {
    pub(in crate::graph) weight: &'a Weight,
    pub(in crate::graph) incidence: &'a [NodeId],
    pub(in crate::graph) _edge_id: PhantomData<EdgeId>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display
    for UndirectedHyperEdge<'a, NodeId, EdgeId>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{{weight: {}, link: ", self.weight))?;
        f.debug_set().entries(self.incidence.iter()).finish()?;
        f.write_str("}")
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for UndirectedHyperEdge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        EdgeKind::UndirectedHyper
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        Vec::new()
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        Vec::new()
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        self.incidence.iter().collect()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

/// Model for directed hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DirectedHyperEdge<'a, NodeId: Identity, EdgeId: Identity> {
    pub(in crate::graph) weight: &'a Weight,
    pub(in crate::graph) incidence: (&'a [NodeId], &'a [NodeId]),
    pub(in crate::graph) _edge_id: PhantomData<EdgeId>,
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display
    for DirectedHyperEdge<'a, NodeId, EdgeId>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{{weight: {}, link: ", self.weight))?;
        f.debug_set().entries(self.incidence.0.iter()).finish()?;
        f.write_str("->")?;
        f.debug_set().entries(self.incidence.1.iter()).finish()?;
        f.write_str("}")
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for DirectedHyperEdge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        EdgeKind::DirectedHyper
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        self.incidence.0.iter().collect()
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        self.incidence.1.iter().collect()
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        Vec::new()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

/// Model for edge or hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MixedHyperEdge<'a, NodeId: Identity, EdgeId: Identity> {
    Undirected(UndirectedHyperEdge<'a, NodeId, EdgeId>),
    Directed(DirectedHyperEdge<'a, NodeId, EdgeId>),
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display for MixedHyperEdge<'a, NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => fmt::Display::fmt(e, f),
            Directed(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for MixedHyperEdge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_weight(),
            Directed(e) => e.get_weight(),
        }
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_kind(),
            Directed(e) => e.get_kind(),
        }
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_source_ids(),
            Directed(e) => e.get_source_ids(),
        }
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_target_ids(),
            Directed(e) => e.get_target_ids(),
        }
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_source_target_ids(),
            Directed(e) => e.get_source_target_ids(),
        }
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        use MixedHyperEdge::*;

        match (self, other) {
            (Undirected(self_edge), Undirected(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            (Directed(self_edge), Directed(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            _ => false,
        }
    }
}

/// Model for edge or hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Edge<'a, NodeId: Identity, EdgeId: Identity> {
    Undirected(UndirectedEdge<'a, NodeId, EdgeId>),
    Directed(DirectedEdge<'a, NodeId, EdgeId>),
    UndirectedHyper(UndirectedHyperEdge<'a, NodeId, EdgeId>),
    DirectedHyper(DirectedHyperEdge<'a, NodeId, EdgeId>),
}

impl<'a, NodeId: Identity, EdgeId: Identity> fmt::Display for Edge<'a, NodeId, EdgeId> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Edge::*;

        match self {
            Undirected(e) => fmt::Display::fmt(e, f),
            Directed(e) => fmt::Display::fmt(e, f),
            UndirectedHyper(e) => fmt::Display::fmt(e, f),
            DirectedHyper(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<'a, NodeId: Identity, EdgeId: Identity> EdgeModel<NodeId, EdgeId>
    for Edge<'a, NodeId, EdgeId>
{
    /// get weight for the edge
    fn get_weight(&self) -> Weight {
        use Edge::*;

        match self {
            Undirected(e) => e.get_weight(),
            Directed(e) => e.get_weight(),
            UndirectedHyper(e) => e.get_weight(),
            DirectedHyper(e) => e.get_weight(),
        }
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind {
        use Edge::*;

        match self {
            Undirected(e) => e.get_kind(),
            Directed(e) => e.get_kind(),
            UndirectedHyper(e) => e.get_kind(),
            DirectedHyper(e) => e.get_kind(),
        }
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&NodeId> {
        use Edge::*;

        match self {
            Undirected(e) => e.get_source_ids(),
            Directed(e) => e.get_source_ids(),
            UndirectedHyper(e) => e.get_source_ids(),
            DirectedHyper(e) => e.get_source_ids(),
        }
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&NodeId> {
        use Edge::*;

        match self {
            Undirected(e) => e.get_target_ids(),
            Directed(e) => e.get_target_ids(),
            UndirectedHyper(e) => e.get_target_ids(),
            DirectedHyper(e) => e.get_target_ids(),
        }
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&NodeId> {
        use Edge::*;

        match self {
            Undirected(e) => e.get_source_target_ids(),
            Directed(e) => e.get_source_target_ids(),
            UndirectedHyper(e) => e.get_source_target_ids(),
            DirectedHyper(e) => e.get_source_target_ids(),
        }
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        use Edge::*;

        match (self, other) {
            (Undirected(self_edge), Undirected(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            (Directed(self_edge), Directed(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            (UndirectedHyper(self_edge), UndirectedHyper(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            (DirectedHyper(self_edge), DirectedHyper(other_edge)) => {
                self_edge.is_equal_to_without_weight(other_edge)
            }
            _ => false,
        }
    }
}
