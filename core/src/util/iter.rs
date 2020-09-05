//! utility for iterator

use std::collections::BTreeMap;
use std::iter::FusedIterator;

use crate::util::alias::{GroupId, ItemId};

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

/// iterator for all items ordering by item_id in all groups
#[derive(Debug, Clone)]
pub struct IterGroupByAll<'a, I: 'a> {
    iters: Vec<DoubleEndedPeekable<std::collections::btree_map::Iter<'a, ItemId, I>>>,
}

impl<'a, I: 'a> Iterator for IterGroupByAll<'a, I> {
    type Item = (&'a ItemId, &'a I);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut min_item_id: Option<ItemId> = None;
        // It is assumed that there are few registered group_ids.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_id, _)) = iterable.peek() {
                match min_item_id {
                    None => {
                        target_index = Some(index);
                        min_item_id = Some(**item_id);
                    }
                    Some(_min_item_id) if _min_item_id >= **item_id => {
                        target_index = Some(index);
                        min_item_id = Some(**item_id);
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

impl<'a, I: 'a> ExactSizeIterator for IterGroupByAll<'a, I> {}

impl<'a, I: 'a> DoubleEndedIterator for IterGroupByAll<'a, I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut max_item_id: Option<ItemId> = None;
        // It is assumed that there are few registered group_ids.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_id, _)) = iterable.peek_back() {
                match max_item_id {
                    None => {
                        target_index = Some(index);
                        max_item_id = Some(**item_id);
                    }
                    Some(_min_item_id) if _min_item_id <= **item_id => {
                        target_index = Some(index);
                        max_item_id = Some(**item_id);
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

impl<'a, I: 'a> FusedIterator for IterGroupByAll<'a, I> {}

impl<'a, I: 'a> IterGroupByAll<'a, I> {
    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_btree_map(map: &'a BTreeMap<GroupId, BTreeMap<ItemId, I>>) -> Self {
        let mut iters = Vec::new();
        for (_, map) in map.iter() {
            iters.push(DoubleEndedPeekable::from_iter(map.iter()));
        }
        IterGroupByAll { iters }
    }
}

/// iterator for all items ordering by item_id in specified groups
#[derive(Debug, Clone)]
pub struct IterGroupByList<'a, I: 'a> {
    groups: Vec<GroupId>,
    iters: Vec<DoubleEndedPeekable<std::collections::btree_map::Iter<'a, ItemId, I>>>,
}

impl<'a, I: 'a> Iterator for IterGroupByList<'a, I> {
    type Item = (&'a ItemId, &'a I);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut min_item_id: Option<ItemId> = None;
        // It is assumed that there are few registered group_ids.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_id, _)) = iterable.peek() {
                match min_item_id {
                    None => {
                        target_index = Some(index);
                        min_item_id = Some(**item_id);
                    }
                    Some(_min_item_id) if _min_item_id >= **item_id => {
                        target_index = Some(index);
                        min_item_id = Some(**item_id);
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

impl<'a, I: 'a> ExactSizeIterator for IterGroupByList<'a, I> {}

impl<'a, I: 'a> DoubleEndedIterator for IterGroupByList<'a, I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut target_index: Option<usize> = None;
        let mut max_item_id: Option<ItemId> = None;
        // It is assumed that there are few registered group_ids.
        for (index, iterable) in self.iters.iter_mut().enumerate() {
            if let Some((item_id, _)) = iterable.peek_back() {
                match max_item_id {
                    None => {
                        target_index = Some(index);
                        max_item_id = Some(**item_id);
                    }
                    Some(_min_item_id) if _min_item_id <= **item_id => {
                        target_index = Some(index);
                        max_item_id = Some(**item_id);
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

impl<'a, I: 'a> FusedIterator for IterGroupByList<'a, I> {}

impl<'a, I: 'a> IterGroupByList<'a, I> {
    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_btree_map(
        groups: &[GroupId],
        map: &'a BTreeMap<GroupId, BTreeMap<ItemId, I>>,
    ) -> Self {
        let mut list = Vec::new();
        let mut iters = Vec::new();
        for (group_id, map) in map.iter() {
            if groups.contains(group_id) {
                list.push(*group_id);
                iters.push(DoubleEndedPeekable::from_iter(map.iter()));
            }
        }
        IterGroupByList {
            groups: list,
            iters,
        }
    }

    /// get specified group list for limiter of this iterator.
    pub fn using_groups(&self) -> &[GroupId] {
        self.groups.as_slice()
    }
}

/// iterator for all items ordering by item_id grouped by group_id
#[derive(Debug, Clone)]
pub struct IterGroupById<'a, I: 'a> {
    group_id: GroupId,
    inner_iter: Option<std::collections::btree_map::Iter<'a, ItemId, I>>,
}

impl<'a, I: 'a> Iterator for IterGroupById<'a, I> {
    type Item = (&'a ItemId, &'a I);

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

impl<'a, I: 'a> ExactSizeIterator for IterGroupById<'a, I> {}

impl<'a, I: 'a> DoubleEndedIterator for IterGroupById<'a, I> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner_iter.as_mut() {
            None => None,
            Some(inner) => inner.next_back(),
        }
    }

    #[inline]
    fn nth_back(&mut self, mut n: usize) -> Option<Self::Item> {
        match self.inner_iter.as_mut() {
            None => None,
            Some(inner) => inner.nth(n),
        }
    }

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

impl<'a, I: 'a> FusedIterator for IterGroupById<'a, I> {}

impl<'a, I: 'a> IterGroupById<'a, I> {
    /// initializer for this iterator.
    /// This group id is group's id for I.
    pub fn from_map(group_id: GroupId, map: Option<&'a BTreeMap<ItemId, I>>) -> Self {
        IterGroupById {
            group_id,
            inner_iter: map.map(|map| map.iter()),
        }
    }

    /// group id for grouping
    pub fn get_group_id(&self) -> GroupId {
        self.group_id
    }

    /// check iter has item. This is **NOT** checker for result of next() is None.
    pub fn has_iter(&self) -> bool {
        self.inner_iter.is_some()
    }
}

#[cfg(test)]
mod test {
    // TODO  動作確認
}
