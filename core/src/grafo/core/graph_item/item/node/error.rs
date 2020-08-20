use crate::grafo::graph_item::item::node::NodeItem;
use crate::grafo::graph_item::GraphBuilderErrorBase;
use crate::grafo::NameIdError;
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

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

impl<Name: NameType> std::fmt::Display for NodeItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeItemError::FailResolveBelongGroup(item_id, None) => {
                write!(f, "Node {}: not specify belong group", item_id)
            }
            NodeItemError::FailResolveBelongGroup(item_id, Some(name)) => {
                write!(f, "Node {}: not found belong group \"{}\"", item_id, name)
            }
            NodeItemError::NameIdError(item_id, e) => write!(f, "Node {}: {}", item_id, e),
        }
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
