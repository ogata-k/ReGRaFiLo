//! item pool

use std::collections::BTreeMap;
use std::slice::{Iter, SliceIndex};

use regrafilo_util::log::{KindGroup4Logger, Logger};

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;

/// Item's base set
pub trait ItemBase: KindGroup4Logger {
    fn set_item_id(&mut self, index: ItemIndex);
    fn get_item_id(&self) -> ItemIndex;
}

/// Builder for ItemArena
#[derive(Debug, Clone)]
pub struct ItemArenaBuilder<T: ItemBase> {
    count: ItemIndex,
    names: BTreeMap<String, ItemIndex>,
    arena: Vec<T>,
}

impl<T: ItemBase> ItemArenaBuilder<T> {
    /// initializer
    pub fn new() -> Self {
        Logger::builder_start_log(T::kind_group());
        ItemArenaBuilder {
            count: 0,
            names: BTreeMap::default(),
            arena: Vec::default(),
        }
    }

    /// push item into arena
    pub fn push(&mut self, mut item: T) -> ItemIndex {
        let push_index = self.count;
        item.set_item_id(push_index);
        self.arena.push(item);
        self.count += 1;
        Logger::push_log(T::kind_group(), push_index);
        push_index
    }

    /// push item with name into arena
    pub fn push_with_name(&mut self, name: &str, mut item: T) -> ItemIndex {
        let push_index = self.count;
        item.set_item_id(push_index);
        self.names.insert(name.to_string(), push_index);
        self.arena.push(item);
        self.count += 1;
        Logger::with_name_push_log(T::kind_group(), name, push_index);
        push_index
    }

    /// count of items
    pub fn count(&self) -> usize {
        self.count
    }

    /// convert to ItemArena and name's map with optimize
    pub fn build(self) -> (ItemArena<T>, BTreeMap<String, ItemIndex>) {
        let ItemArenaBuilder {
            count,
            names,
            mut arena,
        } = self;
        (&mut arena).shrink_to_fit();
        Logger::builder_finish_log(T::kind_group());
        (ItemArena { count, arena }, names)
    }
}

/// item pool. support reference only.
#[derive(Debug, Clone)]
pub struct ItemArena<T: ItemBase> {
    count: ItemIndex,
    arena: Vec<T>,
}

impl<T: ItemBase> ItemArena<T> {
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

    /// get all items
    pub fn all(&self) -> &[T] {
        self.as_slice()
    }

    /// count of item
    pub fn count(&self) -> usize {
        self.count
    }

    /// item pool is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// to iterator
    pub fn iter(&self) -> Iter<'_, T> {
        self.arena.iter()
    }

    /// to slice
    pub fn as_slice(&self) -> &[T] {
        self.arena.as_slice()
    }
}

#[cfg(test)]
mod test {
    use regrafilo_util::log::{KindGroup4Logger, Logger};

    use crate::util::item_arena::{ItemArenaBuilder, ItemBase, ItemIndex};

    const COUNT: usize = 10;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct Item {
        id: ItemIndex,
    }

    impl Item {
        fn new() -> Self {
            Item { id: 0 }
        }
    }

    impl KindGroup4Logger for Item {
        fn kind_group() -> &'static str {
            "example"
        }
    }

    impl ItemBase for Item {
        fn set_item_id(&mut self, index: usize) {
            self.id = index;
        }

        fn get_item_id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn is_empty() {
        Logger::init(true);
        assert!(ItemArenaBuilder::<Item>::new().build().0.is_empty());
    }

    #[test]
    fn no_name_count() {
        Logger::init(true);
        let mut builder = ItemArenaBuilder::<Item>::new();
        for _ in 0..COUNT {
            builder.push(Item::new());
        }
        let (arena, names) = builder.build();
        assert_eq!(arena.count(), COUNT);
        assert_eq!(names.len(), 0);
    }

    #[test]
    fn no_name_each_eq() {
        Logger::init(true);
        let mut builder = ItemArenaBuilder::<Item>::new();
        for _ in 0..COUNT {
            builder.push(Item::new());
        }
        let (arena, _) = builder.build();
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
        let mut builder = ItemArenaBuilder::<Item>::new();
        for i in 0..COUNT {
            builder.push_with_name(&format!("{}", i), Item::new());
        }
        let (arena, names) = builder.build();
        assert_eq!(arena.count(), COUNT);
        assert_eq!(names.len(), COUNT);
    }

    #[test]
    fn with_name_each_eq() {
        Logger::init(true);
        let mut builder = ItemArenaBuilder::<Item>::new();
        for i in 0..COUNT {
            builder.push_with_name(&format!("{}", i), Item::new());
        }
        let (arena, names) = builder.build();
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            assert_eq!(names.get(&format!("{}", index)), Some(&index));
            index += 1;
        }
        assert_eq!(index, COUNT);
    }

    #[test]
    fn mixed_count() {
        Logger::init(true);
        let mut builder = ItemArenaBuilder::<Item>::new();
        for i in 0..COUNT {
            builder.push_with_name(&format!("{}", i), Item::new());
        }
        for _ in 0..COUNT {
            builder.push(Item::new());
        }
        let (arena, names) = builder.build();
        assert_eq!(arena.count(), 2 * COUNT);
        assert_eq!(names.len(), COUNT);
    }

    #[test]
    fn mixed_each_eq() {
        Logger::init(true);
        let mut builder = ItemArenaBuilder::<Item>::new();
        for i in 0..COUNT {
            builder.push_with_name(&format!("{}", i), Item::new());
        }
        for _ in 0..COUNT {
            builder.push(Item::new());
        }
        let (arena, names) = builder.build();
        let mut index: usize = 0;
        for item in (&arena).iter() {
            assert_eq!(item.get_item_id(), index);
            assert_eq!(
                names.get(&format!("{}", index)),
                if index < COUNT { Some(&index) } else { None }
            );
            index += 1;
        }
        assert_eq!(index, 2 * COUNT);
    }
}
