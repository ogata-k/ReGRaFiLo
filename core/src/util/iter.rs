//! utility for iterator

use std::collections::{BTreeMap, HashMap};
use std::iter::FusedIterator;

use std::hash::Hash;

/// iterator with peekable from first and last.
#[derive(Clone, Debug)]
pub struct DoubleEndedPeekable<I: DoubleEndedIterator + ExactSizeIterator> {
    iter: I,
    peeked: Option<Option<I::Item>>,
    peeked_last: Option<Option<I::Item>>,
}

impl<I: DoubleEndedIterator + ExactSizeIterator> Iterator for DoubleEndedPeekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => {
                if self.iter.len() != 0 {
                    self.iter.next()
                } else {
                    self.peeked_last.take().flatten()
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let peek_len = match (&self.peeked, &self.peeked_last) {
            (Some(None), Some(None)) => return (0, Some(0)),
            (Some(Some(_)), Some(None))
            | (Some(None), Some(Some(_)))
            | (None, Some(Some(_)))
            | (Some(Some(_)), None) => 1,
            (Some(Some(_)), Some(Some(_))) => 2,
            (None, None) | (None, Some(None)) | (Some(None), None) => 0,
        };
        let (lo, hi) = self.iter.size_hint();
        let lo = lo.saturating_add(peek_len);
        let hi = match hi {
            Some(x) => x.checked_add(peek_len),
            None => None,
        };
        (lo, hi)
    }

    #[inline]
    fn count(mut self) -> usize {
        let head = match self.peeked.take() {
            Some(None) => 0,
            Some(Some(_)) => 1 + self.iter.count(),
            None => self.iter.count(),
        };
        let last = match self.peeked_last.take() {
            Some(None) | None => 0,
            Some(Some(_)) => 1,
        };
        head + last
    }

    #[inline]
    fn last(mut self) -> Option<I::Item> {
        match self.peeked_last.take() {
            Some(None) => return None,
            Some(v) => v,
            None => None,
        }
        .or(self.iter.last().or(match self.peeked.take() {
            Some(None) => return None,
            Some(v) => v,
            None => None,
        }))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        match self.peeked.take() {
            Some(None) => None,
            Some(v @ Some(_)) if n == 0 => v,
            Some(Some(_)) => match &self.peeked_last {
                Some(None) | None => self.iter.nth(n - 1),
                Some(Some(_)) => {
                    if self.iter.len() == n - 1 {
                        self.peeked_last.take().flatten()
                    } else {
                        self.iter.nth(n - 1)
                    }
                }
            },
            None => self.iter.nth(n),
        }
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator> ExactSizeIterator for DoubleEndedPeekable<I> {}

impl<I: DoubleEndedIterator + ExactSizeIterator> DoubleEndedIterator for DoubleEndedPeekable<I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.peeked_last.take() {
            None | Some(None) => match self.peeked.as_mut() {
                Some(v @ Some(_)) => self.iter.next_back().or_else(|| v.take()),
                Some(None) => None,
                None => self.iter.next_back(),
            },
            Some(Some(v)) => Some(v),
        }
    }
}

impl<I: FusedIterator + DoubleEndedIterator + ExactSizeIterator> FusedIterator
    for DoubleEndedPeekable<I>
{
}

impl<I: DoubleEndedIterator + ExactSizeIterator> DoubleEndedPeekable<I> {
    /// initializer for this iterator from other iterator
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(iter: I) -> DoubleEndedPeekable<I> {
        DoubleEndedPeekable {
            iter,
            peeked: None,
            peeked_last: None,
        }
    }

    /// returns a reference to the next() value without advancing the iterator.
    #[inline]
    pub fn peek(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }

    /// returns a reference to the next_back() value without advancing the iterator.
    #[inline]
    pub fn peek_back(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.peeked_last
            .get_or_insert_with(|| iter.next_back())
            .as_ref()
    }
}

/// iterator for all items ordering by item's index in all groups
#[derive(Debug, Clone)]
pub struct IterLimitedByAllGroup<'a, K: Copy + Ord + 'a, V: 'a> {
    iters: Vec<DoubleEndedPeekable<std::collections::btree_map::Iter<'a, K, V>>>,
}

