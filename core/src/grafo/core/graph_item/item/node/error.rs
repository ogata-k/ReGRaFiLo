use crate::grafo::graph_item::item::node::NodeItem;
use crate::grafo::graph_item::GraphBuilderErrorBase;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NodeItemError {
    FailResolveBelongGroup(ItemId),
    NameIdError(ItemId, NameIdError<GraphItemKind>),
}

impl HasGraphItemKind for NodeItemError {
    fn kind() -> GraphItemKind {
        NodeItem::kind()
    }
}

impl Display for NodeItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Into<GrafoError> for NodeItemError {
    fn into(self) -> GrafoError {
        GrafoError::NodeItemError(self)
    }
}

impl Error for NodeItemError {}
impl ItemErrorBase for NodeItemError {}
impl FromWithItemId<NameIdError<GraphItemKind>> for NodeItemError {
    fn from_with_id(item_id: ItemId, from: NameIdError<GraphItemKind>) -> Self {
        NodeItemError::NameIdError(item_id, from)
    }
}
impl GraphBuilderErrorBase for NodeItemError {}
