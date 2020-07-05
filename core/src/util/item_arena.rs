//! item pool

use std::collections::BTreeMap;
use std::slice::{Iter, SliceIndex};

use regrafilo_util::log::Logger;

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;

/// Item's base set
pub trait ItemBase {
    fn kind_string() -> &'static str;
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
        Logger::builder_start_log(T::kind_string());
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
        Logger::push_log(T::kind_string(), push_index);
        push_index
    }

    /// push item with name into arena
    pub fn push_with_name(&mut self, name: &str, mut item: T) -> ItemIndex {
        let push_index = self.count;
        item.set_item_id(push_index);
        self.names.insert(name.to_string(), push_index);
        self.arena.push(item);
        self.count += 1;
        Logger::with_name_push_log(T::kind_string(), name, push_index);
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
        Logger::builder_finish_log(T::kind_string());
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

    /// to iterator
    pub fn iter(&self) -> Iter<'_, T> {
        self.arena.iter()
    }

    /// to slice
    pub fn as_slice(&self) -> &[T] {
        self.arena.as_slice()
    }
}

// TODO add test
