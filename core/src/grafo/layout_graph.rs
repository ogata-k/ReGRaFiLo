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
use crate::util::iter::{IterGroupByAll, IterGroupById, IterGroupByList};
use crate::util::kind::GraphItemKind;
use crate::util::name_type::NameType;
use crate::util::writer::DisplayAsJson;

/// builder for Grafo
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

// TODO Layout関係のメソッド
impl<Name: NameType> GrafoBuilder<Name> {
    /// initializer for Grafo Builder
    pub fn new() -> Self {
        Default::default()
    }

    /// build to Grafo with default root group. But root group doesn't have name.
    pub fn build_with_no_name_default_group(self) -> Grafo<Name> {
        self.build_with_default_group(None)
    }

    /// build to Grafo with default root group which has specified name.
    pub fn build_with_name_default_group<S: Into<Name>>(self, group_name: S) -> Grafo<Name> {
        self.build_with_default_group(Some(group_name.into()))
    }

    /// build to Grafo with default root group. You can specify name by arg.
    fn build_with_default_group(self, group_name: Option<Name>) -> Grafo<Name> {
        let mut group_store = ItemArena::<GroupItem>::new();
        let GrafoBuilder {
            mut resolver,
            layout,
        } = self;

        let (result, errors) = group_store.push_default(
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
                if let Err(e) = resolver.insert_graph_item_id_or_override(
                    GraphItemKind::Group,
                    group_name.clone(),
                    DEFAULT_ITEM_ID,
                    DEFAULT_ITEM_ID,
                ) {
                    errors
                        .push(GroupItemError::from_with_id(DEFAULT_ITEM_ID, group_name, e).into());
                    validate &= true;
                }

                (validate, errors)
            },
        );
        if !result {
            unreachable!(
                "fail build with error: {}",
                errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
        Grafo {
            group_arena: group_store,
            node_arena: Default::default(),
            edge_arena: Default::default(),
            resolver,
            layout,
        }
    }

    /// build to Grafo with specify group as root group.
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
                if let Err(e) = resolver.insert_graph_item_id_or_override(
                    GraphItemKind::Group,
                    name.clone(),
                    DEFAULT_ITEM_ID,
                    DEFAULT_ITEM_ID,
                ) {
                    errors.push(GroupItemError::from_with_id(DEFAULT_ITEM_ID, name, e).into());
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

impl<Name: NameType> DisplayAsJson for Grafo<Name> {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"resolver\": ")?;
        self.resolver.fmt_as_json(f)?;
        write!(f, ", \"group_items\": ")?;
        self.group_arena.fmt_as_json(f)?;
        write!(f, ", \"node_items\": ")?;
        self.node_arena.fmt_as_json(f)?;
        write!(f, ", \"edge_items\": ")?;
        self.edge_arena.fmt_as_json(f)?;
        write!(f, ", \"layout_items\": ")?;
        self.layout.fmt_as_json(f)?;
        write!(f, "}}")
    }
}

impl<Name: NameType> std::fmt::Display for Grafo<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grafo")?;
        self.fmt_as_json(f)
    }
}

// TODO Layout関係のメソッド
impl<Name: NameType> Grafo<Name> {
    //
    // initializer
    //

    /// wrapper function for builder with default group. If you want to build with your group, use `GrafoBuilder`'s method.
    pub fn with_no_name_default_group() -> Self {
        GrafoBuilder::new().build_with_no_name_default_group()
    }

    /// wrapper function for builder with default group which has name. If you want to build with your group, use `GrafoBuilder`'s method.
    pub fn with_name_default_group<S: Into<Name>>(group_name: S) -> Self {
        GrafoBuilder::new().build_with_name_default_group(group_name)
    }

    /// wrapper function for builder with default group which has name as optional. If you want to build with your group, use `GrafoBuilder`'s method.
    pub fn with_default_group<S: Into<Name>>(group_name: Option<S>) -> Self {
        let builder = GrafoBuilder::new();
        match group_name {
            None => builder.build_with_no_name_default_group(),
            Some(name) => builder.build_with_name_default_group(name),
        }
    }

