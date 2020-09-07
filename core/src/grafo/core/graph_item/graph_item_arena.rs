//! item pool for graph item

use std::collections::BTreeMap;

use crate::grafo::graph_item::{GraphBuilderErrorBase, GraphItemBase, GraphItemBuilderBase};
use crate::grafo::GrafoError;
use crate::grafo::Resolver;
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::item_base::HasItemBuilderMethod;
use crate::util::iter;
use crate::util::kind::GraphItemKind;
use crate::util::name_type::NameType;
use crate::util::writer::DisplayAsJson;

/// item pool
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ItemArena<I> {
    id_counter: ItemId,
    /// (GroupId, ItemId) => Item
    arena: BTreeMap<GroupId, BTreeMap<ItemId, I>>,
}

impl<I> Default for ItemArena<I> {
    /// initialize without log
    fn default() -> Self {
        ItemArena {
            id_counter: DEFAULT_ITEM_ID,
            arena: Default::default(),
        }
    }
}

impl<I: DisplayAsJson + GraphItemBase> DisplayAsJson for ItemArena<I> {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"items\": [")?;
        for (i, (_, item)) in self.iter_all().enumerate() {
            if i == 0 {
                item.fmt_as_json(f)?;
            } else {
                write!(f, ", ")?;
                item.fmt_as_json(f)?;
            }
        }
        write!(f, "]}}")
    }
}

impl<I: std::fmt::Display + DisplayAsJson + GraphItemBase> std::fmt::Display for ItemArena<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GraphItemArena")?;
        self.fmt_as_json(f)
    }
}

impl<I: GraphItemBase> ItemArena<I> {
    /// initialize
    pub fn new() -> Self {
        ItemArena::default()
    }

    //
    // helper
    //

    /// get the next index with increment as soon as possible
    fn get_push_id(&mut self) -> ItemId {
        self.id_counter += 1;
        self.id_counter
    }

    //
    // setter
    //

    /// push the item into arena with action for conclusion<br/>
    /// F: fn(item_kind, group_id, Result<(item_id, extension), err>)
    pub(crate) fn push<
        Name: NameType,
        F,
        O,
        E: GraphBuilderErrorBase<Name>,
        B: GraphItemBuilderBase<Name, Item = I, ItemError = E>
            + HasItemBuilderMethod<Name, Item = I, ItemOption = O, ItemError = E>,
    >(
        &mut self,
        resolver: &mut Resolver<Name>,
        item_builder: B,
        action: F,
    ) -> (bool, Vec<GrafoError<Name>>)
    where
        F: FnOnce(
            &mut Resolver<Name>,
            GraphItemKind,
            GroupId,
            ItemId,
            B::ItemOption,
        ) -> (bool, Vec<GrafoError<Name>>),
    {
        let push_id = self.get_push_id();
        let (item_option, mut errors) = item_builder.build(push_id, resolver);
        match item_option {
            None => (false, errors),
            Some((item, option)) => {
                let group_id = item.get_belong_group_id();
                let (result, action_errors) =
                    action(resolver, item.get_kind(), group_id, push_id, option);
                errors.extend(action_errors);
                if result {
                    self.arena
                        .entry(group_id)
                        .or_default()
                        .insert(push_id, item);
                }
                (result, errors)
            }
        }
    }

    /// item getter
    pub fn get(&self, group_id: GroupId, index: ItemId) -> Option<&I> {
        match self.arena.get(&group_id) {
            None => None,
            Some(map) => map.get(&index),
        }
    }

    //
    // reference
    //

    /// count for all of item
    pub fn count_all(&self) -> usize {
        self.arena.iter().map(|(_, map)| map.len()).sum()
    }

    /// count for all of item grouping by group id
    pub fn count_by(&self, group_id: GroupId) -> usize {
        self.arena
            .get(&group_id)
            .map(|map| map.len())
            .unwrap_or_else(|| 0)
    }

    /// item pool is empty
    pub fn is_empty_all(&self) -> bool {
        self.arena.iter().map(|(_, map)| map.is_empty()).all(|b| b)
    }

    /// item pool is empty grouping by group id
    pub fn is_empty_by(&self, group_id: GroupId) -> bool {
        self.arena
            .get(&group_id)
            .map(|map| map.is_empty())
            .unwrap_or_else(|| true)
    }

    //
    // iter or slice
    //

