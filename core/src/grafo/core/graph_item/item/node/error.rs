use crate::grafo::core::graph_item::GraphItemErrorBase;
use crate::grafo::GrafoError;
use crate::util::item_base::ItemErrorBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum NodeItemError {
    // TODO
}

impl HasGraphItemKind for NodeItemError {
    fn kind() -> GraphItemKind {
        GraphItemKind::Node
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
impl GraphItemErrorBase for NodeItemError {}
