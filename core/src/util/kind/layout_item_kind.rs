use crate::util::kind::GraphItemKind;
use std::fmt::{Display, Formatter};

/// kind of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKind {
    Form,
    Group,
}

fn item_kind_to_str(attribute_kind: &AttributeKind) -> &str {
    match attribute_kind {
        AttributeKind::Form => "Form",
        AttributeKind::Group => "Group",
    }
}

impl Display for AttributeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

pub trait HasAttributeKind {
    fn kind() -> AttributeKind;
    fn get_kind(&self) -> AttributeKind {
        Self::kind()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum WithItemLayoutKind {
    // TODO
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum LayoutItemKind {
    IsolateAttribute(AttributeKind),
    WithItemAttribute(GraphItemKind, WithItemLayoutKind),
}

impl LayoutItemKind {
    pub fn new_attribute(attribute_kind: AttributeKind) -> Self {
        Self::IsolateAttribute(attribute_kind)
    }

    pub fn new_layout(item_kind: GraphItemKind, layout_kind: WithItemLayoutKind) -> Self {
        Self::WithItemAttribute(item_kind, layout_kind)
    }

    pub fn is_attribute(&self) -> bool {
        match self {
            LayoutItemKind::IsolateAttribute(_) => true,
            LayoutItemKind::WithItemAttribute(_, _) => false,
        }
    }

    pub fn need_graph_item(&self) -> bool {
        match self {
            LayoutItemKind::IsolateAttribute(_) => false,
            LayoutItemKind::WithItemAttribute(_, _) => true,
        }
    }
}

impl Display for LayoutItemKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
