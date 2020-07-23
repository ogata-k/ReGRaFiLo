//! graph with the layout for a converter from an input to an output

use crate::grafo::graph_item::edge::EdgeItem;
use crate::grafo::graph_item::group::{GroupItem, GroupItemBuilder, GroupItemOption};
use crate::grafo::graph_item::node::NodeItem;
use crate::grafo::graph_item::ItemArena;
use crate::grafo::layout_item::Layout;
use crate::grafo::{GrafoError, Resolver};
use crate::util::alias::DEFAULT_ITEM_ID;
use crate::util::kind::GraphItemKind;

#[derive(Debug, Clone)]
pub struct GrafoBuilder<'a> {
    // structure resolver
    resolver: Resolver<'a>,

    // layout
    layout: Layout,
}

impl<'a> Default for GrafoBuilder<'a> {
    fn default() -> Self {
        Self {
            resolver: Default::default(),
            layout: Default::default(),
        }
    }
}

impl<'a> GrafoBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_with_default(self) -> Grafo<'a> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            mut resolver,
            layout,
        } = self;

        group_store.push_default(
            &mut resolver,
            |resolver, item_kind, group_id, push_index, _: GroupItemOption| {
                if item_kind == GraphItemKind::Group
                    && group_id == push_index
                    && group_id == DEFAULT_ITEM_ID
                {
                    panic!("fail set default root group");
                }
                resolver.set_root_group_id(push_index);

                // TODO action before insert
                Ok(())
            },
        );

        Grafo {
            group_arena: group_store,
            node_arena: Default::default(),
            edge_arena: Default::default(),
            resolver,
            layout,
        }
    }

    pub fn build_with_user_group(
        self,
        group_builder: GroupItemBuilder,
    ) -> Result<Grafo<'a>, Vec<GrafoError>> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            mut resolver,
            layout,
        } = self;
        group_store.push_user_item_as_default(
            &mut resolver,
            group_builder,
            |resolver, item_kind, group_id, push_index, option| {
                if item_kind == GraphItemKind::Group
                    && group_id == push_index
                    && group_id == DEFAULT_ITEM_ID
                {
                    panic!("fail set user root group");
                }
                resolver.set_root_group_id(push_index);

                // TODO action before insert
                Ok(())
            },
        )?;
        Ok(Grafo {
            group_arena: group_store,
            node_arena: Default::default(),
            edge_arena: Default::default(),
            resolver,
            layout,
        })
    }
}

/// Grafo is Graph with Layout
#[derive(Debug, Clone)]
pub struct Grafo<'a> {
    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // name to id
    resolver: Resolver<'a>,

    // layout
    layout: Layout,
}

impl<'a> Grafo<'a> {
    // TODO
}
