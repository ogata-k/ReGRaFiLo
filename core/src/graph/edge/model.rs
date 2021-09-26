//! Module of edge model

use crate::graph::edge;
use crate::util::Identity;

use std::fmt;
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
   pub fn is_undirected(&self) -> bool{
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
    pub fn is_undirected_hyper(&self) -> bool{
        self == &EdgeKind::UndirectedHyper
    }

    /// check edge is directed hyper edge
    pub fn is_directed_hyper(&self) -> bool{
        self == &EdgeKind::DirectedHyper
    }

    /// check edge is undirected or directed hyper edge
    pub fn is_hyper_edge(&self) -> bool {
        self.is_undirected_hyper() || self.is_directed_hyper()
    }
}

/// Model trait for Edge
pub trait EdgeModel<Id: Identity> {
    // ---
    // getter
    // ---

    /// get weight for the edge.
    fn get_weight(&self) -> i16;

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind;

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id>;

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id>;

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id>;

    // ---
    // checker
    // ---

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool;

    /// check edge is undirected edge
    fn is_undirected(&self) -> bool{
        self.get_kind().is_undirected()
    }

    /// check edge is directed edge
    fn is_directed(&self) -> bool{
        self.get_kind().is_directed()
    }

    /// check edge is undirected or directed edge
    fn is_edge(&self) -> bool {
        self.get_kind().is_edge()
    }

    /// check edge is undirected hyper edge
    fn is_undirected_hyper(&self) -> bool{
        self.get_kind().is_undirected_hyper()
    }

    /// check edge is directed hyper edge
    fn is_directed_hyper(&self) -> bool{
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
pub struct UndirectedEdge<'a, Id: Identity> {
    weight: &'a i16,
    incidence: &'a [Id; 2],
}

impl<'a, Id: Identity> fmt::Display for UndirectedEdge<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{weight: {}, link: {:?}--{:?}}}",
            self.weight, self.incidence[0], self.incidence[1]
        ))
    }
}

impl<'a, Id: Identity> EdgeModel<Id> for UndirectedEdge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
        EdgeKind::Undirected
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id> {
        Vec::new()
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id> {
        Vec::new()
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id> {
        self.incidence.iter().collect()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

impl<'a, Id: Identity> UndirectedEdge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create undirected edge structure
    #[inline]
    pub(crate) fn _create(weight: &'a i16, incidence: &'a [Id; 2]) -> Self {
        UndirectedEdge { weight, incidence }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for directed edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DirectedEdge<'a, Id: Identity> {
    weight: &'a i16,
    incidence: (&'a Id, &'a Id),
}

impl<'a, Id: Identity> fmt::Display for DirectedEdge<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{{weight: {}, link: {:?}->{:?}}}",
            self.weight, self.incidence.0, self.incidence.1
        ))
    }
}

