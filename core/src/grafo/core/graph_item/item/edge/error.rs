use crate::grafo::core::graph_item::item::edge::EdgeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::item_base::ItemErrorBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EdgeItemError {
    // TODO
}

impl HasGraphItemKind for EdgeItemError {
    fn kind() -> GraphItemKind {
        EdgeItem::kind()
    }
}

impl Display for EdgeItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Into<GrafoError> for EdgeItemError {
    fn into(self) -> GrafoError {
        GrafoError::EdgeItemError(self)
    }
}

impl Error for EdgeItemError {}
impl ItemErrorBase for EdgeItemError {}
impl From<NameIdError<GraphItemKind>> for EdgeItemError {
    fn from(error: NameIdError<GraphItemKind>) -> Self {
        unimplemented!()
    }
}
impl GraphBuilderErrorBase for EdgeItemError {}