    //
    // pusher
    //

    /// push group. Group is specified as graph item. Group can has nodes and edge as graph item.<br/>
    /// return value is pair of the check flag for push result and the warning or error when build the item.
    pub fn push_group(&mut self, builder: GroupItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.group_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let GroupItemOption { name } = option;
                if let Err(e) = resolver.insert_graph_item_id_or_override(
                    kind,
                    name.clone(),
                    belong_group_id,
                    item_id,
                ) {
                    errors.push(GroupItemError::from_with_id(item_id, name, e).into());
                    validate &= true;
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

    /// push node. Node is specified as graph item. Node is also called vertex in Graph Theory.<br/>
    /// return value is pair of the check flag for push result and the warning or error when build the item.
    pub fn push_node(&mut self, builder: NodeItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.node_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let NodeItemOption { name } = option;
                if let Err(e) = resolver.insert_graph_item_id_or_override(
                    kind,
                    name.clone(),
                    belong_group_id,
                    item_id,
                ) {
                    errors.push(NodeItemError::from_with_id(item_id, name, e).into());
                    validate &= true;
                }

                (validate, errors)
            },
        )
    }

    /// push edge. Edge is specified as graph item. Edge can has endpoints which have name from a graph item to a graph item.<br/>
    /// return value is pair of the check flag for push result and the warning or error when build the item.
    pub fn push_edge(&mut self, builder: EdgeItemBuilder<Name>) -> (bool, Vec<GrafoError<Name>>) {
        self.edge_arena.push(
            &mut self.resolver,
            builder,
            |resolver, kind, belong_group_id, item_id, option| {
                let mut errors: Vec<GrafoError<Name>> = Vec::new();
                let mut validate = true;
                let EdgeItemOption { name } = option;
                if let Err(e) = resolver.insert_graph_item_id_or_override(
                    kind,
                    name.clone(),
                    belong_group_id,
                    item_id,
                ) {
                    errors.push(EdgeItemError::from_with_id(item_id, name, e).into());
                    validate &= true;
                }

                (validate, errors)
            },
        )
    }

    //
    // getter
    //

    /// get reference indexes for names and hierarchy tree for group id
    pub fn resolver(&self) -> &Resolver<Name> {
        &self.resolver
    }

    /// get root group item. This method is usually success you get item.
    pub fn get_root_group_item(&self) -> Option<&GroupItem> {
        self.group_arena.get_default()
    }

    /// get group item of item_id belonging to group having id of belong_group_id.<br/>
    /// Route group's item_id is same to belong_group_id.
    pub fn get_group_item(&self, belong_group_id: GroupId, item_id: ItemId) -> Option<&GroupItem> {
        self.group_arena.get(belong_group_id, item_id)
    }

    /// get node item of item_id belonging to group having id of belong_group_id.
    pub fn get_node_item(&self, belong_group_id: GroupId, item_id: ItemId) -> Option<&NodeItem> {
        self.node_arena.get(belong_group_id, item_id)
    }

    /// get edge item of item_id belonging to group having id of belong_group_id.
    pub fn get_edge_item(&self, belong_group_id: GroupId, item_id: ItemId) -> Option<&EdgeItem> {
        self.edge_arena.get(belong_group_id, item_id)
    }

    //
    // iterator
    //

    /// iter for all Group item. This iterator sorted by ItemId.
    pub fn get_group_item_iter_all(&self) -> IterGroupByAll<GroupItem> {
        self.group_arena.iter_all()
    }

    /// iter for all Group item grouping by specified groups. This iterator sorted by ItemId.
    pub fn get_group_item_iter_group_by_list(
        &self,
        groups: &[GroupId],
    ) -> IterGroupByList<GroupItem> {
        let mut iter = self.group_arena.iter_group_by_list(groups);
        if groups.contains(&DEFAULT_ITEM_ID) {
            // group item having id which is equal to DEFAULT_ITEM_ID is belong to self. So remove.
            let _next = iter.next();
        }
        iter
    }