impl<'a, Id: Identity> EdgeModel<Id> for DirectedEdge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
        EdgeKind::Directed
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id> {
        vec![self.incidence.0]
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id> {
        vec![self.incidence.1]
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id> {
        Vec::new()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

impl<'a, Id: Identity> DirectedEdge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create directed edge structure
    #[inline]
    pub(crate) fn _create(weight: &'a i16, source: &'a Id, target: &'a Id) -> Self {
        DirectedEdge {
            weight,
            incidence: (source, target),
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for edge or hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MixedEdge<'a, Id: Identity> {
    Undirected(UndirectedEdge<'a, Id>),
    Directed(DirectedEdge<'a, Id>),
}

impl<'a, Id: Identity> fmt::Display for MixedEdge<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MixedEdge::*;

        match self {
            Undirected(e) => fmt::Display::fmt(e, f),
            Directed(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<'a, Id: Identity> EdgeModel<Id> for MixedEdge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_weight(),
            Directed(e) => e.get_weight(),
        }
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_kind(),
            Directed(e) => e.get_kind(),
        }
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id> {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_source_ids(),
            Directed(e) => e.get_source_ids(),
        }
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id> {
        use MixedEdge::*;

        match self {
            Undirected(e) => e.get_target_ids(),
            Directed(e) => e.get_target_ids(),
        }
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id> {
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

impl<'a, Id: Identity> MixedEdge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create edge structure
    #[inline]
    pub(crate) fn _create(edge: &'a edge::Edge<Id>) -> Option<Self> {
        match edge {
            edge::Edge::Undirected { weight, ids } => {
                Some(MixedEdge::Undirected(UndirectedEdge::_create(weight, ids)))
            }
            edge::Edge::Directed {
                weight,
                source_id,
                target_id,
            } => Some(MixedEdge::Directed(DirectedEdge::_create(
                weight, source_id, target_id,
            ))),
            edge::Edge::UndirectedHyper { .. } | edge::Edge::DirectedHyper { .. } => None,
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for undirected hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UndirectedHyperEdge<'a, Id: Identity> {
    weight: &'a i16,
    incidence: &'a [Id],
}

impl<'a, Id: Identity> fmt::Display for UndirectedHyperEdge<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{{weight: {}, link: ", self.weight))?;
        f.debug_set().entries(self.incidence.iter()).finish()?;
        f.write_str("}")
    }
}

impl<'a, Id: Identity> EdgeModel<Id> for UndirectedHyperEdge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
        EdgeKind::UndirectedHyper
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id> {
        Vec::new()
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id> {
        Vec::new()
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id> {
        self.incidence.iter().collect()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

impl<'a, Id: Identity> UndirectedHyperEdge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create undirected hyper edge structure
    #[inline]
    pub(crate) fn _create(weight: &'a i16, incidence: &'a [Id]) -> Self {
        UndirectedHyperEdge { weight, incidence }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for directed hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DirectedHyperEdge<'a, Id: Identity> {
    weight: &'a i16,
    incidence: (&'a [Id], &'a [Id]),
}

impl<'a, Id: Identity> fmt::Display for DirectedHyperEdge<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{{weight: {}, link: ", self.weight))?;
        f.debug_set().entries(self.incidence.0.iter()).finish()?;
        f.write_str("->")?;
        f.debug_set().entries(self.incidence.1.iter()).finish()?;
        f.write_str("}")
    }
}

impl<'a, Id: Identity> EdgeModel<Id> for DirectedHyperEdge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        *self.weight
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
        EdgeKind::DirectedHyper
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id> {
        self.incidence.0.iter().collect()
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id> {
        self.incidence.1.iter().collect()
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id> {
        Vec::new()
    }

    /// check edge is same to other edge without weight
    fn is_equal_to_without_weight(&self, other: &Self) -> bool {
        self.incidence == other.incidence
    }
}

impl<'a, Id: Identity> DirectedHyperEdge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create directed hyper edge structure
    #[inline]
    pub(crate) fn _create(weight: &'a i16, source: &'a [Id], target: &'a [Id]) -> Self {
        DirectedHyperEdge {
            weight,
            incidence: (source, target),
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for edge or hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MixedHyperEdge<'a, Id: Identity> {
    Undirected(UndirectedHyperEdge<'a, Id>),
    Directed(DirectedHyperEdge<'a, Id>),
}

impl<'a, Id: Identity> fmt::Display for MixedHyperEdge<'a, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => fmt::Display::fmt(e, f),
            Directed(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl<'a, Id: Identity> EdgeModel<Id> for MixedHyperEdge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_weight(),
            Directed(e) => e.get_weight(),
        }
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_kind(),
            Directed(e) => e.get_kind(),
        }
    }

    /// get source node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_source_ids(&self) -> Vec<&Id> {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_source_ids(),
            Directed(e) => e.get_source_ids(),
        }
    }

    /// get target node ids
    ///
    /// If undirected edge, then return empty vector.
    fn get_target_ids(&self) -> Vec<&Id> {
        use MixedHyperEdge::*;

        match self {
            Undirected(e) => e.get_target_ids(),
            Directed(e) => e.get_target_ids(),
        }
    }

    /// get source and target node ids.
    ///
    /// If directed edge, then return empty vector.
    fn get_source_target_ids(&self) -> Vec<&Id> {
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

impl<'a, Id: Identity> MixedHyperEdge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create node structure
    #[inline]
    pub(crate) fn _create(edge: &'a edge::Edge<Id>) -> Option<Self> {
        match edge {
            edge::Edge::Undirected { .. } | edge::Edge::Directed { .. } => None,
            edge::Edge::UndirectedHyper { weight, ids } => Some(MixedHyperEdge::Undirected(
                UndirectedHyperEdge::_create(weight, ids),
            )),
            edge::Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => Some(MixedHyperEdge::Directed(DirectedHyperEdge::_create(
                weight, source_ids, target_ids,
            ))),
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}

/// Model for edge or hyper edge.
/// If weight is 1 or no weight, the edge's weight is 1.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Edge<'a, Id: Identity> {
    Undirected(UndirectedEdge<'a, Id>),
    Directed(DirectedEdge<'a, Id>),
    UndirectedHyper(UndirectedHyperEdge<'a, Id>),
    DirectedHyper(DirectedHyperEdge<'a, Id>),
}

impl<'a, Id: Identity> fmt::Display for Edge<'a, Id> {
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

impl<'a, Id: Identity> EdgeModel<Id> for Edge<'a, Id> {
    /// get weight for the edge
    fn get_weight(&self) -> i16 {
        use Edge::*;

        match self {
            Undirected(e) => e.get_weight(),
            Directed(e) => e.get_weight(),
            UndirectedHyper(e) => e.get_weight(),
            DirectedHyper(e) => e.get_weight(),
        }
    }

    /// get edge kind for the edge
    fn get_kind(&self) -> EdgeKind{
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
    fn get_source_ids(&self) -> Vec<&Id> {
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
    fn get_target_ids(&self) -> Vec<&Id> {
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
    fn get_source_target_ids(&self) -> Vec<&Id> {
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

impl<'a, Id: Identity> Edge<'a, Id> {
    // ---
    // constructor
    // ---

    /// create edge structure
    #[inline]
    pub(crate) fn _create(edge: &'a edge::Edge<Id>) -> Self {
        match edge {
            edge::Edge::Undirected { weight, ids } => {
                Edge::Undirected(UndirectedEdge::_create(weight, ids))
            }
            edge::Edge::Directed {
                weight,
                source_id,
                target_id,
            } => Edge::Directed(DirectedEdge::_create(weight, source_id, target_id)),
            edge::Edge::UndirectedHyper { weight, ids } => {
                Edge::UndirectedHyper(UndirectedHyperEdge::_create(weight, ids))
            }
            edge::Edge::DirectedHyper {
                weight,
                source_ids,
                target_ids,
            } => Edge::DirectedHyper(DirectedHyperEdge::_create(weight, source_ids, target_ids)),
        }
    }

    // ---
    // getter
    // ---

    // ---
    // checker
    // ---
}
