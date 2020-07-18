use crate::grafo::core::graph_item::GraphItemErrorBase;
use crate::grafo::GrafoError;
use crate::util::item_base::ItemErrorBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum EdgeItemError {
    // TODO
}

impl HasGraphItemKind for EdgeItemError {
    fn kind() -> GraphItemKind {
        GraphItemKind::Edge
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
impl GraphItemErrorBase for EdgeItemError {}