    /// iter for all Group item grouping by specified group_id. This iterator sorted by ItemId
    pub fn get_group_item_iter_group_by_id(&self, group_id: GroupId) -> IterGroupById<GroupItem> {
        let mut iter = self.group_arena.iter_group_by_id(group_id);
        if group_id == DEFAULT_ITEM_ID {
            // group item having id which is equal to DEFAULT_ITEM_ID is belong to self. So remove.
            let _next = iter.next();
        }
        iter
    }

    /// iter for all Node item. This iterator sorted by ItemId.
    pub fn get_node_item_iter_all(&self) -> IterGroupByAll<NodeItem> {
        self.node_arena.iter_all()
    }

    /// iter for all Node item grouping by specified groups. This iterator sorted by ItemId.
    pub fn get_node_item_iter_group_by_list(
        &self,
        groups: &[GroupId],
    ) -> IterGroupByList<NodeItem> {
        self.node_arena.iter_group_by_list(groups)
    }

    /// iter for all Node item grouping by specified group_id. This iterator sorted by ItemId
    pub fn get_node_item_iter_group_by_id(&self, group_id: GroupId) -> IterGroupById<NodeItem> {
        self.node_arena.iter_group_by_id(group_id)
    }

    /// iter for all Edge item. This iterator sorted by ItemId.
    pub fn get_edge_item_iter_all(&self) -> IterGroupByAll<EdgeItem> {
        self.edge_arena.iter_all()
    }

    /// iter for all Edge item grouping by specified groups. This iterator sorted by ItemId.
    pub fn get_edge_item_iter_group_by_list(
        &self,
        groups: &[GroupId],
    ) -> IterGroupByList<EdgeItem> {
        self.edge_arena.iter_group_by_list(groups)
    }

    /// iter for all Edge item grouping by specified group_id. This iterator sorted by ItemId
    pub fn get_edge_item_iter_group_by_id(&self, group_id: GroupId) -> IterGroupById<EdgeItem> {
        self.edge_arena.iter_group_by_id(group_id)
    }

    //
    // count
    //

    /// count all of Group items
    pub fn count_all_of_group(&self) -> usize {
        self.group_arena.count_all()
    }

    /// count all Group items in specified group
    pub fn count_group_items_group_by(&self, group_id: GroupId) -> usize {
        if group_id == DEFAULT_ITEM_ID {
            // root group belong to self. So removed. But arena's count do not remove.
            self.group_arena.count_by(group_id).saturating_sub(1)
        } else {
            self.group_arena.count_by(group_id)
        }
    }

    /// count all of Node items
    pub fn count_all_of_node(&self) -> usize {
        self.node_arena.count_all()
    }

    /// count all Node items in specified group
    pub fn count_node_items_group_by(&self, group_id: GroupId) -> usize {
        self.node_arena.count_by(group_id)
    }

    /// count all of Edge items
    pub fn count_all_of_edge(&self) -> usize {
        self.edge_arena.count_all()
    }

    /// count all Edge items in specified group
    pub fn count_edge_items_group_by(&self, group_id: GroupId) -> usize {
        self.edge_arena.count_by(group_id)
    }

    //
    // checker
    //

    /// check has Group item
    pub fn group_is_empty(&self) -> bool {
        self.group_arena.is_empty_all()
    }

    /// check has Group item in specified group
    pub fn group_is_empty_by(&self, group_id: GroupId) -> bool {
        if group_id == DEFAULT_ITEM_ID {
            // root group belong to self. So removed. But arena's count do not remove.
            self.count_group_items_group_by(group_id) == 0
        } else {
            self.group_arena.is_empty_by(group_id)
        }
    }

    /// check has Node item
    pub fn node_is_empty(&self) -> bool {
        self.node_arena.is_empty_all()
    }

    /// check has Node item in specified group
    pub fn node_is_empty_by(&self, group_id: GroupId) -> bool {
        self.node_arena.is_empty_by(group_id)
    }

    /// check has Edge item
    pub fn edge_is_empty(&self) -> bool {
        self.edge_arena.is_empty_all()
    }

    /// check has Edge item in specified group
    pub fn edge_is_empty_by(&self, group_id: GroupId) -> bool {
        self.edge_arena.is_empty_by(group_id)
    }
}

