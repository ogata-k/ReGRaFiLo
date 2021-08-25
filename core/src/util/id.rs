//! Module for id

/// Id trait for ReGRaFiLo's Item
pub trait Identity: Eq + Ord {}

impl<T: Eq + Ord> Identity for T {}
