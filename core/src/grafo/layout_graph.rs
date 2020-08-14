//! graph with the layout for a converter from an input to an output

use crate::grafo::core::graph_item::node::{NodeItemBuilder, NodeItemError};
use crate::grafo::graph_item::edge::{EdgeItem, EdgeItemBuilder, EdgeItemError, EdgeItemOption};
use crate::grafo::graph_item::group::{GroupItem, GroupItemBuilder, GroupItemOption};
use crate::grafo::graph_item::node::{NodeItem, NodeItemOption};
use crate::grafo::graph_item::ItemArena;
use crate::grafo::layout_item::Layout;
use crate::grafo::{GrafoError, Resolver, ResolverError};
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
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
                    if let Err(e) = resolver.push_graph_item_value(
                        GraphItemKind::Group,
                        n,
                        DEFAULT_ITEM_ID,
                        DEFAULT_ITEM_ID,
                    ) {
                        errors.push(NodeItemError::from_with_id(DEFAULT_ITEM_ID, e).into());
                    }
                    validate &= true;
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
                    if let Err(e) = resolver.push_graph_item_value(
                        GraphItemKind::Group,
                        n,
                        DEFAULT_ITEM_ID,
                        DEFAULT_ITEM_ID,
                    ) {
                        errors.push(NodeItemError::from_with_id(DEFAULT_ITEM_ID, e).into());
                    }
                    validate &= true;
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
    layout: Layout,
}

impl<Name: NameType> Grafo<Name> {
    pub fn resolver(&self) -> &Resolver<Name> {
        &self.resolver
    }

    // TODO push_group

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

    pub fn push_edge(&mut self, builder: EdgeItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.edge_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let EdgeItemOption { name } = option;
                if let Some(n) = name {
                    if let Err(e) =
                        resolver.push_graph_item_value(kind, n, belong_group_id, item_id)
                    {
                        errors.push(EdgeItemError::from_with_id(item_id, e).into());
                    }
                    validate &= true;
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
    use crate::grafo::graph_item::edge::{EdgeItemBuilder, EdgeItemError};
    use crate::grafo::graph_item::node::{NodeItemBuilder, NodeItemError};
    use crate::grafo::graph_item::GraphItemBuilderBase;
    use crate::grafo::{NameIdError, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
    use crate::util::kind::GraphItemKind;

    type Graph = NameStrGrafo;
    type GraphBuilder = NameStrGrafoBuilder;
    type GraphError = NameStrGrafoError;

    const ITERATE_COUNT: usize = 10;

    // TODO Group

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
    fn push_node_success_has_error() {
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
                    NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string())
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
    fn push_edge_success_has_error() {
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
            [
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
            .to_vec()
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
                    NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string())
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
                EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(
                    1,
                    (GraphItemKind::Group, "root group".to_string())
                )
                .into(),
                EdgeItemError::FailResolveEndEndpoint(
                    1,
                    Some((GraphItemKind::Group, "root group".to_string()))
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
                    NameIdError::NotExist(GraphItemKind::Node, "not exist".to_string())
                )
                .into(),
                EdgeItemError::FailResolveEndEndpoint(
                    1,
                    Some((GraphItemKind::Node, "not exist".to_string()))
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

    // TODO Whole
}
