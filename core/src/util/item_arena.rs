//! item pool

use std::slice::{Iter, SliceIndex};

use regrafilo_util::log::{KindBase, Logger};

use crate::util::item_base::{ItemBase, ItemBuilderBase, ItemIndex};
use std::fmt::Display;

/// item pool
#[derive(Debug, Clone)]
pub struct ItemArena<I> {
    arena: Vec<I>,
}

impl<K: KindBase, I: ItemBase<ItemKind = K>> ItemArena<I> {
    /// initialize
    pub fn new() -> Self {
        Logger::initializer_log(K::group_kind_string(), None);
        ItemArena::default()
    }
}

impl<K: KindBase, I: ItemBase<ItemKind = K>> ItemArena<I> {
    //
    // helper
    //

    /// get the next index with increment as soon as possible
    fn get_push_index(&self) -> ItemIndex {
        self.arena.len()
    }

    //
    // setter
    //

    /// push the item into arena with action for conclusion<br/>
    /// F: fn(item_kind, group_id, item_id, extension)
    pub fn push_with_action<
        F,
        IO,
        BFE: Display,
        IB: ItemBuilderBase<ItemKind = K, Item = I, ItemOption = IO, BuildFailErr = BFE>,
    >(
        &mut self,
        item_builder: IB,
        action: F,
    ) where
        F: FnOnce(K, ItemIndex, ItemIndex, Result<IB::ItemOption, IB::BuildFailErr>),
    {
        let push_index = self.get_push_index();
        let group_id = item_builder.get_group_id();
        let item_kind = IB::kind();
        let result = item_builder.build(push_index);
        match result {
            Ok((item, option)) => {
                Logger::push_log(
                    K::group_kind_string(),
                    item_kind.key_kind_string(),
                    push_index,
                );
                self.arena.push(item);
                action(item_kind, group_id, push_index, Ok(option));
            }
            Err(err) => {
                Logger::push_err_log(
                    K::group_kind_string(),
                    item_kind.key_kind_string(),
                    push_index,
                    &err,
                );
                action(item_kind, group_id, push_index, Err(err));
            }
        }
    }

    /// push the item into arena without action for conclusion
    pub fn push<
        IO,
        BFE: Display,
        IB: ItemBuilderBase<ItemKind = K, Item = I, ItemOption = IO, BuildFailErr = BFE>,
    >(
        &mut self,
        item_builder: IB,
    ) {
        let push_index = self.get_push_index();
        let item_kind = IB::kind();
        let result = item_builder.build(push_index);
        match result {
            Ok((item, _)) => {
                self.arena.push(item);
                Logger::push_log(
                    K::group_kind_string(),
                    item_kind.key_kind_string(),
                    push_index,
                );
            }
            Err(err) => {
                Logger::push_err_log(
                    K::group_kind_string(),
                    item_kind.key_kind_string(),
                    push_index,
                    &err,
                );
            }
        }
    }
}

impl<K: KindBase, I: ItemBase<ItemKind = K>> ItemArena<I> {
    /// item getter
    pub fn get<SI>(&self, index: SI) -> Option<&<SI as SliceIndex<[I]>>::Output>
    where
        SI: SliceIndex<[I]>,
    {
        self.arena.get(index)
    }

