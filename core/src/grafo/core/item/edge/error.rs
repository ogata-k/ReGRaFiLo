use crate::grafo::core::item::{HasItemKind, ItemErrorBase};
use crate::grafo::GrafoError;
use crate::util::item_kind::ItemKind;
use std::error::Error;
use std::fmt::{Display, Formatter};

// TODO doc commentや実装などのリファクタリングとbuilderメソッドをエンドユーザーから隠す

#[derive(Debug, Clone)]
pub enum EdgeItemError {
    // TODO
}

impl HasItemKind for EdgeItemError {
    fn kind() -> ItemKind {
        ItemKind::Edge
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
