//! item pool

use std::collections::btree_map::{Iter, Range};
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::RangeBounds;
use std::sync::{Arc, Mutex};

use crate::event::{Event, ItemEventKind, Visitor};
use crate::grafo::core::item::{ItemBase, ItemBuilderBase};
use crate::util::alias::ItemIndex;
use crate::util::util_trait::KindBase;

/// item pool
#[derive(Debug, Clone)]
pub(crate) struct ItemArena<I> {
    pushed_index: Arc<Mutex<ItemIndex>>,
    arena: BTreeMap<ItemIndex, I>,
}

impl<K: KindBase + Into<ItemEventKind>, I: ItemBase<ItemKind = K>> ItemArena<I> {
    /// initialize
    pub(crate) fn new<V: Visitor>(visitor: &mut V) -> Self {
        visitor.visit(&Event::InitializeStore(I::kind().into()));
        ItemArena::default()
    }
}

impl<K: KindBase + Into<ItemEventKind>, I: ItemBase<ItemKind = K>> ItemArena<I> {
    //
    // helper
    //

    /// get the next index with increment as soon as possible
    fn get_push_index(&mut self) -> ItemIndex {
        match self.pushed_index.lock() {
            Ok(mut pushed_index) => {
                let next_index: ItemIndex = *pushed_index;
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
        V: Visitor,
        F,
        O,
        E: Error,
        B: ItemBuilderBase<ItemKind = K, Item = I, ItemOption = O, BuildFailErr = E>,
    >(
        &mut self,
        visitor: &mut V,
        item_builder: B,
        action: F,
    ) where
        F: FnOnce(&mut V, K, ItemIndex, Result<(ItemIndex, B::ItemOption), &B::BuildFailErr>),
    {
        let item_kind = B::kind();
        let group_id = item_builder.get_group_id();
        match item_builder.build() {
            Ok((item, option)) => {
                let push_index = self.get_push_index();
                self.arena.insert(push_index, item);
                action(visitor, item_kind, group_id, Ok((push_index, option)));
                visitor.visit(&Event::SucceededPushItem(
                    B::kind().into(),
                    group_id,
                    push_index,
                ));
            }
            Err(err) => {
                visitor.visit(&Event::FailPushItem(B::kind().into(), group_id, &err));
                action(visitor, item_kind, group_id, Err(&err));
            }
        }
    }
}

impl<K: KindBase, I: ItemBase<ItemKind = K>> ItemArena<I> {
    /// item getter
    pub(crate) fn get(&self, index: ItemIndex) -> Option<&I> {
        self.arena.get(&index)
    }

    /// item getter by range
    pub(crate) fn range<R: RangeBounds<ItemIndex>>(&self, range: R) -> Range<ItemIndex, I> {
        self.arena.range(range)
    }

    /// iter with specified indices
    pub(crate) fn select_indices<'a>(&'a self, indices: &'a [ItemIndex]) -> impl Iterator + 'a {
        self.iter().filter_map(move |(index, item)| {
            if indices.contains(index) {
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
    pub(crate) fn iter(&self) -> Iter<ItemIndex, I> {
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

    use crate::event::test::{check_list, Kind, Visitor, ITERATE_COUNT};
    use crate::grafo::core::item::{ItemArena, ItemBase, ItemBuilderBase};
    use crate::util::alias::{ItemIndex, RefIndexOfItem};
    use crate::util::kind_key::KeyWithKind;
    use std::error::Error;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItemBuilder {
        group_id: ItemIndex,
        name: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItem {
        group_id: ItemIndex,
        item_id: ItemIndex,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItemOption {
        group_id: ItemIndex,
        name: Option<String>,
    }

    #[derive(Debug)]
    enum NodeBuildError {
        #[allow(dead_code)]
        BuildFail,
    }

    impl ItemBuilderBase for NodeItemBuilder {
        type ItemKind = Kind;
        type Item = NodeItem;
        type ItemOption = NodeItemOption;
        type BuildFailErr = NodeBuildError;

        fn kind() -> Self::ItemKind {
            Kind::Node
        }

        fn set_group_id(&mut self, group_id: ItemIndex) -> &mut Self {
            self.group_id = group_id;
            self
        }

        fn get_group_id(&self) -> usize {
            self.group_id
        }

        fn build(self) -> Result<(Self::Item, NodeItemOption), NodeBuildError> {
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
        type ItemKind = Kind;

        fn kind() -> Kind {
            Kind::Node
        }

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

    #[test]
    fn is_empty() {
        let mut v = Visitor::new();
        assert!(ItemArena::<NodeItem>::new(&mut v).is_empty());
    }

    #[test]
    fn with_action_count() {
        let mut v = Visitor::new();
        let mut arena_mut = ItemArena::<NodeItem>::new(&mut v);
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0).set_name(format!("{}", i));
            arena_mut.push(&mut v, builder, |_visitor, kind, _group_id, result| {
                if let Ok((
                    item_id,
                    NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    },
                )) = result
                {
                    names.insert(KeyWithKind::new(kind, name), item_id);
                }
            });
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), ITERATE_COUNT);
        assert_eq!(names.len(), ITERATE_COUNT);
    }

    #[test]
    fn with_action_each_eq() {
        let mut v = Visitor::new();
        let mut arena_mut = ItemArena::<NodeItem>::new(&mut v);
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0).set_name(format!("{}", i));
            arena_mut.push(&mut v, builder, |_visitor, kind, _group_id, result| {
                if let Ok((
                    item_id,
                    NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    },
                )) = result
                {
                    names.insert(KeyWithKind::new(kind, name), item_id);
                }
            });
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(index, *item.0);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KeyWithKind::new(kind, format!("{}", index))),
                    if kind == Kind::Node {
                        Some(&index)
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
        let mut v = Visitor::new();
        let mut arena_mut = ItemArena::<NodeItem>::new(&mut v);
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..2 * ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0);
            if i < ITERATE_COUNT {
                builder.set_name(format!("{}", i));
            }
            arena_mut.push(&mut v, builder, |_visitor, kind, _group_id, result| {
                if let Ok((
                    item_id,
                    NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    },
                )) = result
                {
                    names.insert(KeyWithKind::new(kind, name), item_id);
                }
            });
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), 2 * ITERATE_COUNT);
        assert_eq!(names.len(), ITERATE_COUNT);
    }

    #[test]
    fn mixed_each_eq() {
        let mut v = Visitor::new();
        let mut arena_mut = ItemArena::<NodeItem>::new(&mut v);
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..2 * ITERATE_COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0);
            if i < ITERATE_COUNT {
                builder.set_name(format!("{}", i));
            }
            arena_mut.push(&mut v, builder, |_visitor, kind, _group_id, result| {
                if let Ok((
                    item_id,
                    NodeItemOption {
                        group_id: _,
                        name: Some(name),
                    },
                )) = result
                {
                    names.insert(KeyWithKind::new(kind, name), item_id);
                }
            });
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(index, *item.0);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KeyWithKind::new(kind, format!("{}", index))),
                    if index < ITERATE_COUNT && kind == Kind::Node {
                        Some(&index)
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
