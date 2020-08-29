//! traits for an item and the item's builder

use std::error::Error;

use crate::grafo::GrafoError;
use crate::grafo::Resolver;
use crate::util::alias::ItemId;
use crate::util::name_type::NameType;

/// trait for base of builder for item
pub trait ItemBuilderBase<Name: NameType> {
    /// result when build success
    type Item: ItemBase;
    /// result have this error when build success and fail with error
    type ItemError: ItemErrorBase<Name>;
}

/// alias for result of builder's build method
pub(crate) type ItemBuilderResult<Name, Item, ItemOption> =
    (Option<(Item, ItemOption)>, Vec<GrafoError<Name>>);

/// add build method to builder
pub(crate) trait HasItemBuilderMethod<Name: NameType>: ItemBuilderBase<Name> {
    /// optional result for builder
    type ItemOption;
    /// build method for builder
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption>;
}

/// base trait for item
pub trait ItemBase {
    /// getter for item id
    fn get_item_id(&self) -> ItemId;
}

/// base trait for item builder's error
pub trait ItemErrorBase<Name: NameType>: Eq + Error + Into<GrafoError<Name>> {}

/// error converter from an error to an error with item id
pub trait FromWithItemId<T, Name: NameType> {
    /// converter method
    fn from_with_id(item_id: ItemId, name: Option<Name>, from: T) -> Self;
}
