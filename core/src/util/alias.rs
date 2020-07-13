//! type alias

use std::collections::BTreeMap;

/// references indexes
pub(crate) type RefIndex<K, V> = BTreeMap<K, V>;

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;
pub type GroupIndex = ItemIndex;
