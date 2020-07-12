//! base of item and item builder

use crate::util::alias::ItemIndex;
use crate::util::util_trait::KindBase;

/// Item Builder's base set
pub trait ItemBuilderBase {
    type ItemKind: KindBase;
    type Item: ItemBase<ItemKind = Self::ItemKind>;
    type ItemOption;
    type BuildFailErr;

    fn kind() -> Self::ItemKind;
    fn set_group_id(&mut self, group_id: ItemIndex) -> &mut Self;
    fn get_group_id(&self) -> ItemIndex;
    fn build(self) -> Result<(Self::Item, Self::ItemOption), Self::BuildFailErr>;
}

/// Item's base set
pub trait ItemBase {
    type ItemKind: KindBase;
    fn kind() -> Self::ItemKind;
    fn get_group_id(&self) -> ItemIndex;
    fn get_item_id(&self) -> ItemIndex;
}
