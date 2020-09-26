//! style for GroupItem

use crate::grafo::graph_item::GraphItemStyleBase;
use crate::util::writer::DisplayAsJson;

/// style's structure for GroupItem
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct GroupItemStyle {
    // eg. shape: Option<Either<Diamond, Custom>>
}

impl DisplayAsJson for GroupItemStyle {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl Default for GroupItemStyle {
    fn default() -> Self {
        Self {}
    }
}

impl GraphItemStyleBase for GroupItemStyle {}

impl GroupItemStyle {}
