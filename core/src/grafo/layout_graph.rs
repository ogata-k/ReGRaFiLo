//! graph with the layout for a converter from an input to an output

use crate::grafo::core::graph_item::node::{NodeItemBuilder, NodeItemError};
use crate::grafo::graph_item::edge::{EdgeItem, EdgeItemBuilder, EdgeItemError, EdgeItemOption};
use crate::grafo::graph_item::group::{
    GroupItem, GroupItemBuilder, GroupItemError, GroupItemOption,
};
use crate::grafo::graph_item::node::{NodeItem, NodeItemOption};
use crate::grafo::graph_item::ItemArena;
use crate::grafo::layout_item::Layout;
use crate::grafo::{GrafoError, Resolver, ResolverError};
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::item_base::FromWithItemId;
use crate::util::kind::GraphItemKind;
use crate::util::name_type::NameType;
use crate::util::writer::DisplayAsJson;

#[derive(Debug, Clone)]
pub struct GrafoBuilder<Name: NameType> {
    // structure resolver
    resolver: Resolver<Name>,

    // layout
    // TODO Check もし必要ならItemArenaのように分ける
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

    pub fn build_with_no_name_default_group(self) -> Result<Grafo<Name>, Vec<GrafoError<Name>>> {
        self.build_with_default_group(None)
    }

    pub fn build_with_name_default_group<S: Into<Name>>(
        self,
        group_name: S,
    ) -> Result<Grafo<Name>, Vec<GrafoError<Name>>> {
        self.build_with_default_group(Some(group_name.into()))
    }

    fn build_with_default_group(
        self,
        group_name: Option<Name>,
    ) -> Result<Grafo<Name>, Vec<GrafoError<Name>>> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            mut resolver,
            layout,
        } = self;

        let (result, mut errors) = group_store.push_default(
            &mut resolver,
            |resolver, item_kind, group_id, push_index, option: GroupItemOption<Name>| {
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

                let mut validate = true;
                let GroupItemOption { name: _ } = option;
                if let Some(n) = group_name {
                    if let Err(e) = resolver.push_graph_item_value_or_override(
                        GraphItemKind::Group,
                        n,
                        DEFAULT_ITEM_ID,
                        DEFAULT_ITEM_ID,
                    ) {
                        errors.push(NodeItemError::from_with_id(DEFAULT_ITEM_ID, e).into());
                        validate &= true;
                    }
                }

                (validate, errors)
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

                let mut validate = true;
                let GroupItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) = resolver.push_graph_item_value_or_override(
                        GraphItemKind::Group,
                        n,
                        DEFAULT_ITEM_ID,
                        DEFAULT_ITEM_ID,
                    ) {
                        errors.push(NodeItemError::from_with_id(DEFAULT_ITEM_ID, e).into());
                        validate &= true;
                    }
                }

                (validate, errors)
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
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grafo<Name: NameType> {
    // structure resolver
    resolver: Resolver<Name>,

    // item arena
    group_arena: ItemArena<GroupItem>,
    node_arena: ItemArena<NodeItem>,
    edge_arena: ItemArena<EdgeItem>,

    // layout
    // TODO Check もし必要ならItemArenaのように分ける
    layout: Layout,
}

impl<Name: NameType> DisplayAsJson for Grafo<Name> {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"resolver\": ")?;
        self.resolver.fmt_as_json(f)?;
        write!(f, ", \"node_items\": ")?;
        self.node_arena.fmt_as_json(f)?;
        write!(f, ", \"edge_items\": ")?;
        self.edge_arena.fmt_as_json(f)?;
        // TODO layout
        write!(f, "}}")
    }
}

impl<Name: NameType> std::fmt::Display for Grafo<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grafo")?;
        self.fmt_as_json(f)
    }
}

impl<Name: NameType> Grafo<Name> {
    pub fn resolver(&self) -> &Resolver<Name> {
        &self.resolver
    }

