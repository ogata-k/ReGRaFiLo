use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::graph_item::group::GroupItem;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::item_base::ItemErrorBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GroupItemError {
    // TODO
}

impl HasGraphItemKind for GroupItemError {
    fn kind() -> GraphItemKind {
        GroupItem::kind()
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
impl From<NameIdError<GraphItemKind>> for GroupItemError {
    fn from(error: NameIdError<GraphItemKind>) -> Self {
        unimplemented!()
    }
}
impl GraphBuilderErrorBase for GroupItemError {}
