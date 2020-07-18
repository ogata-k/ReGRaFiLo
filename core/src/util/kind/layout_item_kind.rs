//! kind for layout

use crate::util::kind::attribute_kind::AttributeKind;
use crate::util::kind::graph_item_kind::GraphItemKind;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum LayoutItemKind {
    IsolateAttribute(AttributeKind),
    WithItemAttribute(GraphItemKind, AttributeKind),
}

impl LayoutItemKind {
    pub fn new_with_item(item_kind: GraphItemKind, attribute_kind: AttributeKind) -> Self {
        Self::WithItemAttribute(item_kind, attribute_kind)
    }

    pub fn new(attribute_kind: AttributeKind) -> Self {
        Self::IsolateAttribute(attribute_kind)
    }
}

impl Display for LayoutItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