    pub fn push_group(&mut self, builder: GroupItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.group_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let GroupItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) = resolver.push_graph_item_value_or_override(
                        kind,
                        n,
                        belong_group_id,
                        item_id,
                    ) {
                        errors.push(GroupItemError::from_with_id(item_id, e).into());
                        validate &= true;
                    }
                }

                if validate {
                    if let Err(e) = resolver.insert_group(belong_group_id, item_id) {
                        errors.push(e.into());
                        validate &= false;
                    }
                }

                (validate, errors)
            },
        )
    }

    pub fn push_node(&mut self, builder: NodeItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.node_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let NodeItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) = resolver.push_graph_item_value_or_override(
                        kind,
                        n,
                        belong_group_id,
                        item_id,
                    ) {
                        errors.push(NodeItemError::from_with_id(item_id, e).into());
                        validate &= true;
                    }
                }

                (validate, errors)
            },
        )
    }

    pub fn push_edge(&mut self, builder: EdgeItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.edge_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let EdgeItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) = resolver.push_graph_item_value_or_override(
                        kind,
                        n,
                        belong_group_id,
                        item_id,
                    ) {
                        errors.push(EdgeItemError::from_with_id(item_id, e).into());
                        validate &= true;
                    }
                }

                (validate, errors)
            },
        )
    }

    pub fn get_root_group_item(&self) -> Option<&GroupItem> {
        self.group_arena.get(DEFAULT_ITEM_ID, DEFAULT_ITEM_ID)
    }

    pub fn get_group_item(&self, group_id: GroupId, item_id: ItemId) -> Option<&GroupItem> {
        self.group_arena.get(group_id, item_id)
    }

    pub fn get_node_item(&self, group_id: GroupId, item_id: ItemId) -> Option<&NodeItem> {
        self.node_arena.get(group_id, item_id)
    }

    pub fn get_edge_item(&self, group_id: GroupId, item_id: ItemId) -> Option<&EdgeItem> {
        self.edge_arena.get(group_id, item_id)
    }
}

#[cfg(test)]
mod test {
    use crate::grafo::core::graph_item::group::GroupItemBuilder;
    use crate::grafo::graph_item::edge::{EdgeItemBuilder, EdgeItemError};
    use crate::grafo::graph_item::group::GroupItemError;
    use crate::grafo::graph_item::node::{NodeItemBuilder, NodeItemError};
    use crate::grafo::graph_item::{GraphItemBase, GraphItemBuilderBase};
    use crate::grafo::{
        GrafoError, NameIdError, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError,
    };
    use crate::util::item_base::ItemBase;
    use crate::util::kind::GraphItemKind;

    type Graph = NameStrGrafo;
    type GraphBuilder = NameStrGrafoBuilder;
    type GraphError = NameStrGrafoError;

    const ITERATE_COUNT: usize = 10;

    #[test]
    fn build_grafo_fail() {
        let mut group_builder = GroupItemBuilder::new();
        group_builder.set_belong_group("root");
        assert_eq!(
            GraphBuilder::new().build_with_user_group(group_builder),
            Err(vec![
                GroupItemError::CannotSpecifyBelongGroupForRoot("root".to_string()).into(),
                GrafoError::FailBuildGrafo,
            ])
        );
    }

    #[test]
    fn push_default_no_name_group_success() {
        let user_default_graph = GraphBuilder::new().build_with_user_group(GroupItemBuilder::new());
        let default_graph = GraphBuilder::new().build_with_no_name_default_group();
        assert_eq!(user_default_graph, default_graph);
        assert!(user_default_graph.is_ok());
    }

    #[test]
    fn push_default_name_group_success() {
        let mut group_builder = GroupItemBuilder::new();
        group_builder.set_name("root");
        let user_default_graph = GraphBuilder::new().build_with_user_group(group_builder);
        let default_graph = GraphBuilder::new().build_with_name_default_group("root");
        assert_eq!(user_default_graph, default_graph);
        assert!(user_default_graph.is_ok());
    }

    #[test]
    fn push_two_group_success_and_push_node_each_group() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut group_builder_1 = GroupItemBuilder::new();
        group_builder_1.set_name("group_1");
        let (result, errors) = graph.push_group(group_builder_1);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group_1"),
            Ok((0, 1))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut group_builder_2 = GroupItemBuilder::new();
        group_builder_2.set_name("group_2");
        let (result, errors) = graph.push_group(group_builder_2);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group_2"),
            Ok((0, 2))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        assert_eq!(graph.group_arena.count(), 3);

