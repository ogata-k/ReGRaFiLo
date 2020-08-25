//! Grafo's name type

use std::fmt::{Debug, Display};
use std::hash::Hash;

/// can check Equal, Hashing, Order and Duplicate as Clone and show as Debug and Display.
pub trait NameType: Eq + Hash + Ord + Clone + Debug + Display {}

impl<N: Eq + Hash + Ord + Clone + Debug + Display> NameType for N {}
