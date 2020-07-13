use crate::grafo::GrafoError;
use crate::util::item_kind::ItemKind;
use crate::util::layout_kind::AttributeKind;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum AttributeWarning {
    Override(ItemKind, AttributeKind, String),
}

impl Display for AttributeWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for AttributeWarning {}

impl Into<GrafoError> for AttributeWarning{
    fn into(self) -> GrafoError {
       GrafoError::AttributeWarning(self)
    }
}
