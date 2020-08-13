use std::error::Error;

use crate::grafo::GrafoError;
use crate::grafo::Resolver;
use crate::util::alias::ItemId;
use crate::util::name_type::NameType;

pub trait ItemBuilderBase<Name: NameType> {
    type Item: ItemBase;
    type ItemError: ItemErrorBase<Name>;
}

pub(crate) type ItemBuilderResult<Name, Item, ItemOption> =
    (Option<(Item, ItemOption)>, Vec<GrafoError<Name>>);

pub(crate) trait HasItemBuilderMethod<Name: NameType>: ItemBuilderBase<Name> {
    type ItemOption;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption>;
}

pub trait ItemBase: Copy {
    fn get_item_id(&self) -> ItemId;
}

pub trait ItemErrorBase<Name: NameType>: Eq + Error + Into<GrafoError<Name>> {}

pub trait FromWithItemId<T> {
    fn from_with_id(item_id: ItemId, from: T) -> Self;
}
