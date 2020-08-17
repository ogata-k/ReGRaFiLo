use crate::grafo::core::graph_item::item::edge::EdgeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::NameIdError;
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EdgeItemError<Name: NameType> {
    FailResolveBelongGroup(ItemId, Option<Name>),
    NotSpecifyStartEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    FailResolveStartEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    NotSpecifyEndEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    FailResolveEndEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
    CannotSpecifyBelongGroupAsEndpoint(ItemId, (GraphItemKind, Name)),
}

impl<Name: NameType> HasGraphItemKind for EdgeItemError<Name> {
    fn kind() -> GraphItemKind {
        EdgeItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for EdgeItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

impl<Name: NameType> Error for EdgeItemError<Name> {}
impl<Name: NameType> ItemErrorBase<Name> for EdgeItemError<Name> {}
impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>> for EdgeItemError<Name> {
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, GraphItemKind>) -> Self {
        EdgeItemError::NameIdError(item_id, from)
    }
}
impl<Name: NameType> GraphBuilderErrorBase<Name> for EdgeItemError<Name> {}
