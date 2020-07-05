//! utility of ReGRaFiLo's core
use std::collections::BTreeMap;

pub mod item_arena;
pub mod kind_key;

//
// type alias
//

/// references indexes
pub type RefIndex<K, V> = BTreeMap<K, V>;
