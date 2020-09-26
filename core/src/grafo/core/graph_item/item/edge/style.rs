//! style for NodeItem

use crate::grafo::core::graph_item::GraphItemStyleBase;
use crate::util::writer::DisplayAsJson;

/// style's structure for EdgeItem
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EdgeItemStyle {
    // eg. shape: Option<Either<Diamond, Custom>>
}

impl DisplayAsJson for EdgeItemStyle {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl Default for EdgeItemStyle {
    fn default() -> Self {
        Self {}
    }
}

impl GraphItemStyleBase for EdgeItemStyle {}

impl EdgeItemStyle {}
