use std::error::Error;

use crate::grafo::GrafoError;
use crate::grafo::Resolver;
use crate::util::alias::ItemId;

pub trait ItemBuilderBase {
    type Item: ItemBase;
    type ItemError: ItemErrorBase;
}

pub(crate) type ItemBuilderResult<Item, ItemOption> = (Option<(Item, ItemOption)>, Vec<GrafoError>);
pub(crate) trait HasItemBuilderMethod: ItemBuilderBase {
    type ItemOption;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption>;
}

pub trait ItemBase {
    fn get_item_id(&self) -> ItemId;
}

pub trait ItemErrorBase: Eq + Error + Into<GrafoError> {}

pub trait FromWithItemId<T> {
    fn from_with_id(item_id: ItemId, from: T) -> Self;
}