    /// iter with specified indices
    pub fn select_indices<'a>(&'a self, indices: &'a [ItemIndex]) -> impl Iterator + 'a {
        self.iter().enumerate().filter_map(move |(index, item)| {
            if indices.contains(&index) {
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
    pub fn iter(&self) -> Iter<'_, I> {
        self.arena.iter()
    }

    /// get all items
    pub fn all(&self) -> &[I] {
        self.as_slice()
    }

    /// to slice
    pub fn as_slice(&self) -> &[I] {
        self.arena.as_slice()
    }
}

impl<I> Default for ItemArena<I> {
    /// initialize without log
    fn default() -> Self {
        ItemArena {
            arena: Vec::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use regrafilo_util::log::{GroupKind4Logger, KeyKind4Logger, KindBase, Logger};

    use crate::util::item_arena::ItemArena;
    use crate::util::item_base::{ItemBase, ItemBuilderBase, ItemIndex, RefIndexOfItem};
    use crate::util::kind_key::KindKey;
    use std::fmt::{Display, Formatter};

    const COUNT: usize = 10;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    enum Kind {
        Group,
        Node,
        Edge,
    }

    impl GroupKind4Logger for Kind {
        fn group_kind_string() -> &'static str {
            "Graph"
        }
    }

    impl KeyKind4Logger for Kind {
        fn key_kind_string(&self) -> &'static str {
            use Kind::*;
            match self {
                Group => "Group",
                Node => "Node",
                Edge => "Edge",
            }
        }
    }

    impl KindBase for Kind {}

    fn check_list() -> Vec<Kind> {
        use Kind::*;
        vec![Group, Node, Edge]
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItemBuilder {
        group_id: ItemIndex,
        name: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItemOption {
        group_id: ItemIndex,
        item_id: ItemIndex,
        name: Option<String>,
    }

    #[derive(Debug)]
    enum NodeBuildError {
        #[allow(dead_code)]
        BuildFail,
    }

    impl Display for NodeBuildError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            use NodeBuildError::*;
            match &self {
                BuildFail => write!(f, "fail build item"),
            }
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

        fn build(self, item_id: usize) -> Result<(Self::Item, NodeItemOption), NodeBuildError> {
            let NodeItemBuilder { group_id, name } = self;
            Ok((
                NodeItem { group_id, item_id },
                NodeItemOption {
                    group_id,
                    item_id,
                    name,
                },
            ))
        }
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct NodeItem {
        group_id: ItemIndex,
        item_id: ItemIndex,
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

    #[test]
    fn is_empty() {
        Logger::init(true);
        assert!(ItemArena::<NodeItem>::new().is_empty());
    }

    #[test]
    fn count() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<NodeItem>::new();
        for _ in 0..COUNT {
            arena_mut.push(NodeItemBuilder::new());
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), COUNT);
    }

    #[test]
    fn each_eq() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<NodeItem>::new();
        for _ in 0..COUNT {
            arena_mut.push(NodeItemBuilder::new());
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            index += 1;
        }
        assert_eq!(index, COUNT);
    }

    #[test]
    fn with_action_count() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0).set_name(format!("{}", i));
            arena_mut.push_with_action(
                builder,
                |kind, _group_index, item_index, result: Result<NodeItemOption, NodeBuildError>| {
                    if let Ok(NodeItemOption {
                        group_id: _,
                        item_id: _,
                        name: Some(name),
                    }) = result
                    {
                        names.insert(KindKey::new(kind, name), item_index);
                    }
                },
            );
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), COUNT);
        assert_eq!(names.len(), COUNT);
    }

    #[test]
    fn with_action_each_eq() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0).set_name(format!("{}", i));
            arena_mut.push_with_action(builder, |kind, _group_index, item_index, result| {
                if let Ok(NodeItemOption {
                    group_id: _,
                    item_id: _,
                    name: Some(name),
                }) = result
                {
                    names.insert(KindKey::new(kind, name), item_index);
                }
            });
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KindKey::new(kind, format!("{}", index))),
                    if kind == Kind::Node {
                        Some(&index)
                    } else {
                        None
                    }
                );
            }
            index += 1;
        }
        assert_eq!(index, COUNT);
    }

    #[test]
    fn mixed_count() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..2 * COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0);
            if i < COUNT {
                builder.set_name(format!("{}", i));
            }
            arena_mut.push_with_action(builder, |kind, _group_index, item_index, result| {
                if let Ok(NodeItemOption {
                    group_id: _,
                    item_id: _,
                    name: Some(name),
                }) = result
                {
                    names.insert(KindKey::new(kind, name), item_index);
                }
            });
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), 2 * COUNT);
        assert_eq!(names.len(), COUNT);
    }

    #[test]
    fn mixed_each_eq() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<NodeItem>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..2 * COUNT {
            let mut builder = NodeItemBuilder::new();
            builder.set_group_id(0);
            if i < COUNT {
                builder.set_name(format!("{}", i));
            }
            arena_mut.push_with_action(builder, |kind, _group_index, item_index, result| {
                if let Ok(NodeItemOption {
                    group_id: _,
                    item_id: _,
                    name: Some(name),
                }) = result
                {
                    names.insert(KindKey::new(kind, name), item_index);
                }
            });
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KindKey::new(kind, format!("{}", index))),
                    if index < COUNT && kind == Kind::Node {
                        Some(&index)
                    } else {
                        None
                    }
                );
            }
            index += 1;
        }
        assert_eq!(index, 2 * COUNT);
    }
}
