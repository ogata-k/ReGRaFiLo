use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::GrafoError;
use crate::util::item_base::ItemBuilderErrorBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum GroupItemError {
    // TODO
}

impl HasGraphItemKind for GroupItemError {
    fn kind() -> GraphItemKind {
        GraphItemKind::Group
    }
}

impl Display for GroupItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Into<GrafoError> for GroupItemError {
    fn into(self) -> GrafoError {
        GrafoError::GroupItemError(self)
    }
}

impl Error for GroupItemError {}
impl ItemBuilderErrorBase for GroupItemError {}
impl GraphBuilderErrorBase for GroupItemError {}
