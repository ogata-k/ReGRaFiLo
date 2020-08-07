//! graph with the layout for a converter from an input to an output

use crate::grafo::core::graph_item::node::{NodeItemBuilder, NodeItemError};
use crate::grafo::graph_item::edge::EdgeItem;
use crate::grafo::graph_item::group::{GroupItem, GroupItemBuilder, GroupItemOption};
use crate::grafo::graph_item::node::{NodeItem, NodeItemOption};
use crate::grafo::graph_item::ItemArena;
use crate::grafo::layout_item::Layout;
use crate::grafo::{GrafoError, Resolver, ResolverError};
use crate::util::alias::DEFAULT_ITEM_ID;
use crate::util::item_base::FromWithItemId;
use crate::util::kind::GraphItemKind;
use crate::util::name_type::NameType;

#[derive(Debug, Clone)]
pub struct GrafoBuilder<Name: NameType> {
    // structure resolver
    resolver: Resolver<Name>,

    // layout
    layout: Layout,
}

impl<Name: NameType> Default for GrafoBuilder<Name> {
    fn default() -> Self {
        Self {
            resolver: Resolver::new(),
            layout: Default::default(),
        }
    }
}

impl<Name: NameType> GrafoBuilder<Name> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_with_default(self) -> Result<Grafo<Name>, Vec<GrafoError<Name>>> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            mut resolver,
            layout,
        } = self;

        let (result, mut errors) = group_store.push_default(
            &mut resolver,
            |resolver, item_kind, group_id, push_index, _: GroupItemOption<Name>| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();

                if item_kind != GraphItemKind::Group
                    || group_id != push_index
                    || group_id != DEFAULT_ITEM_ID
                {
                    return (false, vec![ResolverError::FailSetRootGraphId.into()]);
                }
                if let Err(e) = resolver.set_root_group_id(push_index) {
                    return (false, vec![e.into()]);
                }

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

    pub fn build_with_user_group(
        self,
        group_builder: GroupItemBuilder<Name>,
    ) -> Result<Grafo<Name>, Vec<GrafoError<Name>>> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            mut resolver,
            layout,
        } = self;
        let (result, mut errors) = group_store.push_user_item_as_default(
            &mut resolver,
            group_builder,
            |resolver, item_kind, group_id, push_index, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();

                if item_kind != GraphItemKind::Group
                    || group_id != push_index
                    || group_id != DEFAULT_ITEM_ID
                {
                    return (false, vec![ResolverError::FailSetRootGraphId.into()]);
                }
                if let Err(e) = resolver.set_root_group_id(push_index) {
                    return (false, vec![e.into()]);
                }

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
pub struct Grafo<Name: NameType> {
    // structure resolver
    resolver: Resolver<Name>,

    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // layout
    layout: Layout,
}

impl<Name: NameType> Grafo<Name> {
    // TODO 2 next push_group
    pub fn push_node(&mut self, builder: NodeItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.node_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let NodeItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) =
                        resolver.push_graph_item_value(kind, n, belong_group_id, item_id)
                    {
                        errors.push(NodeItemError::from_with_id(item_id, e).into());
                    }
                    validate &= true;
                }

                (validate, errors)
            },
        )
    }
    // TODO 3 push_edge
}

#[cfg(test)]
mod test {
    use crate::grafo::graph_item::node::{NodeItemBuilder, NodeItemError};
    use crate::grafo::graph_item::GraphItemBuilderBase;
    use crate::grafo::{GrafoBuilder, GrafoError, NameIdError};
    use crate::util::kind::GraphItemKind;

    const ITERATE_COUNT: usize = 10;

    #[test]
    fn push_node_success() {
        let graph = GrafoBuilder::new().build_with_default();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph = graph.unwrap();

        for i in 0..2 * ITERATE_COUNT {
            let mut node_builder = NodeItemBuilder::new();
            if i % 2 == 0 {
                node_builder.set_name(format!("{}", i));
            }
            let (result, errors) = graph.push_node(node_builder);
            assert_eq!(Vec::<GrafoError<String>>::new(), errors);
            assert!(result);
        }

        assert_eq!(graph.node_arena.count(), 2 * ITERATE_COUNT);
        assert_eq!(
            graph
                .resolver
                .count_names_graph_item_by(GraphItemKind::Node),
            ITERATE_COUNT
        );
    }

    #[test]
    fn push_node_success_has_error() {
        let graph = GrafoBuilder::new().build_with_default();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph = graph.unwrap();

        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node");
        let (result, errors) = graph.push_node(node_builder_1);
        assert_eq!(Vec::<GrafoError<String>>::new(), errors);
        assert!(result);

        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node");
        let (result, errors) = graph.push_node(node_builder_2);
        assert_eq!(
            errors,
            [
                NodeItemError::NameIdError(
                    2,
                    NameIdError::AlreadyExist(GraphItemKind::Node, "node".to_string())
                )
                .into(),
                NodeItemError::NameIdError(
                    2,
                    NameIdError::Override(GraphItemKind::Node, "node".to_string())
                )
                .into(),
            ]
            .to_vec()
        );
        assert!(result);
    }

    #[test]
    fn build_node_fail() {
        let graph = GrafoBuilder::new().build_with_default();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph = graph.unwrap();

        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_belong_group("hoge");
        let (result, errors) = graph.push_node(node_builder);
        assert!(!result);
        assert_ne!(Vec::<GrafoError<String>>::new(), errors);
    }
}
