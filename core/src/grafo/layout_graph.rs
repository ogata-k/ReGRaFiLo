//! graph with the layout for a converter from an input to an output

use crate::grafo::core::graph_item::edge::EdgeItem;
use crate::grafo::core::graph_item::group::GroupItem;
use crate::grafo::core::graph_item::node::NodeItem;
use crate::grafo::core::graph_item::ItemArena;
use crate::grafo::core::layout_item::Layout;
use crate::grafo::core::name_refindex::NameReference;

/// Grafo is Graph with Layout
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Grafo<'a> {
    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // name to id
    name_ref: NameReference<'a>,

    // layout
    layout: Layout,
}

impl<'a> Grafo<'a> {
    // TODO
}
