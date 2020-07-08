//! item pool

use std::slice::{Iter, SliceIndex};

use crate::util::kind_key::KindKey;
use crate::util::RefIndex;
use regrafilo_util::log::{KindBase, Logger};

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;

/// RefIndex for ItemIndex
pub type RefIndexOfItem<K, T> = RefIndex<KindKey<K, T>, ItemIndex>;

/// Item's base set
pub trait ItemBase {
    type ItemKind: KindBase;
    fn set_item_id(&mut self, index: ItemIndex);
    fn get_kind(&self) -> Self::ItemKind;
    fn get_group_id(&self) -> ItemIndex;
    fn get_item_id(&self) -> ItemIndex;
}

/// item pool
#[derive(Debug, Clone)]
pub struct ItemArena<K: KindBase, T: ItemBase<ItemKind = K>> {
    arena: Vec<T>,
}

impl<K: KindBase, T: ItemBase<ItemKind = K>> ItemArena<K, T> {
    /// initializer
    pub fn new() -> Self {
        Logger::initializer_log(K::kind_group());
        ItemArena::default()
    }

    //
    // helper
    //

    /// get next index with increment as soon as possible
    fn get_push_index(&self) -> ItemIndex {
        self.arena.len()
    }

    //
    // setter
    //

    /// push item into arena
    pub fn push(&mut self, mut item: T) {
        let push_index = self.get_push_index();
        let item_kind = item.get_kind();
        item.set_item_id(push_index);
        self.arena.push(item);
        Logger::push_log(item_kind.get_kind_string(), push_index);
    }

    /// push item into arena with action for conclusion
    pub fn push_with_action<F>(&mut self, mut item: T, conclusion: F)
    where
        F: Fn(K, ItemIndex, ItemIndex),
    {
        let push_index = self.get_push_index();
        item.set_item_id(push_index);
        let group_id = item.get_group_id();
        let item_kind = item.get_kind();
        self.arena.push(item);
        conclusion(item_kind, group_id, push_index);
        Logger::push_log(item_kind.get_kind_string(), push_index);
    }

    /// push item with name into arena
    pub fn push_with_name(
        &mut self,
        names: &mut RefIndexOfItem<K, String>,
        name: &str,
        mut item: T,
    ) {
        let push_index = self.get_push_index();
        let item_kind = item.get_kind();
        item.set_item_id(push_index);
        names.insert(KindKey::new(item_kind, name.to_string()), push_index);
        self.arena.push(item);
        Logger::with_name_push_log(item_kind.get_kind_string(), name, push_index);
    }

    /// push item with name into arena with action for conclusion
    pub fn push_with_name_and_action<F>(
        &mut self,
        names: &mut RefIndexOfItem<K, String>,
        name: &str,
        mut item: T,
        conclusion: F,
    ) where
        F: Fn(K, ItemIndex, ItemIndex),
    {
        let push_index = self.get_push_index();
        item.set_item_id(push_index);
        let item_kind = item.get_kind();
        let group_id = item.get_group_id();
        names.insert(KindKey::new(item_kind, name.to_string()), push_index);
        self.arena.push(item);
        conclusion(item_kind, group_id, push_index);
        Logger::with_name_push_log(item_kind.get_kind_string(), name, push_index);
    }

    /// item getter
    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>,
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
    pub fn iter(&self) -> Iter<'_, T> {
        self.arena.iter()
    }

    /// get all items
    pub fn all(&self) -> &[T] {
        self.as_slice()
    }

    /// to slice
    pub fn as_slice(&self) -> &[T] {
        self.arena.as_slice()
    }
}

impl<K: KindBase, T: ItemBase<ItemKind = K>> Default for ItemArena<K, T> {
    fn default() -> Self {
        ItemArena {
            arena: Vec::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use regrafilo_util::log::{KindBase, KindGroup4Logger, KindKey4Logger, Logger};

    use crate::util::item_arena::{ItemArena, ItemBase, ItemIndex, RefIndexOfItem};
    use crate::util::kind_key::KindKey;

    const COUNT: usize = 10;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    enum Kind {
        Group,
        Node,
        Edge,
    }

    impl KindGroup4Logger for Kind {
        fn kind_group() -> &'static str {
            "GraphItem"
        }
    }

    impl KindKey4Logger for Kind {
        fn get_kind_string(&self) -> &'static str {
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

    const CHECKER: Kind = Kind::Group;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Item {
        id: ItemIndex,
        kind: Kind,
    }

    impl Item {
        fn new() -> Self {
            Item {
                id: 0,
                kind: CHECKER,
            }
        }
    }

    impl ItemBase for Item {
        type ItemKind = Kind;

        fn set_item_id(&mut self, index: usize) {
            self.id = index;
        }

        fn get_kind(&self) -> Kind {
            self.kind
        }

        fn get_group_id(&self) -> usize {
            0
        }

        fn get_item_id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn is_empty() {
        Logger::init(true);
        assert!(ItemArena::<Kind, Item>::new().is_empty());
    }

    #[test]
    fn no_name_count() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<Kind, Item>::new();
        for _ in 0..COUNT {
            arena_mut.push(Item::new());
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), COUNT);
    }

    #[test]
    fn no_name_each_eq() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<Kind, Item>::new();
        for _ in 0..COUNT {
            arena_mut.push(Item::new());
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
    fn with_name_count() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<Kind, Item>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..COUNT {
            arena_mut.push_with_name(&mut names, &format!("{}", i), Item::new());
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), COUNT);
        assert_eq!(names.len(), COUNT);
    }

    #[test]
    fn with_name_each_eq() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<Kind, Item>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..COUNT {
            arena_mut.push_with_name(&mut names, &format!("{}", i), Item::new());
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KindKey::new(kind, format!("{}", index))),
                    if kind == CHECKER { Some(&index) } else { None }
                );
            }
            index += 1;
        }
        assert_eq!(index, COUNT);
    }

    #[test]
    fn mixed_count() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<Kind, Item>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..COUNT {
            arena_mut.push_with_name(&mut names, &format!("{}", i), Item::new());
        }
        for _ in 0..COUNT {
            arena_mut.push(Item::new());
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), 2 * COUNT);
        assert_eq!(names.len(), COUNT);
    }

    #[test]
    fn mixed_each_eq() {
        Logger::init(true);
        let mut arena_mut = ItemArena::<Kind, Item>::new();
        let mut names = RefIndexOfItem::<Kind, String>::new();
        for i in 0..COUNT {
            arena_mut.push_with_name(&mut names, &format!("{}", i), Item::new());
        }
        for _ in 0..COUNT {
            arena_mut.push(Item::new());
        }
        let arena = arena_mut;
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            for kind in check_list() {
                assert_eq!(
                    names.get(&KindKey::new(kind, format!("{}", index))),
                    if index < COUNT && kind == CHECKER {
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
