//! item pool

use std::collections::btree_map::{Iter, Range};
use std::collections::BTreeMap;
use std::ops::{Bound, RangeBounds};
use std::sync::{Arc, Mutex};

use crate::grafo::core::item::{ItemBase, ItemBuilderBaseBuilderMethod, ItemErrorBase};
use crate::grafo::core::refindex::NameReference;
use crate::grafo::GrafoError;
use crate::util::alias::{GraphItemId, GroupId};
use crate::util::item_kind::ItemKind;

/// item pool
#[derive(Debug, Clone)]
pub(crate) struct ItemArena<I> {
    pushed_index: Arc<Mutex<GraphItemId>>,
    /// (GroupId, ItemId) => Item
    arena: BTreeMap<(GroupId, GraphItemId), I>,
}

fn range_with_group(
    group_id: GroupId,
    bound: Bound<&GraphItemId>,
) -> Bound<(GroupId, GraphItemId)> {
    match bound {
        Bound::Included(item_id) => Bound::Included((group_id, *item_id)),
        Bound::Excluded(item_id) => Bound::Excluded((group_id, *item_id)),
        Bound::Unbounded => Bound::Unbounded,
    }
}

impl<I: ItemBase> ItemArena<I> {
    /// initialize
    pub(crate) fn new() -> Self {
        ItemArena::default()
    }

    //
    // helper
    //

    /// get the next index with increment as soon as possible
    fn get_push_index(&mut self) -> GraphItemId {
        match self.pushed_index.lock() {
            Ok(mut pushed_index) => {
                let next_index: GraphItemId = *pushed_index;
                *pushed_index += 1;
                next_index
            }
            Err(e) => {
                panic!("fail lock error: {}", e);
            }
        }
    }

    //
    // setter
    //

    /// push the item into arena with action for conclusion<br/>
    /// F: fn(item_kind, group_id, Result<(item_id, extension), err>)
    pub(crate) fn push<
        F,
        O,
        E: ItemErrorBase,
        B: ItemBuilderBaseBuilderMethod<Item = I, ItemOption = O, BuildFailError = E>,
    >(
        &mut self,
        name_ref: &mut NameReference,
        item_builder: B,
        action: F,
    ) -> Option<Vec<GrafoError>>
    where
        F: FnOnce(
            &mut NameReference,
            ItemKind,
            GroupId,
            GraphItemId,
            B::ItemOption,
        ) -> Option<Vec<GrafoError>>,
    {
        let item_kind = B::kind();
        let group_id = item_builder.get_group_id();
        match item_builder.build(name_ref) {
            Ok((item, option)) => {
                let push_index = self.get_push_index();
                self.arena.insert((group_id, push_index), item);
                action(name_ref, item_kind, group_id, push_index, option)
            }
            Err(errors) => Some(errors.into_iter().map(|error| error.into()).collect()),
        }
    }

    /// item getter
    pub(crate) fn get(&self, group_id: GroupId, index: GraphItemId) -> Option<&I> {
        self.arena.get(&(group_id, index))
    }

    /// item getter by range
    pub(crate) fn range<R: RangeBounds<GraphItemId>>(
        &self,
        group_id: GroupId,
        range: R,
    ) -> Range<(GroupId, GraphItemId), I> {
        let start = range_with_group(group_id, range.start_bound());
        let end = range_with_group(group_id, range.end_bound());
        self.arena.range((start, end))
    }