        // in group_1
        let mut node_builder_1_1 = NodeItemBuilder::new();
        node_builder_1_1.set_name("node_1_1");
        node_builder_1_1.set_belong_group("group_1");
        let (result, errors) = graph.push_node(node_builder_1_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());
        let node_1_1_id_pair = graph
            .resolver
            .get_graph_item_id_pair(GraphItemKind::Node, "node_1_1");
        assert_eq!(node_1_1_id_pair, Ok((1, 1)));
        let (node_1_1_belong_group_id, node_1_1_item_id) = node_1_1_id_pair.unwrap();
        let node_1_1 = graph.get_node_item(node_1_1_belong_group_id, node_1_1_item_id);
        assert!(node_1_1.is_some());
        assert_eq!(node_1_1.unwrap().get_belong_group_id(), 1);
        assert_eq!(node_1_1.unwrap().get_item_id(), 1);

        let mut node_builder_1_2 = NodeItemBuilder::new();
        node_builder_1_2.set_name("node_1_2");
        node_builder_1_2.set_belong_group("group_1");
        let (result, errors) = graph.push_node(node_builder_1_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());
        let node_1_2_id_pair = graph
            .resolver
            .get_graph_item_id_pair(GraphItemKind::Node, "node_1_2");
        assert_eq!(node_1_2_id_pair, Ok((1, 2)));
        let (node_1_2_belong_group_id, node_1_2_item_id) = node_1_2_id_pair.unwrap();
        let node_1_2 = graph.get_node_item(node_1_2_belong_group_id, node_1_2_item_id);
        assert!(node_1_2.is_some());
        assert_eq!(node_1_2.unwrap().get_belong_group_id(), 1);
        assert_eq!(node_1_2.unwrap().get_item_id(), 2);

        // in group_2
        let mut node_builder_2_1 = NodeItemBuilder::new();
        node_builder_2_1.set_name("node_2_1");
        node_builder_2_1.set_belong_group("group_2");
        let (result, errors) = graph.push_node(node_builder_2_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());
        let node_2_1_id_pair = graph
            .resolver
            .get_graph_item_id_pair(GraphItemKind::Node, "node_2_1");
        assert_eq!(node_2_1_id_pair, Ok((2, 3)));
        let (node_2_1_belong_group_id, node_2_1_item_id) = node_2_1_id_pair.unwrap();
        let node_2_1 = graph.get_node_item(node_2_1_belong_group_id, node_2_1_item_id);
        assert!(node_2_1.is_some());
        assert_eq!(node_2_1.unwrap().get_belong_group_id(), 2);
        assert_eq!(node_2_1.unwrap().get_item_id(), 3);

