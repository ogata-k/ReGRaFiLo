//! graph with the layout for a converter from an input to an output

use crate::grafo::core::graph_item::node::{NodeItemBuilder, NodeItemError};
use crate::grafo::graph_item::edge::EdgeItem;
use crate::grafo::graph_item::group::{GroupItem, GroupItemBuilder, GroupItemOption};
use crate::grafo::graph_item::node::{NodeItem, NodeItemOption};
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
                if item_kind != GraphItemKind::Group
                    || group_id != push_index
                    || group_id != DEFAULT_ITEM_ID
                {
                    panic!("fail set default root group");
                }
                resolver.set_root_group_id(push_index);

                // TODO action before insert
                (true, Vec::new())
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
        let (result, mut errors) = group_store.push_user_item_as_default(
            &mut resolver,
            group_builder,
            |resolver, item_kind, group_id, push_index, option| {
                let mut errors: Vec<GrafoError> = Vec::new();
                if item_kind != GraphItemKind::Group
                    || group_id != push_index
                    || group_id != DEFAULT_ITEM_ID
                {
                    panic!("fail set user root group");
                }
                resolver.set_root_group_id(push_index);

                // TODO action before insert
                (true, errors)
            },
        );
        if !result {
            errors.push(GrafoError::FailBuildGrafo);
            Err(errors)
        } else {
            Ok(Grafo {
                group_arena: group_store,
                node_arena: Default::default(),
                edge_arena: Default::default(),
                resolver,
                layout,
            })
        }
    }
}

/// Grafo is Graph with Layout
#[derive(Debug, Clone)]
pub struct Grafo<'a> {
    // structure resolver
    resolver: Resolver<'a>,

    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // layout
    layout: Layout,
}

impl<'a> Grafo<'a> {
    // TODO 2 next push_group
    pub fn push_node(&mut self, builder: NodeItemBuilder) -> (bool, Vec<GrafoError>) {
        self.node_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError> = Vec::new();
                let mut validate = true;
                let NodeItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) = resolver.push_item_name(kind, n, belong_group_id, item_id) {
                        errors.push(NodeItemError::from(e).into());
                    }
                }

                (validate, errors)
            },
        )
    }
    // TODO 3 push_edge
}

#[cfg(test)]
mod test {
    use crate::grafo::graph_item::node::NodeItemBuilder;
    use crate::grafo::graph_item::GraphItemBuilderBase;
    use crate::grafo::{GrafoBuilder, GrafoError};
    use crate::util::kind::GraphItemKind;

    const ITERATE_COUNT: usize = 10;
    #[test]
    fn push_node_success() {
        let mut graph = GrafoBuilder::new().build_with_default();
        for i in 0..2 * ITERATE_COUNT {
            let mut node_builder = NodeItemBuilder::new();
            if i % 2 == 0 {
                node_builder.set_name(format!("{}", i));
            }
            let (result, errors) = graph.push_node(node_builder);
            assert_eq!(Vec::<GrafoError>::new(), errors);
            assert!(result);
        }

        assert_eq!(graph.node_arena.count(), 2 * ITERATE_COUNT);
        assert_eq!(
            graph.resolver.item_name_count_by(GraphItemKind::Node),
            ITERATE_COUNT
        );
    }
}
