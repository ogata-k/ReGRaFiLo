//! style for NodeItem

use crate::grafo::core::graph_item::GraphItemStyleBase;
use crate::util::writer::DisplayAsJson;

/// style's structure for NodeItem
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct NodeItemStyle {
    // eg. shape: Option<Either<Diamond, Custom>>
}

impl DisplayAsJson for NodeItemStyle {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl Default for NodeItemStyle {
    fn default() -> Self {
        Self {}
    }
}

impl GraphItemStyleBase for NodeItemStyle {}

impl NodeItemStyle {}