    /// iter for all item. This iterator sorted by ItemId.
    pub fn iter_all(&self) -> iter::IterLimitedByAllGroup<ItemId, I> {
        iter::IterLimitedByAllGroup::<ItemId, I>::from_btree_map(&self.arena)
    }

    /// iter for all item grouping by specified groups. This iterator sorted by ItemId.
    pub fn iter_group_by_list(
        &self,
        groups: &[GroupId],
    ) -> iter::IterLimitedByGroupList<GroupId, ItemId, I> {
        iter::IterLimitedByGroupList::<GroupId, ItemId, I>::from_btree_map(groups, &self.arena)
    }

    /// iter for all item grouping by specified group_id. This iterator sorted by ItemId
    pub fn iter_group_by_id(
        &self,
        group_id: GroupId,
    ) -> iter::IterLimitedByOneGroup<GroupId, ItemId, I> {
        iter::IterLimitedByOneGroup::<GroupId, ItemId, I>::from_btree_map(&group_id, &self.arena)
    }
}

impl<I: GraphItemBase + Default> ItemArena<I> {
    /// item id for default item
    fn get_default_index(&self) -> ItemId {
        DEFAULT_ITEM_ID
    }

    /// push the item into arena with action for conclusion
    pub(crate) fn push_default<Name: NameType, F, O: Default>(
        &mut self,
        resolver: &mut Resolver<Name>,
        action: F,
    ) -> (bool, Vec<GrafoError<Name>>)
    where
        F: FnOnce(
            &mut Resolver<Name>,
            GraphItemKind,
            GroupId,
            ItemId,
            O,
        ) -> (bool, Vec<GrafoError<Name>>),
    {
        let item = I::default();
        let group_id = item.get_belong_group_id();
        let push_id = self.get_default_index();
        let (result, errors) = action(resolver, item.get_kind(), group_id, push_id, O::default());
        if !result || !errors.is_empty() {
            return (false, errors);
        }

        self.arena
            .entry(group_id)
            .or_default()
            .insert(push_id, item);
        (true, errors)
    }

    /// push the item into arena with action for conclusion<br/>
    pub(crate) fn push_user_item_as_default<
        Name: NameType,
        F,
        O,
        E: GraphBuilderErrorBase<Name>,
        B: GraphItemBuilderBase<Name, Item = I, ItemError = E>
            + HasItemBuilderMethod<Name, Item = I, ItemOption = O, ItemError = E>,
    >(
        &mut self,
        resolver: &mut Resolver<Name>,
        item_builder: B,
        action: F,
    ) -> (bool, Vec<GrafoError<Name>>)
    where
        F: FnOnce(
            &mut Resolver<Name>,
            GraphItemKind,
            GroupId,
            ItemId,
            B::ItemOption,
        ) -> (bool, Vec<GrafoError<Name>>),
    {
        let push_id = self.get_default_index();
        let (item_option, mut errors) = item_builder.build(push_id, resolver);
        match item_option {
            None => (false, errors),
            Some((item, option)) => {
                let group_id = item.get_belong_group_id();
                let (result, action_errors) =
                    action(resolver, item.get_kind(), group_id, push_id, option);
                errors.extend(action_errors);
                if result {
                    self.arena
                        .entry(group_id)
                        .or_default()
                        .insert(push_id, item);
                }
                (result, errors)
            }
        }
    }

    /// item getter
    pub(crate) fn get_default(&self) -> Option<&I> {
        self.get(self.get_default_index(), self.get_default_index())
    }
}

#[cfg(test)]
mod test {
    mod definition {
        use std::error::Error;

        use crate::grafo::core::graph_item::{
            GraphBuilderErrorBase, GraphItemBase, GraphItemBuilderBase,
        };
        use crate::grafo::core::{NameIdError, Resolver};
        use crate::grafo::{GrafoError, ResolverError};
        use crate::util::alias::{GroupId, ItemId};
        use crate::util::item_base::{
            FromWithItemId, HasItemBuilderMethod, ItemBase, ItemBuilderBase, ItemBuilderResult,
            ItemErrorBase,
        };
        use crate::util::kind::{GraphItemKind, HasGraphItemKind};

        pub const ITERATE_COUNT: ItemId = 10;
        pub const TARGET_KIND: GraphItemKind = GraphItemKind::Node;

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub struct TargetItemBuilder {
            belong_group: Option<String>,
            name: Option<String>,
        }