impl<'a, K: Copy + Ord + 'a, V: 'a> Iterator for IterLimitedByAllGroup<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut min_item_index: Option<K> = None;
        // It is assumed that there are few registered group's indices.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_index, _)) = iterable.peek() {
                match min_item_index {
                    None => {
                        target_index = Some(index);
                        min_item_index = Some(**item_index);
                    }
                    Some(_min_item_index) if _min_item_index >= **item_index => {
                        target_index = Some(index);
                        min_item_index = Some(**item_index);
                    }
                    _ => {}
                }
            }
        }
        match target_index {
            None => None,
            Some(_target_index) => self
                .iters
                .get_mut(_target_index)
                .map(|iter| iter.next())
                .flatten(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut hint: (usize, Option<usize>) = (0, None);
        for iter in self.iters.iter() {
            let (iter_min, iter_max) = iter.size_hint();
            let next_max = match (hint.1, iter_max) {
                (None, None) => None,
                (Some(i), None) | (None, Some(i)) => Some(i),
                (Some(i1), Some(i2)) => Some(i1 + i2),
            };
            hint = (hint.0 + iter_min, next_max);
        }
        hint
    }
}

impl<'a, K: Copy + Ord + 'a, V: 'a> ExactSizeIterator for IterLimitedByAllGroup<'a, K, V> {}

impl<'a, K: Copy + Ord + 'a, V: 'a> DoubleEndedIterator for IterLimitedByAllGroup<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut max_item_index: Option<K> = None;
        // It is assumed that there are few registered group's indices.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_index, _)) = iterable.peek_back() {
                match max_item_index {
                    None => {
                        target_index = Some(index);
                        max_item_index = Some(**item_index);
                    }
                    Some(_min_item_index) if _min_item_index <= **item_index => {
                        target_index = Some(index);
                        max_item_index = Some(**item_index);
                    }
                    _ => {}
                }
            }
        }
        match target_index {
            None => None,
            Some(_target_index) => self
                .iters
                .get_mut(_target_index)
                .map(|iter| iter.next_back())
                .flatten(),
        }
    }
}

impl<'a, K: Copy + Ord + 'a, V: 'a> FusedIterator for IterLimitedByAllGroup<'a, K, V> {}

impl<'a, K: Copy + Ord + 'a, V: 'a> IterLimitedByAllGroup<'a, K, V> {
    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_btree_map<Group: Eq + Copy>(map: &'a BTreeMap<Group, BTreeMap<K, V>>) -> Self
    where
        Group: Ord,
    {
        let mut iters = Vec::new();
        for (_, map) in map.iter() {
            iters.push(DoubleEndedPeekable::from_iter(map.iter()));
        }
        IterLimitedByAllGroup { iters }
    }

    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_hash_map<Group: Eq + Copy>(map: &'a HashMap<Group, BTreeMap<K, V>>) -> Self
    where
        Group: Hash,
    {
        let mut iters = Vec::new();
        for (_, map) in map.iter() {
            iters.push(DoubleEndedPeekable::from_iter(map.iter()));
        }
        IterLimitedByAllGroup { iters }
    }
}

/// iterator for all items ordering by item's index in specified groups
#[derive(Debug, Clone)]
pub struct IterLimitedByGroupList<'a, Group, K: Copy + Ord + 'a, V: 'a> {
    groups: Vec<Group>,
    iters: Vec<DoubleEndedPeekable<std::collections::btree_map::Iter<'a, K, V>>>,
}

impl<'a, Group, K: Copy + Ord + 'a, V: 'a> Iterator for IterLimitedByGroupList<'a, Group, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut min_item_index: Option<K> = None;
        // It is assumed that there are few registered group's index.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_index, _)) = iterable.peek() {
                match min_item_index {
                    None => {
                        target_index = Some(index);
                        min_item_index = Some(**item_index);
                    }
                    Some(_min_item_index) if _min_item_index >= **item_index => {
                        target_index = Some(index);
                        min_item_index = Some(**item_index);
                    }
                    _ => {}
                }
            }
        }
        match target_index {
            None => None,
            Some(_target_index) => self
                .iters
                .get_mut(_target_index)
                .map(|iter| iter.next())
                .flatten(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut hint: (usize, Option<usize>) = (0, None);
        for iter in self.iters.iter() {
            let (iter_min, iter_max) = iter.size_hint();
            let next_max = match (hint.1, iter_max) {
                (None, None) => None,
                (Some(i), None) | (None, Some(i)) => Some(i),
                (Some(i1), Some(i2)) => Some(i1 + i2),
            };
            hint = (hint.0 + iter_min, next_max);
        }
        hint
    }
}

impl<'a, Group, K: Copy + Ord + 'a, V: 'a> ExactSizeIterator
    for IterLimitedByGroupList<'a, Group, K, V>
{
}

