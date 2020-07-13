//! ReGRaFiLo's Layout

use crate::util::alias::ItemIndex;
use crate::util::kind_key::KeyWithKind;

pub mod error;
pub(crate) mod attribute;
mod reference;

use crate::util::item_kind::ItemKind;
pub use reference::*;

/// helper for make reference key for layout
fn create_layout_key<LayoutKey>(
    item_kind: ItemKind,
    key: LayoutKey,
    index: ItemIndex,
) -> KeyWithKind<ItemKind, KeyWithKind<ItemIndex, LayoutKey>> {
    KeyWithKind::new(item_kind, KeyWithKind::new(index, key))
}
