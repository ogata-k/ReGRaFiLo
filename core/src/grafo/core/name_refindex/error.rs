use crate::grafo::GrafoError;
use crate::util::kind::{GraphItemKind, LayoutItemKind};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameRefError<Kind> {
    Override(Kind, String),
    NotExist(Kind, String),
}

impl<Kind: Display> Display for NameRefError<Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Kind: Debug + Display> Error for NameRefError<Kind> {}

impl Into<GrafoError> for NameRefError<GraphItemKind> {
    fn into(self) -> GrafoError {
        GrafoError::ItemNameRefError(self)
    }
}

impl Into<GrafoError> for NameRefError<LayoutItemKind> {
    fn into(self) -> GrafoError {
        GrafoError::LayoutNameRefError(self)
    }
}