        impl ItemBuilderBase<String> for TargetItemBuilder {
            type Item = TargetItem;
            type ItemError = TargetBuilderError;
        }

        impl GraphItemBuilderBase<String> for TargetItemBuilder {
            fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self {
                self.belong_group = Some(group.into());
                self
            }

            fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
                self.name = Some(name.into());
                self
            }
        }

        impl TargetItemBuilder {
            pub fn new() -> Self {
                TargetItemBuilder {
                    belong_group: None,
                    name: None,
                }
            }
            fn resolve_belong_group(
                &self,
                item_id: ItemId,
                resolver: &Resolver<String>,
                errors: &mut Vec<GrafoError<String>>,
                belong_group: Option<&str>,
            ) -> Option<GroupId> {
                match belong_group {
                    None => match resolver.get_root_group_id() {
                        Ok(id) => Some(id),
                        Err(e) => {
                            errors.push(e.into());
                            None
                        }
                    },
                    Some(belong_group_name) => {
                        let belong_group_result = resolver
                            .get_graph_item_id_pair(GraphItemKind::Group, belong_group_name);
                        match belong_group_result {
                            Ok((_belong_group_id, group_item_id)) => Some(group_item_id),
                            Err(err) => {
                                errors.push(
                                    TargetBuilderError::from_with_id(
                                        item_id,
                                        self.name.clone(),
                                        err,
                                    )
                                    .into(),
                                );
                                None
                            }
                        }
                    }
                }
            }
        }

        impl HasItemBuilderMethod<String> for TargetItemBuilder {
            type ItemOption = TargetItemOption;
            fn build(
                self,
                item_id: ItemId,
                resolver: &Resolver<String>,
            ) -> ItemBuilderResult<String, TargetItem, TargetItemOption> {
                assert_ne!(TARGET_KIND, GraphItemKind::Group);
                let mut errors: Vec<GrafoError<String>> = Vec::new();

                let group_id = (&self).resolve_belong_group(
                    item_id,
                    &resolver,
                    &mut errors,
                    self.belong_group.as_deref(),
                );
                if group_id.is_none() {
                    errors
                        .push(TargetBuilderError::NotFindGroup(item_id, self.name.clone()).into());
                    return (None, errors);
                }
                let group_id = group_id.unwrap();

                let TargetItemBuilder {
                    belong_group: _,
                    name,
                } = self;
                if errors.is_empty() {
                    (
                        Some((
                            TargetItem {
                                belong_group_id: group_id,
                                item_id,
                            },
                            TargetItemOption {
                                belong_group_id: group_id,
                                name,
                            },
                        )),
                        errors,
                    )
                } else {
                    (None, errors)
                }
            }
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub struct TargetItemOption {
            pub belong_group_id: GroupId,
            pub name: Option<String>,
        }

        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum TargetBuilderError {
            BuildFail(ItemId, Option<String>),
            NotFindGroup(ItemId, Option<String>),
        }

        impl From<TargetBuilderError> for GrafoError<String> {
            fn from(_: TargetBuilderError) -> GrafoError<String> {
                unreachable!()
            }
        }

        impl HasGraphItemKind for TargetBuilderError {
            fn kind() -> GraphItemKind {
                TARGET_KIND
            }
        }