        assert_eq!(graph.node_arena.count(), 3);
    }

    #[test]
    pub fn push_group_success_to_not_root_group() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut group_builder_1 = GroupItemBuilder::new();
        group_builder_1.set_name("group_1");
        let (result, errors) = graph.push_group(group_builder_1);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group_1"),
            Ok((0, 1))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut group_builder_2 = GroupItemBuilder::new();
        group_builder_2.set_name("group_2");
        let (result, errors) = graph.push_group(group_builder_2);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group_2"),
            Ok((0, 2))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        assert_eq!(graph.group_arena.count(), 3);
    }

    #[test]
    fn push_group_success_with_name_override() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut group_builder_1 = GroupItemBuilder::new();
        group_builder_1.set_name("group");
        let (result, errors) = graph.push_group(group_builder_1);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group"),
            Ok((0, 1))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut group_builder_2 = GroupItemBuilder::new();
        group_builder_2.set_name("group");
        let (result, errors) = graph.push_group(group_builder_2);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group"),
            Ok((0, 2))
        );
        assert!(result);
        assert_eq!(
            errors,
            vec![
                GroupItemError::NameIdError(
                    2,
                    NameIdError::AlreadyExist(GraphItemKind::Group, "group".to_string())
                )
                .into(),
                GroupItemError::NameIdError(
                    2,
                    NameIdError::Override(GraphItemKind::Group, "group".to_string())
                )
                .into(),
            ]
        );
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group"),
            Ok((0, 2))
        );

        assert_eq!(graph.group_arena.count(), 3);
    }

    #[test]
    fn build_group_fail() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut group_builder = GroupItemBuilder::new();
        group_builder.set_belong_group("hoge");
        let (result, errors) = graph.push_group(group_builder);
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                GroupItemError::NameIdError(
                    1,
                    NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string())
                )
                .into(),
                GroupItemError::FailResolveBelongGroup(1, Some("hoge".to_string())).into(),
            ]
        );
    }

    #[test]
    fn push_node_success() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        for i in 0..2 * ITERATE_COUNT {
            let mut node_builder = NodeItemBuilder::new();
            if i % 2 == 0 {
                node_builder.set_name(format!("{}", i));
            }
            let (result, errors) = graph.push_node(node_builder);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());
        }

        assert_eq!(graph.node_arena.count(), 2 * ITERATE_COUNT);
        assert_eq!(
            graph
                .resolver
                .count_usable_graph_item_names_by(GraphItemKind::Node),
            ITERATE_COUNT
        );
    }

    #[test]
    fn push_node_success_with_name_override() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node");
        let (result, errors) = graph.push_node(node_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node");
        let (result, errors) = graph.push_node(node_builder_2);
        assert!(result);
        assert_eq!(
            errors,
            vec![
                NodeItemError::NameIdError(
                    2,
                    NameIdError::AlreadyExist(GraphItemKind::Node, "node".to_string()),
                )
                .into(),
                NodeItemError::NameIdError(
                    2,
                    NameIdError::Override(GraphItemKind::Node, "node".to_string()),
                )
                .into(),
            ]
        );
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Node, "node"),
            Ok((0, 2))
        );
    }

    #[test]
    fn build_node_fail() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_belong_group("hoge");
        let (result, errors) = graph.push_node(node_builder);
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                NodeItemError::NameIdError(
                    1,
                    NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string()),
                )
                .into(),
                NodeItemError::FailResolveBelongGroup(1, Some("hoge".to_string())).into(),
            ]
        );
    }

    #[test]
    fn push_edge_success() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        for i in 0..2 * ITERATE_COUNT {
            let mut node_builder = NodeItemBuilder::new();
            if i % 2 == 0 {
                node_builder.set_name(format!("{}", i));
            }
            let (result, errors) = graph.push_node(node_builder);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());
        }

        for i in 0..ITERATE_COUNT {
            let mut edge_builder = EdgeItemBuilder::new();
            if i % 2 == 0 {
                edge_builder.set_name(format!("{}", i));
            }
            edge_builder.set_start_endpoint(
                GraphItemKind::Node,
                format!("{}", (2 * i) % (2 * ITERATE_COUNT)),
            );
            edge_builder.set_end_endpoint(
                GraphItemKind::Node,
                format!("{}", (2 * (i + 1)) % (2 * ITERATE_COUNT)),
            );
            let (result, errors) = graph.push_edge(edge_builder);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());
        }

        assert_eq!(graph.edge_arena.count(), ITERATE_COUNT);
        assert_eq!(
            graph
                .resolver
                .count_usable_graph_item_names_by(GraphItemKind::Node),
            ITERATE_COUNT
        );

        assert_eq!(
            graph
                .resolver
                .count_usable_graph_item_names_by(GraphItemKind::Edge),
            if ITERATE_COUNT % 2 == 0 {
                ITERATE_COUNT / 2
            } else {
                (ITERATE_COUNT - 1) / 2
            }
        );
    }

    #[test]
    fn push_edge_success_edges_on_same_endpoints() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node1");
        let (result, errors) = graph.push_node(node_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node2");
        let (result, errors) = graph.push_node(node_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder_1 = EdgeItemBuilder::new();
        edge_builder_1.set_start_endpoint(GraphItemKind::Node, "node1");
        edge_builder_1.set_end_endpoint(GraphItemKind::Node, "node2");
        let (result, errors) = graph.push_edge(edge_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder_2 = EdgeItemBuilder::new();
        edge_builder_2.set_start_endpoint(GraphItemKind::Node, "node1");
        edge_builder_2.set_end_endpoint(GraphItemKind::Node, "node2");
        let (result, errors) = graph.push_edge(edge_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());
    }

    #[test]
    fn push_edge_success_loop_edge() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_name("node");
        let (result, errors) = graph.push_node(node_builder);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_start_endpoint(GraphItemKind::Node, "node");
        edge_builder.set_end_endpoint(GraphItemKind::Node, "node");
        let (result, errors) = graph.push_edge(edge_builder);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());
    }

    #[test]
    fn push_edge_success_opposite_edges() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node1");
        let (result, errors) = graph.push_node(node_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node2");
        let (result, errors) = graph.push_node(node_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder_1 = EdgeItemBuilder::new();
        edge_builder_1.set_start_endpoint(GraphItemKind::Node, "node1");
        edge_builder_1.set_end_endpoint(GraphItemKind::Node, "node2");
        let (result, errors) = graph.push_edge(edge_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder_2 = EdgeItemBuilder::new();
        edge_builder_2.set_start_endpoint(GraphItemKind::Node, "node2");
        edge_builder_2.set_end_endpoint(GraphItemKind::Node, "node1");
        let (result, errors) = graph.push_edge(edge_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());
    }

    #[test]
    fn push_edge_success_endpoints_is_not_same_group() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut group_builder_1 = GroupItemBuilder::new();
        group_builder_1.set_name("group_1");
        let (result, errors) = graph.push_group(group_builder_1);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group_1"),
            Ok((0, 1))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut group_builder_2 = GroupItemBuilder::new();
        group_builder_2.set_name("group_2");
        let (result, errors) = graph.push_group(group_builder_2);
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Group, "group_2"),
            Ok((0, 2))
        );
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        assert_eq!(graph.group_arena.count(), 3);

        // node
        // in group_1
        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node_1");
        node_builder_1.set_belong_group("group_1");
        let (result, errors) = graph.push_node(node_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        // in group_2
        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node_2");
        node_builder_2.set_belong_group("group_2");
        let (result, errors) = graph.push_node(node_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        assert_eq!(graph.node_arena.count(), 2);

        // edge
        // in root group
        let mut edge_builder_1 = EdgeItemBuilder::new();
        edge_builder_1.set_start_endpoint(GraphItemKind::Node, "node_1");
        edge_builder_1.set_end_endpoint(GraphItemKind::Node, "node_2");
        let (result, errors) = graph.push_edge(edge_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        // in group_1
        let mut edge_builder_2 = EdgeItemBuilder::new();
        edge_builder_2.set_belong_group("group_1");
        edge_builder_2.set_start_endpoint(GraphItemKind::Node, "node_1");
        edge_builder_2.set_end_endpoint(GraphItemKind::Node, "node_2");
        let (result, errors) = graph.push_edge(edge_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        // in group_2
        let mut edge_builder_3 = EdgeItemBuilder::new();
        edge_builder_3.set_belong_group("group_2");
        edge_builder_3.set_start_endpoint(GraphItemKind::Node, "node_1");
        edge_builder_3.set_end_endpoint(GraphItemKind::Node, "node_2");
        let (result, errors) = graph.push_edge(edge_builder_3);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        assert_eq!(graph.edge_arena.count(), 3);
    }

    #[test]
    fn push_edge_success_with_name_override() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node1");
        let (result, errors) = graph.push_node(node_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node2");
        let (result, errors) = graph.push_node(node_builder_2);
        assert!(result);
        assert_eq!(Vec::<GraphError>::new(), errors);

        let mut edge_builder_1 = EdgeItemBuilder::new();
        edge_builder_1.set_name("edge");
        edge_builder_1.set_start_endpoint(GraphItemKind::Node, "node1");
        edge_builder_1.set_end_endpoint(GraphItemKind::Node, "node2");
        let (result, errors) = graph.push_edge(edge_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder_2 = EdgeItemBuilder::new();
        edge_builder_2.set_name("edge");
        edge_builder_2.set_start_endpoint(GraphItemKind::Node, "node1");
        edge_builder_2.set_end_endpoint(GraphItemKind::Node, "node2");
        let (result, errors) = graph.push_edge(edge_builder_2);
        assert!(result);
        assert_eq!(
            errors,
            vec![
                EdgeItemError::NameIdError(
                    2,
                    NameIdError::AlreadyExist(GraphItemKind::Edge, "edge".to_string()),
                )
                .into(),
                EdgeItemError::NameIdError(
                    2,
                    NameIdError::Override(GraphItemKind::Edge, "edge".to_string()),
                )
                .into(),
            ]
        );
        assert_eq!(
            graph
                .resolver
                .get_graph_item_id_pair(GraphItemKind::Edge, "edge"),
            Ok((0, 2))
        );
    }

    #[test]
    fn build_edge_fail_not_found_belong_group_name() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder_1 = NodeItemBuilder::new();
        node_builder_1.set_name("node1");
        let (result, errors) = graph.push_node(node_builder_1);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut node_builder_2 = NodeItemBuilder::new();
        node_builder_2.set_name("node2");
        let (result, errors) = graph.push_node(node_builder_2);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_belong_group("hoge");
        edge_builder.set_start_endpoint(GraphItemKind::Node, "node1");
        edge_builder.set_end_endpoint(GraphItemKind::Node, "node2");
        let (result, errors) = graph.push_edge(edge_builder);
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                EdgeItemError::NameIdError(
                    1,
                    NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string()),
                )
                .into(),
                EdgeItemError::FailResolveBelongGroup(1, Some("hoge".to_string())).into(),
            ]
        );
    }

    #[test]
    fn build_edge_fail_specify_cannot_usable_end_endpoint() {
        let graph = GraphBuilder::new().build_with_name_default_group("root group");
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_name("node");
        let (result, errors) = graph.push_node(node_builder);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_start_endpoint(GraphItemKind::Node, "node");
        edge_builder.set_end_endpoint(GraphItemKind::Group, "root group");
        let (result, errors) = graph.push_edge(edge_builder);
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(1, "root group".to_string(),)
                    .into(),
                EdgeItemError::FailResolveEndEndpoint(
                    1,
                    Some((GraphItemKind::Group, "root group".to_string())),
                )
                .into(),
            ]
        );
    }

    #[test]
    fn build_edge_fail_cannot_found_end_endpoint() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_name("node");
        let (result, errors) = graph.push_node(node_builder);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_start_endpoint(GraphItemKind::Node, "node");
        edge_builder.set_end_endpoint(GraphItemKind::Node, "not exist");
        let (result, errors) = graph.push_edge(edge_builder);
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                EdgeItemError::NameIdError(
                    1,
                    NameIdError::NotExist(GraphItemKind::Node, "not exist".to_string()),
                )
                .into(),
                EdgeItemError::FailResolveEndEndpoint(
                    1,
                    Some((GraphItemKind::Node, "not exist".to_string())),
                )
                .into(),
            ]
        );
    }

    #[test]
    fn build_edge_fail_not_specify_end_endpoint() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_name("node");
        let (result, errors) = graph.push_node(node_builder);
        assert!(result);
        assert_eq!(errors, Vec::<GraphError>::new());

        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_start_endpoint(GraphItemKind::Node, "node");
        let (result, errors) = graph.push_edge(edge_builder);
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                EdgeItemError::NotSpecifyEndEndpoint(1, None).into(),
                EdgeItemError::FailResolveEndEndpoint(1, None).into(),
            ]
        );
    }

    #[test]
    fn build_edge_fail_not_specify_endpoints() {
        let graph = GraphBuilder::new().build_with_no_name_default_group();
        if graph.is_err() {
            panic!("errors: {:?}", graph.err().unwrap()); // in test panic
        }
        let mut graph: Graph = graph.unwrap();

        let (result, errors) = graph.push_edge(EdgeItemBuilder::new());
        assert!(!result);
        assert_eq!(
            errors,
            vec![
                EdgeItemError::NotSpecifyStartEndpoint(1, None).into(),
                EdgeItemError::NotSpecifyEndEndpoint(1, None).into(),
                EdgeItemError::FailResolveStartEndpoint(1, None).into(),
                EdgeItemError::FailResolveEndEndpoint(1, None).into(),
            ]
        );
    }
}
