use crate::grafo::GrafoError;
use crate::util::item_kind::ItemKind;
use crate::util::layout_kind::AttributeKind;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum NameRefWarning<Kind> {
    Override(Kind, String),
}

impl<Kind: Display> Display for NameRefWarning<Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Kind: Debug + Display> Error for NameRefWarning<Kind> {}

impl Into<GrafoError> for NameRefWarning<ItemKind> {
    fn into(self) -> GrafoError {
        GrafoError::ItemNameRefWarning(self)
    }
}

impl Into<GrafoError> for NameRefWarning<(ItemKind, AttributeKind)> {
    fn into(self) -> GrafoError {
        GrafoError::AttributeNameRefWarning(self)
    }
}
