//! type alias

use crate::util::kind_key::KeyWithKind;
use std::collections::BTreeMap;

/// references indexes
pub(crate) type RefIndex<K, V> = BTreeMap<K, V>;

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;

/// RefIndex for ItemIndex
pub(crate) type RefIndexOfItem<K, T> = RefIndex<KeyWithKind<K, T>, ItemIndex>;
