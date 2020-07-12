//! ReGRaFiLo's Layout

use crate::util::alias::ItemIndex;
use crate::util::kind_key::KeyWithKind;
use crate::util::util_trait::KindBase;

pub(crate) mod attribute;

/// helper for make reference key for layout
fn create_layout_key<LayoutKey: KindBase, ItemKindKey: KindBase>(
    item_kind: ItemKindKey,
    key: LayoutKey,
    index: ItemIndex,
) -> KeyWithKind<ItemKindKey, KeyWithKind<ItemIndex, LayoutKey>> {
    KeyWithKind::new(item_kind, KeyWithKind::new(index, key))
}
