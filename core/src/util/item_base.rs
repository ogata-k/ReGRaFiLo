use std::error::Error;

use crate::grafo::GrafoError;
use crate::grafo::Resolver;
use crate::util::alias::ItemId;
use crate::util::name_type::{NameType, StoredNameType};

pub trait ItemBuilderBase<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    type Item: ItemBase;
    type ItemError: ItemErrorBase<Name, StoredName>;
}

pub(crate) type ItemBuilderResult<
    Name: NameType<StoredName>,
    StoredName: StoredNameType<Name>,
    Item,
    ItemOption,
> = (
    Option<(Item, ItemOption)>,
    Vec<GrafoError<Name, StoredName>>,
);
pub(crate) trait HasItemBuilderMethod<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>:
    ItemBuilderBase<Name, StoredName>
{
    type ItemOption;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name, StoredName>,
    ) -> ItemBuilderResult<Name, StoredName, Self::Item, Self::ItemOption>;
}

pub trait ItemBase {
    fn get_item_id(&self) -> ItemId;
}

pub trait ItemErrorBase<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>:
    Eq + Error + Into<GrafoError<Name, StoredName>>
{
}

pub trait FromWithItemId<T> {
    fn from_with_id(item_id: ItemId, from: T) -> Self;
}