impl<'a, Group, K: Copy + Ord + 'a, V: 'a> DoubleEndedIterator
    for IterLimitedByGroupList<'a, Group, K, V>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut max_item_index: Option<K> = None;
        // It is assumed that there are few registered group_indices.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_index, _)) = iterable.peek_back() {
                match max_item_index {
                    None => {
                        target_index = Some(index);
                        max_item_index = Some(**item_index);
                    }
                    Some(_min_item_index) if _min_item_index <= **item_index => {
                        target_index = Some(index);
                        max_item_index = Some(**item_index);
                    }
                    _ => {}
                }
            }
        }
        match target_index {
            None => None,
            Some(_target_index) => self
                .iters
                .get_mut(_target_index)
                .map(|iter| iter.next_back())
                .flatten(),
        }
    }
}

impl<'a, Group, K: Copy + Ord + 'a, V: 'a> FusedIterator
    for IterLimitedByGroupList<'a, Group, K, V>
{
}

impl<'a, Group: Eq + Copy, K: Copy + Ord + 'a, V: 'a> IterLimitedByGroupList<'a, Group, K, V> {
    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_btree_map(groups: &[Group], map: &'a BTreeMap<Group, BTreeMap<K, V>>) -> Self {
        let mut list = Vec::new();
        let mut iters = Vec::new();
        for (group, map) in map.iter() {
            if groups.contains(group) {
                list.push(*group);
                iters.push(DoubleEndedPeekable::from_iter(map.iter()));
            }
        }
        IterLimitedByGroupList {
            groups: list,
            iters,
        }
    }

    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_hash_map(groups: &[Group], map: &'a HashMap<Group, BTreeMap<K, V>>) -> Self {
        let mut list = Vec::new();
        let mut iters = Vec::new();
        for (group, map) in map.iter() {
            if groups.contains(group) {
                list.push(*group);
                iters.push(DoubleEndedPeekable::from_iter(map.iter()));
            }
        }
        IterLimitedByGroupList {
            groups: list,
            iters,
        }
    }

    /// get specified group list for limiter of this iterator.
    pub fn using_groups(&self) -> &[Group] {
        self.groups.as_slice()
    }
}

/// iterator for all items ordering by item's index grouped by group's index
#[derive(Debug, Clone)]
pub struct IterLimitedByOneGroup<'a, Group, K: Ord + 'a, V: 'a> {
    group: Group,
    inner_iter: Option<std::collections::btree_map::Iter<'a, K, V>>,
}

impl<'a, Group, K: Ord + 'a, V: 'a> Iterator for IterLimitedByOneGroup<'a, Group, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iter.as_mut() {
            None => None,
            Some(iter) => iter.next().map(|item_res| item_res),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner_iter {
            None => (0, Some(0)),
            Some(iter) => iter.size_hint(),
        }
    }
}

impl<'a, Group, K: Ord + 'a, V: 'a> ExactSizeIterator for IterLimitedByOneGroup<'a, Group, K, V> {}

impl<'a, Group, K: Ord + 'a, V: 'a> DoubleEndedIterator for IterLimitedByOneGroup<'a, Group, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner_iter.as_mut() {
            None => None,
            Some(inner) => inner.next_back(),
        }
    }

    #[allow(unused_mut)]
    #[inline]
    fn nth_back(&mut self, mut n: usize) -> Option<Self::Item> {
        match self.inner_iter.as_mut() {
            None => None,
            Some(inner) => inner.nth(n),
        }
    }

    #[allow(unused_mut)]
    #[inline]
    fn rfold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self.inner_iter.as_mut() {
            None => init,
            Some(inner) => inner.rfold(init, f),
        }
    }

    #[inline]
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        match self.inner_iter.as_mut() {
            None => None,
            Some(inner) => inner.rfind(predicate),
        }
    }
}

impl<'a, Group, K: Ord + 'a, V: 'a> FusedIterator for IterLimitedByOneGroup<'a, Group, K, V> {}

