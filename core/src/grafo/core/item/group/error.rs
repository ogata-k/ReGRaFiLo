use crate::grafo::core::item::{HasItemKind, ItemErrorBase};
use crate::grafo::GrafoError;
use crate::util::item_kind::ItemKind;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum GroupItemError {
    // TODO
}

impl HasItemKind for GroupItemError {
    fn kind() -> ItemKind {
        ItemKind::Group
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

impl ItemErrorBase for GroupItemError {}