        impl std::fmt::Display for TargetBuilderError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use TargetBuilderError::*;
                match &self {
                    BuildFail(_, _) => {
                        self.fmt_header(f)?;
                        write!(f, "fail build item")
                    }
                    NotFindGroup(_, _) => {
                        self.fmt_header(f)?;
                        write!(f, "fail found belong group")
                    }
                }
            }
        }

        impl Error for TargetBuilderError {}

        impl ItemErrorBase<String> for TargetBuilderError {}

        impl FromWithItemId<NameIdError<String, GraphItemKind>, String> for TargetBuilderError {
            fn from_with_id(
                _: ItemId,
                name: Option<String>,
                _: NameIdError<String, GraphItemKind>,
            ) -> Self {
                unimplemented!()
            }
        }

        impl FromWithItemId<ResolverError, String> for TargetBuilderError {
            fn from_with_id(_: ItemId, name: Option<String>, _: ResolverError) -> Self {
                unimplemented!()
            }
        }

        impl GraphBuilderErrorBase<String> for TargetBuilderError {
            fn get_item_id(&self) -> &usize {
                match self {
                    TargetBuilderError::BuildFail(i, _) => i,
                    TargetBuilderError::NotFindGroup(i, _) => i,
                }
            }

            fn get_item_name(&self) -> &Option<String> {
                match self {
                    TargetBuilderError::BuildFail(_, name) => name,
                    TargetBuilderError::NotFindGroup(_, name) => name,
                }
            }
        }

        #[derive(Debug, Eq, PartialEq, Clone, Copy)]
        pub struct TargetItem {
            belong_group_id: GroupId,
            item_id: ItemId,
        }

        impl HasGraphItemKind for TargetItem {
            fn kind() -> GraphItemKind {
                TARGET_KIND
            }
        }

        impl ItemBase for TargetItem {
            fn get_item_id(&self) -> ItemId {
                self.item_id
            }
        }

        impl GraphItemBase for TargetItem {
            fn get_belong_group_id(&self) -> ItemId {
                self.belong_group_id
            }
        }
    }

    mod arena {
        use crate::grafo::graph_item::graph_item_arena::test::definition::TargetItem;
        use crate::grafo::graph_item::ItemArena;

        #[test]
        fn is_empty() {
            assert!(ItemArena::<TargetItem>::new().is_empty_all());
        }
    }

    mod with_name {
        use crate::grafo::graph_item::graph_item_arena::test::definition::{
            TargetBuilderError, TargetItem, TargetItemBuilder, TargetItemOption, ITERATE_COUNT,
            TARGET_KIND,
        };
        use crate::grafo::graph_item::{GraphItemBase, GraphItemBuilderBase, ItemArena};
        use crate::grafo::{GrafoError, NameIdError, Resolver};
        use crate::util::item_base::FromWithItemId;
        use crate::util::kind::test::graph_item_check_list;

        #[test]
        fn with_name_count() {
            let mut arena_mut = ItemArena::<TargetItem>::new();
            let mut resolver = Resolver::default();
            resolver.set_root_group_id(0).unwrap();
            for i in 0..ITERATE_COUNT {
                let mut builder = TargetItemBuilder::new();
                builder.set_name(format!("{}", i));
                let (result, errors) = arena_mut.push(
                    &mut resolver,
                    builder,
                    |resolver, kind, group_id, item_id, option| {
                        let mut errors: Vec<GrafoError<String>> = Vec::new();
                        let TargetItemOption {
                            belong_group_id: _,
                            name,
                        } = option;
                        if let Err(err) = resolver.insert_graph_item_id_or_override(
                            kind,
                            name.clone(),
                            group_id,
                            item_id,
                        ) {
                            errors
                                .push(TargetBuilderError::from_with_id(item_id, name, err).into());
                        }
                        (errors.is_empty(), errors)
                    },
                );
                assert_eq!(Vec::<GrafoError<String>>::new(), errors);
                assert!(result);
            }
            let arena = arena_mut;
            assert_eq!(arena.count_all(), ITERATE_COUNT as usize);
            for target in graph_item_check_list() {
                assert_eq!(
                    resolver.count_usable_graph_item_names_by(target),
                    if target == TARGET_KIND {
                        ITERATE_COUNT as usize
                    } else {
                        0
                    }
                );
            }
        }

        #[test]
        fn with_name_each_eq() {
            let mut arena_mut = ItemArena::<TargetItem>::new();
            let mut resolver = Resolver::default();
            resolver.set_root_group_id(0).unwrap();

            for i in 1..=ITERATE_COUNT {
                let mut builder = TargetItemBuilder::new();
                builder.set_name(format!("{}", i));
                let (result, errors) = arena_mut.push(
                    &mut resolver,
                    builder,
                    |resolver, kind, group_id, item_id, option| {
                        let mut errors: Vec<GrafoError<String>> = Vec::new();
                        let TargetItemOption {
                            belong_group_id: _,
                            name,
                        } = option;
                        if let Err(err) = resolver.insert_graph_item_id_or_override(
                            kind,
                            name.clone(),
                            group_id,
                            item_id,
                        ) {
                            errors.push(
                                TargetBuilderError::from_with_id(item_id, name.clone(), err).into(),
                            );
                        }

                        (errors.is_empty(), errors)
                    },
                );
                assert_eq!(Vec::<GrafoError<String>>::new(), errors);
                assert!(result);
            }
            let arena = arena_mut;
            for (item_id, item) in (&arena).iter_all() {
                for kind in graph_item_check_list() {
                    let name = format!("{}", item_id);
                    let ref_result = resolver.get_graph_item_id_pair(kind, &name);
                    if let Ok(success) = ref_result {
                        // デフォルトがitem_id = 0占有
                        assert_eq!(success, (item.get_belong_group_id(), *item_id));
                    } else {
                        assert_eq!(
                            ref_result,
                            Err(NameIdError::NotExist(kind, format!("{}", item_id)))
                        );
                    }
                }
            }
        }
    }

    mod mixed {
        use crate::grafo::graph_item::graph_item_arena::test::definition::{
            TargetBuilderError, TargetItem, TargetItemBuilder, TargetItemOption, ITERATE_COUNT,
            TARGET_KIND,
        };
        use crate::grafo::graph_item::{GraphItemBase, GraphItemBuilderBase, ItemArena};
        use crate::grafo::{GrafoError, NameIdError, Resolver};
        use crate::util::item_base::FromWithItemId;
        use crate::util::kind::test::graph_item_check_list;

        #[test]
        fn mixed_count() {
            let mut arena_mut = ItemArena::<TargetItem>::new();
            let mut resolver = Resolver::default();
            resolver.set_root_group_id(0).unwrap();
            for i in 1..=2 * ITERATE_COUNT {
                let mut builder = TargetItemBuilder::new();
                if i <= ITERATE_COUNT {
                    builder.set_name(format!("{}", i));
                }
                let (result, errors) = arena_mut.push(
                    &mut resolver,
                    builder,
                    |resolver, kind, group_id, item_id, option| {
                        let mut errors: Vec<GrafoError<String>> = Vec::new();
                        let TargetItemOption {
                            belong_group_id: _,
                            name,
                        } = option;
                        if let Err(err) = resolver.insert_graph_item_id_or_override(
                            kind,
                            name.clone(),
                            group_id,
                            item_id,
                        ) {
                            errors
                                .push(TargetBuilderError::from_with_id(item_id, name, err).into());
                        }

                        (errors.is_empty(), errors)
                    },
                );
                assert_eq!(Vec::<GrafoError<String>>::new(), errors);
                assert!(result)
            }
            let arena = arena_mut;
            assert_eq!(arena.count_all(), 2 * ITERATE_COUNT as usize);
            for target in graph_item_check_list() {
                assert_eq!(
                    resolver.count_usable_graph_item_names_by(target),
                    if target == TARGET_KIND {
                        ITERATE_COUNT as usize
                    } else {
                        0
                    }
                );
            }
        }

        #[test]
        fn mixed_each_eq() {
            let mut arena_mut = ItemArena::<TargetItem>::new();
            let mut resolver = Resolver::default();
            resolver.set_root_group_id(0).unwrap();
            for i in 1..=2 * ITERATE_COUNT {
                let mut builder = TargetItemBuilder::new();
                if i <= ITERATE_COUNT {
                    builder.set_name(format!("{}", i));
                }
                let (result, errors) = arena_mut.push(
                    &mut resolver,
                    builder,
                    |resolver, kind, group_id, item_id, option| {
                        let mut errors: Vec<GrafoError<String>> = Vec::new();
                        let TargetItemOption {
                            belong_group_id: _,
                            name,
                        } = option;
                        if let Err(err) = resolver.insert_graph_item_id_or_override(
                            kind,
                            name.clone(),
                            group_id,
                            item_id,
                        ) {
                            errors
                                .push(TargetBuilderError::from_with_id(item_id, name, err).into());
                        }

                        (errors.is_empty(), errors)
                    },
                );
                assert_eq!(Vec::<GrafoError<String>>::new(), errors);
                assert!(result);
            }
            let arena = arena_mut;
            for (item_id, item) in arena.iter_all() {
                for kind in graph_item_check_list() {
                    let name = format!("{}", item_id);
                    let ref_result = resolver.get_graph_item_id_pair(kind, &name);
                    if item_id <= &ITERATE_COUNT && kind == TARGET_KIND {
                        if let Ok(success) = &ref_result {
                            // デフォルトがitem_id = 0占有
                            assert_eq!(success, &(item.get_belong_group_id(), *item_id));
                        } else {
                            unreachable!("over count and not exist the name \"{}\"", name)
                        }
                    } else {
                        assert_eq!(
                            ref_result,
                            Err(NameIdError::NotExist(kind, format!("{}", item_id)))
                        );
                    }
                }
            }
        }
    }
}