    /// iter by filtering group_id
    pub(crate) fn filter_by_group<'a>(&'a self, group_id: GroupId) -> impl Iterator + 'a {
        self.iter()
            .filter_map(move |((item_group_id, item_id), item)| {
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
    pub(crate) fn count(&self) -> usize {
        self.arena.len()
    }

    /// item pool is empty
    pub(crate) fn is_empty(&self) -> bool {
        self.count() == 0
    }

    //
    // iter or slice
    //

    /// to iterator
    pub(crate) fn iter(&self) -> Iter<(GroupId, GraphItemId), I> {
        self.arena.iter()
    }
}

impl<I> Default for ItemArena<I> {
    /// initialize without log
    fn default() -> Self {
        ItemArena {
            pushed_index: Default::default(),
            arena: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::{Display, Formatter};

    use crate::grafo::core::item::{
        HasItemKind, ItemArena, ItemBase, ItemBuilderBase, ItemBuilderBaseBuilderMethod,
        ItemErrorBase,
    };
    use crate::grafo::core::layout::LayoutReference;
    use crate::grafo::core::refindex::NameReference;
    use crate::grafo::GrafoError;
    use crate::util::alias::{GraphItemId, GroupId, RefIndex};
    use crate::util::item_kind::test::check_list;
    use crate::util::item_kind::ItemKind;
    use crate::util::kind_key::KeyWithKind;
    use std::error::Error;

    const ITERATE_COUNT: usize = 10;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItemBuilder {
        group_id: GraphItemId,
        name: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItem {
        group_id: GraphItemId,
        item_id: GraphItemId,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItemOption {
        group_id: GraphItemId,
        name: Option<String>,
    }

    #[derive(Debug)]
    enum NodeBuildError {
        BuildFail,
    }

    impl Into<GrafoError> for NodeBuildError {
        fn into(self) -> GrafoError {
            unimplemented!()
        }
    }

    impl HasItemKind for NodeItem {
        fn kind() -> ItemKind {
            ItemKind::Node
        }
    }

    impl HasItemKind for NodeItemBuilder {
        fn kind() -> ItemKind {
            ItemKind::Node
        }
    }

    impl HasItemKind for NodeBuildError {
        fn kind() -> ItemKind {
            ItemKind::Node
        }
    }

    impl ItemBuilderBase for NodeItemBuilder {
        type Item = NodeItem;
        type ItemOption = NodeItemOption;
        type BuildFailError = NodeBuildError;

        fn set_group_id(&mut self, group_id: GraphItemId) -> &mut Self {
            self.group_id = group_id;
            self
        }

        fn get_group_id(&self) -> usize {
            self.group_id
        }
    }

    impl ItemBuilderBaseBuilderMethod for NodeItemBuilder {
        fn build(
            self,
            _layout: &NameReference,
        ) -> Result<(NodeItem, NodeItemOption), Vec<NodeBuildError>> {
            let NodeItemBuilder { group_id, name } = self;
            Ok((
                NodeItem {
                    group_id,
                    item_id: 0,
                },
                NodeItemOption { group_id, name },
            ))
        }
    }

    impl NodeItemBuilder {
        fn new() -> Self {
            NodeItemBuilder {
                group_id: 0,
                name: None,
            }
        }

        fn set_name(&mut self, name: String) -> &mut Self {
            self.name = Some(name);
            self
        }
    }

    impl ItemBase for NodeItem {
        fn get_group_id(&self) -> usize {
            self.group_id
        }

        fn get_item_id(&self) -> usize {
            self.item_id
        }
    }

    impl Display for NodeBuildError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            use NodeBuildError::*;
            match &self {
                BuildFail => write!(f, "fail build item"),
            }
        }
    }

    impl Error for NodeBuildError {}

    impl ItemErrorBase for NodeBuildError {}

    type NameRefIndex = RefIndex<KeyWithKind<ItemKind, String>, (GroupId, GraphItemId)>;

    #[test]
    fn is_empty() {
        assert!(ItemArena::<NodeItem>::new().is_empty());
    }

    #[test]
    fn with_action_count() {
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = NameReference::default();
        for i in 0..ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0).set_name(format!("{}", i));
            arena_mut.push(
                &mut names,
                builder,
                |_layout, kind, group_id, item_id, option| {
                    if let NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    } = option
                    {
                        names.insert(KeyWithKind::new(kind, name), (group_id, item_id));
                    }
                    None
                },
            );
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), ITERATE_COUNT);
        assert_eq!(names.len(), ITERATE_COUNT);
    }

    #[test]
    fn with_action_each_eq() {
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = NameReference::default();
        for i in 0..ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0).set_name(format!("{}", i));
            arena_mut.push(
                &mut names,
                builder,
                |_layout, kind, group_id, item_id, option| {
                    if let NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    } = option
                    {
                        names.insert(KeyWithKind::new(kind, name), (group_id, item_id));
                    }
                    None
                },
            );
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            let result: (usize, usize) = (0, index);
            assert_eq!(result, *item.0);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KeyWithKind::new(kind, format!("{}", index))),
                    if kind == ItemKind::Node {
                        Some(&result)
                    } else {
                        None
                    }
                );
            }
            index += 1;
        }
        assert_eq!(index, ITERATE_COUNT);
    }

    #[test]
    fn mixed_count() {
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = NameReference::default();
        for i in 0..2 * ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0);
            if i < ITERATE_COUNT {
                builder.set_name(format!("{}", i));
            }
            arena_mut.push(
                &mut names,
                builder,
                |_layout, kind, group_id, item_id, option| {
                    if let NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    } = option
                    {
                        names.insert(KeyWithKind::new(kind, name), (group_id, item_id));
                    }
                    None
                },
            );
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), 2 * ITERATE_COUNT);
        assert_eq!(names.len(), ITERATE_COUNT);
    }

    #[test]
    fn mixed_each_eq() {
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = NameReference::default();
        for i in 0..2 * ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0);
            if i < ITERATE_COUNT {
                builder.set_name(format!("{}", i));
            }
            arena_mut.push(
                &mut names,
                builder,
                |_layout, kind, group_id, item_id, option| {
                    if let NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    } = option
                    {
                        names.insert(KeyWithKind::new(kind, name), (group_id, item_id));
                    }
                    None
                },
            );
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            let result: (usize, usize) = (0, index);
            assert_eq!(result, *item.0);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KeyWithKind::new(kind, format!("{}", index))),
                    if index < ITERATE_COUNT && kind == ItemKind::Node {
                        Some(&result)
                    } else {
                        None
                    }
                );
            }
            index += 1;
        }
        assert_eq!(index, 2 * ITERATE_COUNT);
    }
}
