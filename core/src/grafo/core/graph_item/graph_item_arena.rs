//! item pool

use std::collections::btree_map::{Iter, Range};
use std::collections::BTreeMap;
use std::ops::{Bound, RangeBounds};

use crate::grafo::graph_item::{GraphBuilderErrorBase, GraphItemBase, GraphItemBuilderBase};
use crate::grafo::GrafoError;
use crate::grafo::Resolver;
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::item_base::HasItemBuilderMethod;
use crate::util::kind::GraphItemKind;
use crate::util::name_type::NameType;

/// item pool
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ItemArena<I> {
    id_counter: ItemId,
    /// (GroupId, ItemId) => Item
    arena: BTreeMap<(GroupId, ItemId), I>,
}

fn range_with_group(group_id: GroupId, bound: Bound<&ItemId>) -> Bound<(GroupId, ItemId)> {
    match bound {
        Bound::Included(item_id) => Bound::Included((group_id, *item_id)),
        Bound::Excluded(item_id) => Bound::Excluded((group_id, *item_id)),
        Bound::Unbounded => Bound::Unbounded,
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
                    self.arena.insert((group_id, push_id), item);
                }
                (result, errors)
            }
        }
    }

    /// item getter
    pub fn get(&self, group_id: GroupId, index: ItemId) -> Option<&I> {
        self.arena.get(&(group_id, index))
    }

    /// item getter by range
    pub fn range<R: RangeBounds<ItemId>>(
        &self,
        group_id: GroupId,
        range: R,
    ) -> Range<(GroupId, ItemId), I> {
        let start = range_with_group(group_id, range.start_bound());
        let end = range_with_group(group_id, range.end_bound());
        self.arena.range((start, end))
    }

    /// iter by filtering group_id
    pub fn filter_by_group<'a>(&'a self, group_id: GroupId) -> impl Iterator + 'a {
        self.iter()
            .filter_map(move |((item_group_id, _item_id), item)| {
                if item_group_id == &group_id {
                    Some(item)
                } else {
                    None
                }
            })
    }

    //
    // reference
    //

    /// count of item
    pub fn count(&self) -> usize {
        self.arena.len()
    }

    /// item pool is empty
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    //
    // iter or slice
    //

    /// to iterator
    pub fn iter(&self) -> Iter<(GroupId, ItemId), I> {
        self.arena.iter()
    }
}

impl<I: GraphItemBase + Default> ItemArena<I> {
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

