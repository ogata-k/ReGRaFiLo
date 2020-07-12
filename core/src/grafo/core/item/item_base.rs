//! base of item and item builder

use crate::util::alias::ItemIndex;
use crate::util::item_kind::ItemKind;
use std::error::Error;

/// Item Builder's base set
pub trait ItemBuilderBase {
    type Item: ItemBase;
    type ItemOption;
    type BuildFailErr: Error;

    fn kind() -> ItemKind;
    fn set_group_id(&mut self, group_id: ItemIndex) -> &mut Self;
    fn get_group_id(&self) -> ItemIndex;
    fn build(self) -> Result<(Self::Item, Self::ItemOption), Self::BuildFailErr>;
}

/// Item's base set
pub trait ItemBase {
    fn kind() -> ItemKind;
    fn get_group_id(&self) -> ItemIndex;
    fn get_item_id(&self) -> ItemIndex;
}
