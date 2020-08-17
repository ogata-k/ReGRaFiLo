use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::graph_item::group::GroupItem;
use crate::grafo::NameIdError;
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GroupItemError<Name: NameType> {
    CannotSpecifyBelongGroupForRoot(ItemId, Name),
    FailResolveBelongGroup(ItemId, Option<Name>),
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
}

impl<Name: NameType> HasGraphItemKind for GroupItemError<Name> {
    fn kind() -> GraphItemKind {
        GroupItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for GroupItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

impl<Name: NameType> Error for GroupItemError<Name> {}
impl<Name: NameType> ItemErrorBase<Name> for GroupItemError<Name> {}
impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>> for GroupItemError<Name> {
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, GraphItemKind>) -> Self {
        Self::NameIdError(item_id, from)
    }
}
impl<Name: NameType> GraphBuilderErrorBase<Name> for GroupItemError<Name> {}