        self.arena.insert((group_id, push_id), item);
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
                    self.arena.insert((group_id, push_id), item);
                }
                (result, errors)
            }
        }
    }

    /// item getter
    pub(crate) fn get_default(&self, group_id: GroupId) -> Option<&I> {
        self.arena.get(&(group_id, self.get_default_index()))
    }
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

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    use crate::grafo::core::graph_item::{
        GraphBuilderErrorBase, GraphItemBase, GraphItemBuilderBase, ItemArena,
    };
    use crate::grafo::core::{NameIdError, Resolver};
    use crate::grafo::GrafoError;
    use crate::util::alias::{GroupId, ItemId};
    use crate::util::item_base::{
        FromWithItemId, HasItemBuilderMethod, ItemBase, ItemBuilderBase, ItemBuilderResult,
        ItemErrorBase,
    };
    use crate::util::kind::test::graph_item_check_list;
    use crate::util::kind::{GraphItemKind, HasGraphItemKind};

    const ITERATE_COUNT: ItemId = 10;
    const TARGET_KIND: GraphItemKind = GraphItemKind::Node;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct TargetItemBuilder {
        belong_group: Option<String>,
        name: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    struct TargetItem {
        belong_group_id: GroupId,
        item_id: ItemId,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct TargetItemOption {
        belong_group_id: GroupId,
        name: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    enum TargetBuilderError {
        BuildFail(ItemId),
        NotFindGroup(ItemId),
    }

    impl<'a> Into<GrafoError<String>> for TargetBuilderError {
        fn into(self) -> GrafoError<String> {
            unimplemented!()
        }
    }

    impl HasGraphItemKind for TargetItem {
        fn kind() -> GraphItemKind {
            TARGET_KIND
        }
    }

    impl HasGraphItemKind for TargetBuilderError {
        fn kind() -> GraphItemKind {
            TARGET_KIND
        }
    }

    impl<'a> ItemBuilderBase<String> for TargetItemBuilder {
        type Item = TargetItem;
        type ItemError = TargetBuilderError;
    }

    impl<'a> GraphItemBuilderBase<String> for TargetItemBuilder {
        fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self {
            self.belong_group = Some(group.into());
            self
        }

        fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
            self.name = Some(name.into());
            self
        }
    }

    impl<'a> TargetItemBuilder {
        fn new() -> Self {
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
                    let belong_group_result =
                        resolver.get_graph_item_id_pair(GraphItemKind::Group, belong_group_name);
                    match belong_group_result {
                        Ok((_belong_group_id, group_item_id)) => Some(group_item_id),
                        Err(err) => {
                            errors.push(TargetBuilderError::from_with_id(item_id, err).into());
                            None
                        }
                    }
                }
            }
        }
    }

    impl<'a> HasItemBuilderMethod<String> for TargetItemBuilder {
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
                errors.push(TargetBuilderError::NotFindGroup(item_id).into());
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

    impl Display for TargetBuilderError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            use TargetBuilderError::*;
            match &self {
                BuildFail(id) => write!(f, "id: {} fail build item", id),
                NotFindGroup(id) => write!(f, "id: {} fail found belong group", id),
            }
        }
    }

    impl Error for TargetBuilderError {}

    impl<'a> ItemErrorBase<String> for TargetBuilderError {}

    impl<'a> FromWithItemId<NameIdError<String, GraphItemKind>> for TargetBuilderError {
        fn from_with_id(item_id: ItemId, from: NameIdError<String, GraphItemKind>) -> Self {
            unimplemented!()
        }
    }
    impl<'a> GraphBuilderErrorBase<String> for TargetBuilderError {}

    #[test]
    fn is_empty() {
        assert!(ItemArena::<TargetItem>::new().is_empty());
    }

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
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        if let Err(err) =
                            resolver.push_graph_item_value(kind, name, group_id, item_id)
                        {
                            errors.push(TargetBuilderError::from_with_id(item_id, err).into());
                        }
                    }
                    (errors.is_empty(), errors)
                },
            );
            assert_eq!(Vec::<GrafoError<String>>::new(), errors);
            assert!(result);
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), ITERATE_COUNT as usize);
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
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        if let Err(err) =
                            resolver.push_graph_item_value(kind, name, group_id, item_id)
                        {
                            errors.push(TargetBuilderError::from_with_id(item_id, err).into());
                        }
                    }

                    (errors.is_empty(), errors)
                },
            );
            assert_eq!(Vec::<GrafoError<String>>::new(), errors);
            assert!(result);
        }
        let arena = arena_mut;
        for (index, _item) in (&arena).iter() {
            for kind in graph_item_check_list() {
                let name = format!("{}", index.1);
                let ref_result = resolver.get_graph_item_id_pair(kind, &name);
                if let Ok(success) = ref_result {
                    // デフォルトがitem_id = 0占有
                    assert_eq!(success, *index);
                } else {
                    assert_eq!(
                        ref_result,
                        Err(NameIdError::NotExist(kind, format!("{}", index.1),))
                    );
                }
            }
        }
    }

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
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        if let Err(err) =
                            resolver.push_graph_item_value(kind, name, group_id, item_id)
                        {
                            errors.push(TargetBuilderError::from_with_id(item_id, err).into());
                        }
                    }

                    (errors.is_empty(), errors)
                },
            );
            assert_eq!(Vec::<GrafoError<String>>::new(), errors);
            assert!(result)
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), 2 * ITERATE_COUNT as usize);
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
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        if let Err(err) =
                            resolver.push_graph_item_value(kind, name, group_id, item_id)
                        {
                            errors.push(TargetBuilderError::from_with_id(item_id, err).into());
                        }
                    }

                    (errors.is_empty(), errors)
                },
            );
            assert_eq!(Vec::<GrafoError<String>>::new(), errors);
            assert!(result);
        }
        let arena = arena_mut;
        for (index, _item) in (&arena).iter() {
            for kind in graph_item_check_list() {
                let name = format!("{}", index.1);
                let ref_result = resolver.get_graph_item_id_pair(kind, &name);
                if index.1 <= ITERATE_COUNT && kind == TARGET_KIND {
                    if let Ok(success) = &ref_result {
                        // デフォルトがitem_id = 0占有
                        assert_eq!(success, index);
                    } else {
                        unreachable!("over count and not exist the name \"{}\"", name)
                    }
                } else {
                    assert_eq!(
                        ref_result,
                        Err(NameIdError::NotExist(kind, format!("{}", index.1),))
                    );
                }
            }
        }
    }
}
