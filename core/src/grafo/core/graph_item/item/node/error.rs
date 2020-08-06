use crate::grafo::graph_item::item::node::NodeItem;
use crate::grafo::graph_item::GraphBuilderErrorBase;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::{NameType, StoredNameType};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NodeItemError<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    FailResolveBelongGroup(ItemId),
    NameIdError(ItemId, NameIdError<Name, StoredName, GraphItemKind>),
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> HasGraphItemKind
    for NodeItemError<Name, StoredName>
{
    fn kind() -> GraphItemKind {
        NodeItem::kind()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Display
    for NodeItemError<Name, StoredName>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    Into<GrafoError<Name, StoredName>> for NodeItemError<Name, StoredName>
{
    fn into(self) -> GrafoError<Name, StoredName> {
        GrafoError::NodeItemError(self)
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Error
    for NodeItemError<Name, StoredName>
{
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> ItemErrorBase<Name, StoredName>
    for NodeItemError<Name, StoredName>
{
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    FromWithItemId<NameIdError<Name, StoredName, GraphItemKind>>
    for NodeItemError<Name, StoredName>
{
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, StoredName, GraphItemKind>) -> Self {
        NodeItemError::NameIdError(item_id, from)
    }
}
impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    GraphBuilderErrorBase<Name, StoredName> for NodeItemError<Name, StoredName>
{
}