#[cfg(test)]
mod test {
    mod grafo_builder {
        use crate::grafo::core::graph_item::group::GroupItemBuilder;
        use crate::grafo::graph_item::group::GroupItemError;
        use crate::grafo::graph_item::GraphItemBuilderBase;
        use crate::grafo::{GrafoError, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};

        type Graph = NameStrGrafo;
        type GraphBuilder = NameStrGrafoBuilder;
        type GraphError = NameStrGrafoError;

        #[test]
        fn build_grafo_fail() {
            let mut group_builder = GroupItemBuilder::new();
            group_builder.set_belong_group("root");
            assert_eq!(
                GraphBuilder::new().build_with_user_group(group_builder),
                Err(vec![
                    GroupItemError::CannotSpecifyBelongGroupForRoot(0, None, "root".to_string())
                        .into(),
                    GrafoError::FailBuildGrafo,
                ])
            );
        }

        #[test]
        fn push_default_no_name_group_success() {
            let user_default_graph =
                GraphBuilder::new().build_with_user_group(GroupItemBuilder::new());
            let default_graph = GraphBuilder::new().build_with_no_name_default_group();
            assert_eq!(user_default_graph, Ok(default_graph));
            assert!(user_default_graph.is_ok());
        }

        #[test]
        fn push_default_name_group_success() {
            let mut group_builder = GroupItemBuilder::new();
            group_builder.set_name("root");
            let user_default_graph = GraphBuilder::new().build_with_user_group(group_builder);
            let default_graph = GraphBuilder::new().build_with_name_default_group("root");
            assert_eq!(user_default_graph, Ok(default_graph));
            assert!(user_default_graph.is_ok());
        }
    }

    mod group {
        use crate::grafo::core::graph_item::group::GroupItemBuilder;
        use crate::grafo::graph_item::group::GroupItemError;
        use crate::grafo::graph_item::node::NodeItemBuilder;
        use crate::grafo::graph_item::{GraphItemBase, GraphItemBuilderBase};
        use crate::grafo::{NameIdError, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
        use crate::util::alias::GroupId;
        use crate::util::item_base::ItemBase;
        use crate::util::kind::GraphItemKind;

        type Graph = NameStrGrafo;
        type GraphBuilder = NameStrGrafoBuilder;
        type GraphError = NameStrGrafoError;

        #[test]
        fn push_two_group_success_and_push_node_each_group() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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

            assert_eq!(graph.group_arena.count_all(), 3);

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

            assert_eq!(graph.node_arena.count_all(), 3);
        }

        #[test]
        pub fn push_group_success_to_not_root_group() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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

            assert_eq!(graph.group_arena.count_all(), 3);
        }