impl<'a, Group: Eq + Copy, K: Ord + 'a, V: 'a> IterLimitedByOneGroup<'a, Group, K, V> {
    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_btree_map(group: &Group, map: &'a BTreeMap<Group, BTreeMap<K, V>>) -> Self
    where
        Group: Ord,
    {
        IterLimitedByOneGroup {
            group: *group,
            inner_iter: map.get(group).map(|map| map.iter()),
        }
    }

    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_hash_map(group: &Group, map: &'a HashMap<Group, BTreeMap<K, V>>) -> Self
    where
        Group: Hash,
    {
        IterLimitedByOneGroup {
            group: *group,
            inner_iter: map.get(group).map(|map| map.iter()),
        }
    }

    /// group for grouping
    pub fn get_group(&self) -> Group {
        self.group
    }

    /// check iter has item. This is **NOT** checker for result of next() is None.
    pub fn has_iter(&self) -> bool {
        self.inner_iter.is_some()
    }
}

#[cfg(test)]
mod test {
    mod double_ended_peekable {
        use crate::util::iter::DoubleEndedPeekable;

        const ITEM_COUNT: usize = 5;

        fn tester_vec() -> Vec<usize> {
            (0..ITEM_COUNT).map(|i| 2 * i).collect()
        }

        #[test]
        fn incremental() {
            let vec = tester_vec();
            let mut iter = DoubleEndedPeekable::from_iter(vec.iter());
            for i in 0..ITEM_COUNT {
                assert_eq!(iter.peek(), Some(2_usize * i).as_ref().as_ref());
                assert_eq!(iter.next(), Some(2_usize * i).as_ref());
            }
            assert_eq!(iter.peek(), None);
            assert_eq!(iter.peek_back(), None);
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);
        }

        #[test]
        fn first() {
            let vec = tester_vec();
            let mut iter = DoubleEndedPeekable::from_iter(vec.iter());

            assert_eq!(iter.nth(0), tester_vec().first());
        }

        #[test]
        fn third() {
            let vec = tester_vec();
            let mut iter = DoubleEndedPeekable::from_iter(vec.iter());

            assert_eq!(iter.nth(2), tester_vec().iter().nth(2));
        }

        #[test]
        fn decremental() {
            let vec = tester_vec();
            let mut iter = DoubleEndedPeekable::from_iter(vec.iter());
            for i in (0..ITEM_COUNT).rev() {
                assert_eq!(iter.peek_back(), Some(2_usize * i).as_ref().as_ref());
                assert_eq!(iter.next_back(), Some(2_usize * i).as_ref());
            }
            assert_eq!(iter.peek(), None);
            assert_eq!(iter.peek_back(), None);
            assert_eq!(iter.next_back(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn last() {
            let vec = tester_vec();
            let mut iter = DoubleEndedPeekable::from_iter(vec.iter());

            let last_index = iter.len() - 1;
            assert_eq!(iter.nth(last_index), tester_vec().last());
        }

        #[test]
        fn third_back() {
            let vec = tester_vec();
            let mut iter = DoubleEndedPeekable::from_iter(vec.iter());

            assert_eq!(iter.nth_back(2), tester_vec().iter().nth_back(2));
        }

        #[test]
        fn collect() {
            let vec = tester_vec();
            let iter = DoubleEndedPeekable::from_iter(vec.iter());

            assert_eq!(iter.copied().collect::<Vec<usize>>(), vec);
        }
    }

    mod all_group {
        use crate::util::alias::{GroupId, ItemId};
        use crate::util::iter::IterLimitedByAllGroup;
        use std::collections::BTreeMap;

        const ITEM_COUNT: usize = 20;

        fn tester_map() -> BTreeMap<GroupId, BTreeMap<ItemId, usize>> {
            let mut map_list: BTreeMap<GroupId, BTreeMap<ItemId, usize>> = BTreeMap::new();
            for i in 0..ITEM_COUNT {
                map_list.entry(i % 5).or_default().insert(i, 2 * i);
            }
            map_list
        }

        #[test]
        fn incremental() {
            let map = tester_map();
            let mut iter = IterLimitedByAllGroup::from_btree_map(&map);
            for i in 0..ITEM_COUNT {
                assert_eq!(iter.next().map(|(k, _)| k), Some(i).as_ref());
            }
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);
        }

        #[test]
        fn decremental() {
            let map = tester_map();
            let mut iter = IterLimitedByAllGroup::from_btree_map(&map);
            for i in (0..ITEM_COUNT).rev() {
                assert_eq!(iter.next_back().map(|(k, _)| k), Some(i).as_ref());
            }
            assert_eq!(iter.next_back(), None);
            assert_eq!(iter.next(), None);
        }
    }

    mod limited_by_group_list {
        use crate::util::alias::{GroupId, ItemId};
        use crate::util::iter::IterLimitedByGroupList;
        use std::collections::BTreeMap;

        const ITEM_COUNT: usize = 20;
        const GROUP_COUNT: usize = 5;

        fn tester_group_list() -> Vec<usize> {
            (0..GROUP_COUNT).filter(|i| i % 2 == 0).collect()
        }

        fn tester_map() -> BTreeMap<GroupId, BTreeMap<ItemId, usize>> {
            let mut map_list: BTreeMap<GroupId, BTreeMap<ItemId, usize>> = BTreeMap::new();
            for i in 0..ITEM_COUNT {
                map_list
                    .entry(i % GROUP_COUNT)
                    .or_default()
                    .insert(i, 2 * i);
            }
            map_list
        }

        #[test]
        fn incremental() {
            let map = tester_map();
            let mut creator_group_list = tester_group_list();
            creator_group_list.push(GROUP_COUNT * 10);
            let mut iter = IterLimitedByGroupList::from_btree_map(&creator_group_list, &map);
            assert!(iter
                .using_groups()
                .iter()
                .zip(tester_group_list())
                .all(|(u, t)| u == &t));
            for i in (0..ITEM_COUNT).filter(|i| (i % GROUP_COUNT) % 2 == 0) {
                if iter.using_groups().contains(&(i % GROUP_COUNT)) {
                    assert_eq!(iter.next().map(|(k, _)| k), Some(i).as_ref());
                } else {
                    unreachable!("occurred error when index {}", i);
                }
            }
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn decremental() {
            let map = tester_map();
            let mut creator_group_list = tester_group_list();
            creator_group_list.push(GROUP_COUNT * 10);
            let mut iter = IterLimitedByGroupList::from_btree_map(&creator_group_list, &map);
            assert!(iter
                .using_groups()
                .iter()
                .zip(tester_group_list())
                .all(|(u, t)| u == &t));
            for i in (0..ITEM_COUNT).filter(|i| (i % GROUP_COUNT) % 2 == 0).rev() {
                if iter.using_groups().contains(&(i % GROUP_COUNT)) {
                    assert_eq!(iter.next_back().map(|(k, _)| k), Some(i).as_ref());
                } else {
                    unreachable!("occurred error when index {}", i);
                }
            }
            assert_eq!(iter.next_back(), None);
        }
    }

    mod limited_by_one_group {
        use crate::util::alias::{GroupId, ItemId};
        use crate::util::iter::IterLimitedByOneGroup;
        use std::collections::BTreeMap;

        const ITEM_COUNT: usize = 20;
        const GROUP_COUNT: usize = 5;
        const EXIST_GROUP_INDEX: usize = 3;
        const NOT_EXIST_GROUP_INDEX: usize = 30;

        fn tester_map() -> BTreeMap<GroupId, BTreeMap<ItemId, usize>> {
            let mut map_list: BTreeMap<GroupId, BTreeMap<ItemId, usize>> = BTreeMap::new();
            for i in 0..ITEM_COUNT {
                map_list
                    .entry(i % GROUP_COUNT)
                    .or_default()
                    .insert(i, 2 * i);
            }
            map_list
        }

        #[test]
        fn exist_incremental() {
            let map = tester_map();
            let mut iter = IterLimitedByOneGroup::from_btree_map(&EXIST_GROUP_INDEX, &map);
            assert_eq!(iter.get_group(), EXIST_GROUP_INDEX);
            for i in 0..ITEM_COUNT {
                if iter.get_group() == i % GROUP_COUNT {
                    assert_eq!(iter.next().map(|(k, _)| k), Some(i).as_ref());
                }
            }
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn not_exist_incremental() {
            let map = tester_map();
            let mut iter = IterLimitedByOneGroup::from_btree_map(&NOT_EXIST_GROUP_INDEX, &map);
            assert_eq!(iter.get_group(), NOT_EXIST_GROUP_INDEX);
            assert!(!iter.has_iter());
            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn exist_decremental() {
            let map = tester_map();
            let mut iter = IterLimitedByOneGroup::from_btree_map(&EXIST_GROUP_INDEX, &map);
            assert_eq!(iter.get_group(), EXIST_GROUP_INDEX);
            for i in (0..ITEM_COUNT).rev() {
                if iter.get_group() == i % GROUP_COUNT {
                    assert_eq!(iter.next_back().map(|(k, _)| k), Some(i).as_ref());
                }
            }
            assert_eq!(iter.next_back(), None);
        }

        #[test]
        fn not_exist_decremental() {
            let map = tester_map();
            let mut iter = IterLimitedByOneGroup::from_btree_map(&NOT_EXIST_GROUP_INDEX, &map);
            assert_eq!(iter.get_group(), NOT_EXIST_GROUP_INDEX);
            assert!(!iter.has_iter());
            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next_back(), None);
        }
    }
}
