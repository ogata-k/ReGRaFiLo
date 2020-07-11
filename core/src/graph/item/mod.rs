//! helper for item

mod item_arena;
mod item_base;

/// RefIndex for ItemIndex
#[allow(dead_code)]
type RefIndexOfItem<K, T> = RefIndex<KeyWithKind<K, T>, ItemIndex>;

use crate::util::alias::{ItemIndex, RefIndex};
use crate::util::kind_key::KeyWithKind;
pub use item_arena::*;
pub use item_base::*;
