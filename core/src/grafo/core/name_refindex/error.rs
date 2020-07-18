use crate::grafo::GrafoError;
use crate::util::kind::{GraphItemKind, LayoutItemKind};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameRefWarning<Kind> {
    Override(Kind, String),
    NotExist(Kind, String),
}

impl<Kind: Display> Display for NameRefWarning<Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Kind: Debug + Display> Error for NameRefWarning<Kind> {}

impl Into<GrafoError> for NameRefWarning<GraphItemKind> {
    fn into(self) -> GrafoError {
        GrafoError::ItemNameRefWarning(self)
    }
}

impl Into<GrafoError> for NameRefWarning<LayoutItemKind> {
    fn into(self) -> GrafoError {
        GrafoError::AttributeNameRefWarning(self)
    }
}
