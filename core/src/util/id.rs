//! Module for id
use std::fmt;

/// Id trait for ReGRaFiLo's Item
pub trait Identity: Eq + Ord + Clone + fmt::Debug {}

impl<T: Eq + Ord + Clone + fmt::Debug> Identity for T {}
