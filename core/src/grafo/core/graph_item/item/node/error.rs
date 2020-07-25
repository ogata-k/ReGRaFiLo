use crate::grafo::core::graph_item::item::node::NodeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::item_base::ItemBuilderErrorBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NodeItemError {
    // TODO
    FailResolveBelongGroup,
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
impl ItemBuilderErrorBase for NodeItemError {}
impl From<NameIdError<GraphItemKind>> for NodeItemError {
    fn from(error: NameIdError<GraphItemKind>) -> Self {
        unimplemented!()
    }
}
impl GraphBuilderErrorBase for NodeItemError {}
