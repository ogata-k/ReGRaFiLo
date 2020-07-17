//! graph with the layout for a converter from an input to an output

use crate::grafo::core::item::edge::EdgeItem;
use crate::grafo::core::item::group::GroupItem;
use crate::grafo::core::item::node::NodeItem;
use crate::grafo::core::item::ItemArena;
use crate::grafo::core::layout::LayoutReference;
use crate::grafo::core::refindex::NameReference;

/// Grafo is Graph with Layout
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Grafo<'a> {
    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // name to id
    item_names: NameReference<'a>,

    // layout
    layout: LayoutReference,
}

impl<'a> Grafo<'a> {
    // TODO
}
