//! base of item and item builder

use crate::util::kind_key::KindKey;
use crate::util::RefIndex;
use regrafilo_util::log::KindBase;

/// index of item<br/>
/// alias of usize because of use as vector index
pub type ItemIndex = usize;

/// RefIndex for ItemIndex
pub type RefIndexOfItem<K, T> = RefIndex<KindKey<K, T>, ItemIndex>;

/// Item Builder's base set
pub trait ItemBuilderBase<Ext> {
    type ItemKind: KindBase;
    type Item: ItemBase<ItemKind = Self::ItemKind>;
    fn kind() -> Self::ItemKind;
    fn set_group_id(&mut self, group_id: ItemIndex) -> &mut Self;
    fn get_group_id(&self) -> ItemIndex;
    fn build(self, item_index: ItemIndex) -> (Self::Item, Ext);
}

/// Item's base set
pub trait ItemBase {
    type ItemKind: KindBase;
    fn kind() -> Self::ItemKind;
    fn get_group_id(&self) -> ItemIndex;
    fn get_item_id(&self) -> ItemIndex;
}
