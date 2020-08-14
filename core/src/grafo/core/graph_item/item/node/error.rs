use crate::grafo::graph_item::item::node::NodeItem;
use crate::grafo::graph_item::GraphBuilderErrorBase;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NodeItemError<Name: NameType> {
    FailResolveBelongGroup(ItemId, Option<Name>),
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
}

impl<Name: NameType> HasGraphItemKind for NodeItemError<Name> {
    fn kind() -> GraphItemKind {
        NodeItem::kind()
    }
}

impl<Name: NameType> Display for NodeItemError<Name> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType> Into<GrafoError<Name>> for NodeItemError<Name> {
    fn into(self) -> GrafoError<Name> {
        GrafoError::NodeItemError(self)
    }
}

impl<Name: NameType> Error for NodeItemError<Name> {}

impl<Name: NameType> ItemErrorBase<Name> for NodeItemError<Name> {}

impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>> for NodeItemError<Name> {
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, GraphItemKind>) -> Self {
        NodeItemError::NameIdError(item_id, from)
    }
}

impl<Name: NameType> GraphBuilderErrorBase<Name> for NodeItemError<Name> {}
