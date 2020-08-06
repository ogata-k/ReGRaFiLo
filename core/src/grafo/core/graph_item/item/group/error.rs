use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::graph_item::group::GroupItem;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::{NameType, StoredNameType};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GroupItemError<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    // TODO
    FailResolveBelongGroup(ItemId),
    NameIdError(ItemId, NameIdError<Name, StoredName, GraphItemKind>),
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> HasGraphItemKind
    for GroupItemError<Name, StoredName>
{
    fn kind() -> GraphItemKind {
        GroupItem::kind()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Display
    for GroupItemError<Name, StoredName>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    Into<GrafoError<Name, StoredName>> for GroupItemError<Name, StoredName>
{
    fn into(self) -> GrafoError<Name, StoredName> {
        GrafoError::GroupItemError(self)
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Error
    for GroupItemError<Name, StoredName>
{
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> ItemErrorBase<Name, StoredName>
    for GroupItemError<Name, StoredName>
{
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    FromWithItemId<NameIdError<Name, StoredName, GraphItemKind>>
    for GroupItemError<Name, StoredName>
{
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, StoredName, GraphItemKind>) -> Self {
        unimplemented!()
    }
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    GraphBuilderErrorBase<Name, StoredName> for GroupItemError<Name, StoredName>
{
}
