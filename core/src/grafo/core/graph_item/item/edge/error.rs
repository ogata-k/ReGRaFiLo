use crate::grafo::core::graph_item::item::edge::EdgeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::{NameType, StoredNameType};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EdgeItemError<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    // TODO
    FailResolveBelongGroup(ItemId),
    NameIdError(ItemId, NameIdError<Name, StoredName, GraphItemKind>),
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> HasGraphItemKind
    for EdgeItemError<Name, StoredName>
{
    fn kind() -> GraphItemKind {
        EdgeItem::kind()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Display
    for EdgeItemError<Name, StoredName>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    Into<GrafoError<Name, StoredName>> for EdgeItemError<Name, StoredName>
{
    fn into(self) -> GrafoError<Name, StoredName> {
        GrafoError::EdgeItemError(self)
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Error
    for EdgeItemError<Name, StoredName>
{
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> ItemErrorBase<Name, StoredName>
    for EdgeItemError<Name, StoredName>
{
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    FromWithItemId<NameIdError<Name, StoredName, GraphItemKind>>
    for EdgeItemError<Name, StoredName>
{
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, StoredName, GraphItemKind>) -> Self {
        unimplemented!()
    }
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    GraphBuilderErrorBase<Name, StoredName> for EdgeItemError<Name, StoredName>
{
}
