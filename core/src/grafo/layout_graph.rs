//! graph with the layout for a converter from an input to an output

use crate::grafo::core::graph_item::edge::EdgeItem;
use crate::grafo::core::graph_item::group::{GroupItem, GroupItemBuilder};
use crate::grafo::core::graph_item::node::NodeItem;
use crate::grafo::core::graph_item::ItemArena;
use crate::grafo::core::layout_item::Layout;
use crate::grafo::core::resolve::Resolver;
use crate::grafo::GrafoError;
use crate::util::kind::GraphItemKind;

#[derive(Debug, Clone)]
pub struct GrafoBuilder<'a> {
    // TODO グループ構造の管理(GroupTree)

    // name to id
    resolver: Resolver<'a>,

    // layout
    layout: Layout,
}

impl<'a> GrafoBuilder<'a> {
    // TODO build_with_default_group(self)->Grafo<'a>

    pub fn build(self, group_builder: GroupItemBuilder) -> Result<Grafo<'a>, Vec<GrafoError>> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            resolver: mut name_ref,
            layout,
        } = self;
        // TODO pushのactionの引数にGroupTree???
        let push_result = group_store.push(
            &mut name_ref,
            group_builder,
            |resolver, item_kind, group_id, push_index, option| {
                // グループのルートを設定
                // ルートのIDは自身と同じ
                if item_kind == GraphItemKind::Group && group_id == push_index {
                    resolver.set_root_group_id(push_index);
                }

                // TODO ここでGroupTreeを指定したい
                None
            },
        );
        match push_result {
            Some(errors) => Err(errors),
            None => Ok(Grafo {
                group_arena: group_store,
                node_arena: Default::default(),
                edge_arena: Default::default(),
                name_ref,
                layout,
            }),
        }
    }
}

/// Grafo is Graph with Layout
#[derive(Debug, Clone)]
pub struct Grafo<'a> {
    // TODO グループ構造の管理(GroupTree)

    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // name to id
    name_ref: Resolver<'a>,

    // layout
    layout: Layout,
}

impl<'a> Grafo<'a> {
    // TODO
}
