//! utility for iterator

use std::iter::FusedIterator;

/// iterator with peekable from start and end.
#[derive(Clone, Debug)]
pub struct DoubleEndedPeekable<I: FusedIterator + DoubleEndedIterator + Iterator> {
    iter: I,
    peeked: Option<Option<I::Item>>,
    peeked_last: Option<Option<I::Item>>,
}

impl<I: FusedIterator + DoubleEndedIterator + Iterator> DoubleEndedPeekable<I> {
    pub(super) fn new(iter: I) -> DoubleEndedPeekable<I> {
        DoubleEndedPeekable {
            iter,
            peeked: None,
            peeked_last: None,
        }
    }
}

impl<I: FusedIterator + DoubleEndedIterator + Iterator> Iterator for DoubleEndedPeekable<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => {
                let item = self.iter.next();
                if item.is_none() {
                    match self.peeked_last.take() {
                        Some(v) => v,
                        None => None,
                    }
                } else {
                    item
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
                    let count = self.iter.by_ref().count();
                    if count == n - 1 {
                        self.peeked_last.take().flatten()
                    } else {
                        self.iter.nth(n - 1)
                    }
                }
            },
            None => self.iter.nth(n),
        }
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let acc = match self.peeked {
            Some(None) => return init,
            Some(Some(v)) => fold(init, v),
            None => init,
        };
        let new_acc = self.iter.fold(acc, fold);
        match self.peeked_last {
            Some(None) | None => new_acc,
            Some(Some(v)) => fold(new_acc, v),
        }
    }
}

impl<I: FusedIterator + DoubleEndedIterator + Iterator> FusedIterator for DoubleEndedPeekable<I> {}

impl<I: FusedIterator + DoubleEndedIterator + Iterator> ExactSizeIterator for DoubleEndedPeekable<I> {}

impl<I: FusedIterator + DoubleEndedIterator + Iterator> DoubleEndedIterator
    for DoubleEndedPeekable<I>
where
    I: DoubleEndedIterator,
{
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

    #[inline]
    fn rfold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let acc = match self.peeked_last {
            Some(None) => init,
            Some(Some(v)) => self.iter.rfold(fold(init, v), fold),
            None => self.iter.rfold(init, fold),
        };
        match self.peeked {
            Some(None)| None => acc,
            Some(Some(v)) => fold(acc, v),
        }
    }
}

impl<I: FusedIterator + DoubleEndedIterator + Iterator> DoubleEndedPeekable<I> {
    #[inline]
    pub fn peek(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }

    #[inline]
    pub fn peek_back(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.peeked_last.get_or_insert_with(|| iter.next_back()).as_ref()
    }

    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        match self.next() {
            Some(matched) if func(&matched) => Some(matched),
            other => {
                // Since we called `self.next()`, we consumed `self.peeked`.
                assert!(self.peeked.is_none());
                self.peeked = Some(other);
                None
            }
        }
    }

    pub fn next_if_eq<R>(&mut self, expected: &R) -> Option<I::Item>
    where
        R: ?Sized,
        I::Item: PartialEq<R>,
    {
        self.next_if(|next| next == expected)
    }
}

#[cfg(test)]
mod test {
    // TODO 実装
}