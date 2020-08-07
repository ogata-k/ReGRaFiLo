use std::fmt::{Debug, Display};
use std::hash::Hash;

pub trait NameType: Eq + Hash + Ord + Clone + Debug + Display {}

impl<N: Eq + Hash + Ord + Clone + Debug + Display> NameType for N {}