        #[test]
        fn push_group_success_with_name_override() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
                        Some("group".to_string()),
                        NameIdError::AlreadyExist(GraphItemKind::Group, "group".to_string()),
                    )
                    .into(),
                    GroupItemError::NameIdError(
                        2,
                        Some("group".to_string()),
                        NameIdError::Override(GraphItemKind::Group, "group".to_string()),
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

            assert_eq!(graph.group_arena.count_all(), 3);
        }

        #[test]
        fn build_group_fail() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

            let mut group_builder = GroupItemBuilder::new();
            group_builder.set_belong_group("hoge");
            let (result, errors) = graph.push_group(group_builder);
            assert!(!result);
            assert_eq!(
                errors,
                vec![
                    GroupItemError::NameIdError(
                        1,
                        None,
                        NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string()),
                    )
                    .into(),
                    GroupItemError::FailResolveBelongGroup(1, None, Some("hoge".to_string()))
                        .into(),
                ]
            );
        }

        // iterator
        const GROUP_DIVIDE_COUNT: usize = 5; // > 0
        fn make_template_graph() -> Graph {
            // Group hierarchy tree
            //   root:               0
            //         1  2    3        4            5
            // bottom: 6  7 8  9 10 11  12 13 14 15  16 17 18 19 20

            let mut graph = Graph::with_name_default_group("group 0");
            for i in 1..=GROUP_DIVIDE_COUNT {
                let mut builder = GroupItemBuilder::new();
                builder.set_belong_group("group 0");
                builder.set_name(format!("group {}", i));
                let (result, errors) = graph.push_group(builder);
                assert!(result);
                assert_eq!(errors, vec![]);
            }

            for i in 1..=GROUP_DIVIDE_COUNT {
                for j in 0..i {
                    let mut child_builder = GroupItemBuilder::new();
                    child_builder.set_belong_group(format!("group {}", i));
                    child_builder.set_name(format!("\"group {}\"'s child group {}", i, j));
                    let (result, errors) = graph.push_group(child_builder);
                    assert!(result);
                    assert_eq!(errors, vec![]);
                }
            }
            graph
        }

        #[test]
        fn iter_group_by_list_has_root_group() {
            let graph = make_template_graph();
            let group_list: Vec<GroupId> = (0..GROUP_DIVIDE_COUNT)
                .filter(|i| i % 2 == 0)
                .chain(vec![100])
                .collect();
            let mut iter = graph.get_group_item_iter_group_by_list(&group_list);
            assert_eq!(
                iter.using_groups(),
                (0..GROUP_DIVIDE_COUNT)
                    .filter(|i| i % 2 == 0)
                    .collect::<Vec<GroupId>>()
            );
            let current = iter.next().map(|(i, _)| i);
            // root group belong to root group. So removed.
            assert_ne!(current, Some(0).as_ref());
        }

        #[test]
        fn iter_group_by_list_not_has_root_group() {
            let graph = make_template_graph();
            let group_list: Vec<GroupId> = (1..GROUP_DIVIDE_COUNT)
                .filter(|i| i % 2 == 1)
                .chain(vec![100])
                .collect();
            let mut iter = graph.get_group_item_iter_group_by_list(&group_list);
            assert_eq!(
                iter.using_groups(),
                (1..GROUP_DIVIDE_COUNT)
                    .filter(|i| i % 2 == 1)
                    .collect::<Vec<GroupId>>()
            );
            let current = iter.next().map(|(i, _)| i);
            // root group belong to root group. So removed.
            assert_ne!(current, Some(0).as_ref());
        }

        #[test]
        fn iter_group_by_id_is_root_group() {
            let graph = make_template_graph();
            let mut iter = graph.get_group_item_iter_group_by_id(0);
            assert_ne!(iter.next().map(|(i, _)| i), Some(0).as_ref());
        }

        #[test]
        fn iter_group_by_id_is_not_has_root_group() {
            let graph = make_template_graph();
            let mut iter = graph.get_group_item_iter_group_by_id(usize::max_value());
            assert_ne!(iter.next().map(|(i, _)| i), Some(0).as_ref());
        }
    }

    mod node {
        use crate::grafo::graph_item::edge::EdgeItemBuilder;
        use crate::grafo::graph_item::node::{NodeItemBuilder, NodeItemError};
        use crate::grafo::graph_item::GraphItemBuilderBase;
        use crate::grafo::{NameIdError, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
        use crate::util::kind::GraphItemKind;

        type Graph = NameStrGrafo;
        type GraphBuilder = NameStrGrafoBuilder;
        type GraphError = NameStrGrafoError;

        const ITERATE_COUNT: usize = 10;

        #[test]
        fn push_node_success() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

            for i in 0..2 * ITERATE_COUNT {
                let mut node_builder = NodeItemBuilder::new();
                if i % 2 == 0 {
                    node_builder.set_name(format!("{}", i));
                }
                let (result, errors) = graph.push_node(node_builder);
                assert!(result);
                assert_eq!(errors, Vec::<GraphError>::new());
            }

            assert_eq!(graph.node_arena.count_all(), 2 * ITERATE_COUNT);
            assert_eq!(
                graph
                    .resolver
                    .count_usable_graph_item_names_by(GraphItemKind::Node),
                ITERATE_COUNT
            );
        }

        #[test]
        fn push_node_success_with_name_override() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
                        Some("node".to_string()),
                        NameIdError::AlreadyExist(GraphItemKind::Node, "node".to_string()),
                    )
                    .into(),
                    NodeItemError::NameIdError(
                        2,
                        Some("node".to_string()),
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
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

            let mut node_builder = NodeItemBuilder::new();
            node_builder.set_belong_group("hoge");
            let (result, errors) = graph.push_node(node_builder);
            assert!(!result);
            assert_eq!(
                errors,
                vec![
                    NodeItemError::NameIdError(
                        1,
                        None,
                        NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string()),
                    )
                    .into(),
                    NodeItemError::FailResolveBelongGroup(1, None, Some("hoge".to_string())).into(),
                ]
            );
        }

        #[test]
        fn push_edge_success() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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

            assert_eq!(graph.edge_arena.count_all(), ITERATE_COUNT);
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
    }

    mod edge {
        use crate::grafo::core::graph_item::group::GroupItemBuilder;
        use crate::grafo::graph_item::edge::{EdgeItemBuilder, EdgeItemError};
        use crate::grafo::graph_item::node::NodeItemBuilder;
        use crate::grafo::graph_item::GraphItemBuilderBase;
        use crate::grafo::{NameIdError, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
        use crate::util::kind::GraphItemKind;

        type Graph = NameStrGrafo;
        type GraphBuilder = NameStrGrafoBuilder;
        type GraphError = NameStrGrafoError;

        #[test]
        fn push_edge_success_edges_on_same_endpoints() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
        fn push_edge_success_with_name_override() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
                        Some("edge".to_string()),
                        NameIdError::AlreadyExist(GraphItemKind::Edge, "edge".to_string()),
                    )
                    .into(),
                    EdgeItemError::NameIdError(
                        2,
                        Some("edge".to_string()),
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
        fn push_edge_success_group_endpoint() {
            let mut graph = GraphBuilder::new().build_with_name_default_group("root");

            let mut group_builder_1 = GroupItemBuilder::new();
            group_builder_1.set_name("group 1");
            let (result, errors) = graph.push_group(group_builder_1);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut group_builder_2 = GroupItemBuilder::new();
            group_builder_2.set_name("group 2");
            let (result, errors) = graph.push_group(group_builder_2);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut edge_builder = EdgeItemBuilder::new();
            edge_builder.set_belong_group("root");
            edge_builder.set_start_endpoint(GraphItemKind::Group, "group 1");
            edge_builder.set_end_endpoint(GraphItemKind::Group, "group 2");
            let (result, errors) = graph.push_edge(edge_builder);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());
        }

        #[test]
        fn push_edges_endpoints_is_not_same_group() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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

            assert_eq!(graph.group_arena.count_all(), 3);

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

            assert_eq!(graph.node_arena.count_all(), 2);

            // edge
            // in root group
            let mut edge_builder_1 = EdgeItemBuilder::new();
            edge_builder_1.set_start_endpoint(GraphItemKind::Node, "node_1");
            edge_builder_1.set_end_endpoint(GraphItemKind::Node, "node_2");
            let (result, errors) = graph.push_edge(edge_builder_1);
            // error
            assert!(!result);
            assert_eq!(
                errors,
                vec![EdgeItemError::InappropriateGroup(1, None, None).into()]
            );

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

            assert_eq!(graph.edge_arena.count_all(), 2);
        }

        #[test]
        fn build_edge_fail_inappropriate_node_as_endpoint_belong_group() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

            let mut group_builder_1 = GroupItemBuilder::new();
            group_builder_1.set_name("group 1");
            let (result, errors) = graph.push_group(group_builder_1);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut group_builder_2 = GroupItemBuilder::new();
            group_builder_2.set_name("group 2");
            let (result, errors) = graph.push_group(group_builder_2);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut node_builder_1 = NodeItemBuilder::new();
            node_builder_1.set_belong_group("group 1");
            node_builder_1.set_name("node 1");
            let (result, errors) = graph.push_node(node_builder_1);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut node_builder_2 = NodeItemBuilder::new();
            node_builder_2.set_belong_group("group 2");
            node_builder_2.set_name("node 2");
            let (result, errors) = graph.push_node(node_builder_2);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut edge_builder = EdgeItemBuilder::new();
            edge_builder.set_start_endpoint(GraphItemKind::Node, "node 1");
            edge_builder.set_end_endpoint(GraphItemKind::Node, "node 2");
            let (result, errors) = graph.push_edge(edge_builder);
            assert!(!result);
            assert_eq!(
                errors,
                vec![EdgeItemError::InappropriateGroup(1, None, None).into()]
            );
        }

        #[test]
        fn build_edge_fail_inappropriate_group_as_endpoint_belong_group() {
            let mut graph = GraphBuilder::new().build_with_name_default_group("root");

            let mut group_builder_1 = GroupItemBuilder::new();
            group_builder_1.set_name("group 1");
            let (result, errors) = graph.push_group(group_builder_1);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut group_builder_2 = GroupItemBuilder::new();
            group_builder_2.set_belong_group("group 1");
            group_builder_2.set_name("group 2");
            let (result, errors) = graph.push_group(group_builder_2);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut group_builder_3 = GroupItemBuilder::new();
            group_builder_3.set_name("group 3");
            let (result, errors) = graph.push_group(group_builder_3);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut group_builder_4 = GroupItemBuilder::new();
            group_builder_4.set_belong_group("group 3");
            group_builder_4.set_name("group 4");
            let (result, errors) = graph.push_group(group_builder_4);
            assert!(result);
            assert_eq!(errors, Vec::<GraphError>::new());

            let mut edge_builder = EdgeItemBuilder::new();
            edge_builder.set_belong_group("root");
            edge_builder.set_start_endpoint(GraphItemKind::Group, "group 2");
            edge_builder.set_end_endpoint(GraphItemKind::Group, "group 4");
            let (result, errors) = graph.push_edge(edge_builder);
            assert!(!result);
            assert_eq!(
                errors,
                vec![EdgeItemError::InappropriateGroup(1, None, Some("root".to_string())).into()]
            );
        }

        #[test]
        fn build_edge_fail_not_found_belong_group_name() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
                        None,
                        NameIdError::NotExist(GraphItemKind::Group, "hoge".to_string()),
                    )
                    .into(),
                    EdgeItemError::FailResolveBelongGroup(1, None, Some("hoge".to_string())).into(),
                ]
            );
        }

        #[test]
        fn build_edge_fail_specify_cannot_usable_end_endpoint() {
            let mut graph = GraphBuilder::new().build_with_name_default_group("root group");

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
                        None,
                        "root group".to_string(),
                    )
                    .into(),
                    EdgeItemError::FailResolveEndEndpoint(
                        1,
                        None,
                        Some((GraphItemKind::Group, "root group".to_string())),
                    )
                    .into(),
                ]
            );
        }

        #[test]
        fn build_edge_fail_cannot_found_end_endpoint() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
                        None,
                        NameIdError::NotExist(GraphItemKind::Node, "not exist".to_string()),
                    )
                    .into(),
                    EdgeItemError::FailResolveEndEndpoint(
                        1,
                        None,
                        Some((GraphItemKind::Node, "not exist".to_string())),
                    )
                    .into(),
                ]
            );
        }

        #[test]
        fn build_edge_fail_not_specify_end_endpoint() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

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
                    EdgeItemError::NotSpecifyEndEndpoint(1, None, None).into(),
                    EdgeItemError::FailResolveEndEndpoint(1, None, None).into(),
                ]
            );
        }

        #[test]
        fn build_edge_fail_not_specify_endpoints() {
            let mut graph = GraphBuilder::new().build_with_no_name_default_group();

            let (result, errors) = graph.push_edge(EdgeItemBuilder::new());
            assert!(!result);
            assert_eq!(
                errors,
                vec![
                    EdgeItemError::NotSpecifyStartEndpoint(1, None, None).into(),
                    EdgeItemError::NotSpecifyEndEndpoint(1, None, None).into(),
                    EdgeItemError::FailResolveStartEndpoint(1, None, None).into(),
                    EdgeItemError::FailResolveEndEndpoint(1, None, None).into(),
                ]
            );
        }
    }
}
