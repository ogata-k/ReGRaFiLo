use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::graph_item::group::GroupItem;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GroupItemError<Name: NameType> {
    FailResolveBelongGroup(ItemId, Option<Name>),
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
}

impl<Name: NameType> HasGraphItemKind for GroupItemError<Name> {
    fn kind() -> GraphItemKind {
        GroupItem::kind()
    }
}

impl<Name: NameType> Display for GroupItemError<Name> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType> Into<GrafoError<Name>> for GroupItemError<Name> {
    fn into(self) -> GrafoError<Name> {
        GrafoError::GroupItemError(self)
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
