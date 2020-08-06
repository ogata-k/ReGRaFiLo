use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait NameType<BorrowedName>:
    Into<BorrowedName> + Borrow<BorrowedName> + Eq + Hash + Ord + Clone + Debug + Display
{
}

pub trait StoredNameType<Name>: Into<Name> + Eq + Ord + Hash + Clone + Debug + Display {}

impl<N: Into<S> + Borrow<S> + Eq + Hash + Ord + Clone + Debug + Display, S> NameType<S> for N {}
impl<N, S: Into<N> + Eq + Ord + Hash + Clone + Debug + Display> StoredNameType<N> for S {}
