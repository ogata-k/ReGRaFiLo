//! item pool

use std::collections::BTreeMap;
use std::slice::{Iter, SliceIndex};

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;

/// Builder for ItemArena
#[derive(Debug, Clone)]
pub struct ItemArenaBuilder<T> {
    count: ItemIndex,
    names: BTreeMap<String, ItemIndex>,
    arena: Vec<T>,
}

impl<T> ItemArenaBuilder<T> {
    /// initializer
    pub fn new() -> Self {
        ItemArenaBuilder::<T>::default()
    }

    /// push item into arena
    pub fn push(&mut self, item: T) -> ItemIndex {
        self.arena.push(item);
        let push_index = self.count;
        self.count += 1;
        push_index
    }

    /// push item with name into arena
    pub fn push_with_name<S: ToString>(&mut self, name: &S, item: T) {
        self.names.insert(name.to_string(), self.count);
        self.arena.push(item);
        self.count += 1;
    }

    /// get index when push at next time
    pub fn get_next_push_index(&self) -> ItemIndex {
        self.count
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
        (ItemArena { count, arena }, names)
    }
}

//
// impl traits
//

impl<T> Default for ItemArenaBuilder<T> {
    fn default() -> Self {
        ItemArenaBuilder {
            count: 0,
            names: BTreeMap::default(),
            arena: Vec::default(),
        }
    }
}

impl<T> From<Vec<T>> for ItemArenaBuilder<T> {
    fn from(v: Vec<T>) -> Self {
        let l = v.len();
        ItemArenaBuilder {
            count: l,
            names: BTreeMap::default(),
            arena: v,
        }
    }
}

/// item pool. support reference only.
#[derive(Debug, Clone)]
pub struct ItemArena<T> {
    count: ItemIndex,
    arena: Vec<T>,
}

impl<T> ItemArena<T> {
    /// initializer
    pub fn new() -> Self {
        ItemArena::<T>::default()
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

//
// impl traits
//

impl<T> Default for ItemArena<T> {
    fn default() -> Self {
        ItemArena {
            count: 0,
            arena: Vec::with_capacity(0),
        }
    }
}

impl<'a, T> Into<&'a [T]> for &'a ItemArena<T> {
    fn into(self) -> &'a [T] {
        &self.arena
    }
}
