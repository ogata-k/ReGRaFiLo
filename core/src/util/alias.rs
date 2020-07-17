//! type alias

use std::collections::HashMap;

/// references indexes
pub(crate) type RefIndex<K, V> = HashMap<K, V>;

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemId = usize;
pub type GraphItemId = ItemId;
pub type GroupId = ItemId;
pub type LayoutItemId = ItemId;
