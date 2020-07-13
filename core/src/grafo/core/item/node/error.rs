use crate::grafo::core::item::{HasItemKind, ItemErrorBase};
use crate::grafo::GrafoError;
use crate::util::item_kind::ItemKind;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum NodeItemError {
    // TODO
}

impl HasItemKind for NodeItemError {
    fn kind() -> ItemKind {
        ItemKind::Node
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
